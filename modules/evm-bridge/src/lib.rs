#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

use alloc::string::{String, ToString};
// use frame_support::storage::bounded_vec::BoundedVec;
// use rpc::Params;
// use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use frame_support::traits::Get;
// use jsonrpc_core as rpc;
use codec::{Decode, Encode, MaxEncodedLen};
use frame_system::{
	offchain::{
		AppCrypto, CreateSignedTransaction, Signer,
		SendSignedTransaction,
		// SignedPayload, Signer, SigningTypes, SubmitTransaction,
	},
};
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
	offchain::{
		http,
		storage::{StorageRetrievalError, StorageValueRef},
		Duration,
	},
	SaturatedConversion, RuntimeDebug,
};
use sp_std::vec::{Vec};
use ethereum_types::{U256, H256};
use sp_std::str::FromStr;
use frame_support::{
	traits::{Currency, Contains}, transactional
};
use node_primitives::{evm::EvmAddress};
use module_support::{AddressMapping};
use scale_info::TypeInfo;
use serde_json::Value;

/// Type alias for currency balance.
pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Defines application identifier for crypto keys of this module.
///
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"evmb");

/// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrappers.
/// We can use from supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
/// the types with this pallet-specific identifier.
pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify,
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// This pallet's configuration trait
	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The overarching dispatch call type.
		type Call: From<Call<Self>>;

		/// The Currency for managing Evm account assets.
		type Currency: Currency<Self::AccountId>;

		/// Mapping from address to account id.
		type AddressMapping: AddressMapping<Self::AccountId>;

		type BridgeContains: Contains<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_block_number: T::BlockNumber) {
			log::info!("evm-bridge");

			let latest_block = Self::get_evm_latest_block().unwrap();
			let latest_processed_block_key = StorageValueRef::persistent(b"evm_bridge::latest_processed_block");
			if let Ok(Some(latest_processed_block)) = latest_processed_block_key.get::<U256>() {
				let res = Self::get_evm_event(&latest_processed_block, &latest_block);
				if let Err(e) = res {
					log::error!("Error: {}", e);
				}
			} else {
				let latest_processed_block = latest_block - U256::from(1u32);
				let res = Self::get_evm_event(&latest_processed_block, &latest_block);
				if let Err(e) = res {
					log::error!("Error: {}", e);
				}
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		#[transactional]
		pub fn claim(
			origin: OriginFor<T>,
			tx_hash: H256,
			log_index: U256,
			eth_address: EvmAddress,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(T::BridgeContains::contains(&who), Error::<T>::OnlyOwner);

			if let Some(mut event_info) = Claims::<T>::get(tx_hash, log_index) {
				event_info.confirmation += 1u32;
				Claims::<T>::insert(
					tx_hash.clone(),
					log_index.clone(),
					event_info.clone(),
				);
			} else {
				let event_info = Erc20BridgeInfo {
					confirmation: 1u32,
					is_executed: false, 
				};
				Claims::<T>::insert(
					tx_hash.clone(),
					log_index.clone(),
					event_info.clone(),
				);
			}

			if let Some(mut event_info) = Claims::<T>::get(tx_hash, log_index) {
				if event_info.is_executed {
					return Ok(());
				}
		
				// TODO: load this threshold from config
				// TODO: check duplicate submit
				if event_info.confirmation >= 1u32 {
					let account_id = T::AddressMapping::get_account_id(&eth_address);
					T::Currency::deposit_creating(&account_id, amount);
					Self::deposit_event(Event::Claimed(account_id, eth_address, amount));
		
					event_info.is_executed = true;
					Claims::<T>::insert(
						tx_hash.clone(),
						log_index.clone(),
						event_info,
					);
				}
			}

			Ok(())
		}
	}

	/// Events for the pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Claimed(T::AccountId, EvmAddress, BalanceOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		OnlyOwner,
	}

	#[pallet::storage]
	#[pallet::getter(fn claims)]
	pub type Claims<T> = StorageDoubleMap<
	  _,
	  Blake2_128Concat,
	  H256,
	  Blake2_128Concat,
	  U256,
	  Erc20BridgeInfo,
	>;
}

#[derive(Encode, Decode, Eq, PartialEq, Clone, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Erc20BridgeInfo {
	pub confirmation: u32,
	pub is_executed: bool,
}

impl<T: Config> Pallet<T> {
	fn get_evm_latest_block() -> Result<U256, &'static str> {
		let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
		let body = serde_json::json!({
			"jsonrpc": "2.0",
			"method": "eth_blockNumber",
			"params": [],
			"id": 1,
		});
		let body = serde_json::to_vec(&body).map_err(|_| "Unknown")?;

		let url = "https://data-seed-prebsc-1-s3.binance.org:8545/";
		let mut request = http::Request::post(url, vec![body]);
		request = request.add_header("content-type", "application/json");

		let pending = request.deadline(deadline).send().map_err(|_| "IoError")?;
		let response = pending.try_wait(deadline).map_err(|_| "DeadlineReached")?.map_err(|_| "DeadlineReached")?;
		if response.code != 200 {
			return Err("Unknown");
		}

		let body = response.body().collect::<Vec<u8>>();
		let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
		let result = json["result"].clone();
		if result.is_null() {
			return Err("IoError");
		}

		let string_result_with_escapes = result.to_string();
		let string_result_without_escapes: &str = &string_result_with_escapes[1..string_result_with_escapes.len() - 1];
		let latest_block = U256::from_str(string_result_without_escapes).expect("internal U256 is valid; qed");

		let latest_block_key = StorageValueRef::persistent(b"evm_bridge::latest_block");
		latest_block_key.mutate(|last_send: Result<Option<U256>, StorageRetrievalError>| {
			match last_send {
				Ok(Some(block)) if block >= latest_block => Err(block),
				_ => Ok(latest_block),
			}
		}).map_err(|_| "StorageRetrievalError")?;

		Ok(latest_block)
	}

	fn get_evm_event(latest_processed_block: &U256, latest_block: &U256) -> Result<(), String> {
		let from_block: U256 = latest_processed_block + U256::from(1u32);
		let to_block: U256 = if latest_block - latest_processed_block <= U256::from(5u32) { *latest_block } else { latest_processed_block + U256::from(5u32) };
		let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(3_000));

		log::info!("fromBlock 0x{:x}", from_block);
		log::info!("toBlock 0x{:x}", to_block);

		let mut map = serde_json::Map::new();
		map.insert("fromBlock".to_string(), Value::String(format!("0x{:x}", from_block)));
		map.insert("toBlock".to_string(), Value::String(format!("0x{:x}", to_block)));
		map.insert(
			"address".to_string(),
			Value::String("0xada53e625c5cb7de867b197aeab7c39be95b1685".to_string()),
		);
		map.insert(
			"topics".to_string(),
			Value::Array(
				vec![
					Value::String("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string()),
					Value::Null,
					Value::String("0x000000000000000000000000b1b11e04348f4271b163db51138704f3dec0c128".to_string()),
				]
			),
		);

		let param: serde_json::Value = Value::Object(map);
		let body = serde_json::json!({
			"jsonrpc": "2.0",
			"method": "eth_getLogs",
			"params": vec![param],
			"id": 1,
		});
		let body = serde_json::to_vec(&body).map_err(|_| "Unknown")?;

		let url = "https://data-seed-prebsc-1-s3.binance.org:8545/";
		let mut request = http::Request::post(url, vec![body]);
		request = request.add_header("content-type", "application/json");

		let pending = request.deadline(deadline).send().map_err(|_| "IoError")?;
		let response = pending.try_wait(deadline).map_err(|_| "DeadlineReached")?.map_err(|_| "DeadlineReached")?;
		if response.code != 200 {
			return Err("response.code != 200".to_string());
		}

		let body = response.body().collect::<Vec<u8>>();
		let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
		let result = json["result"].clone();
		if result.is_null() {
			return Err("result.is_null".to_string());
		}

		let events: Vec<JsonEventEntity> = serde_json::from_value(result).unwrap();

		let signer = Signer::<T, T::AuthorityId>::all_accounts();
		if !signer.can_sign() {
			return Err(
				"No local accounts available. Consider adding one via `author_insertKey` RPC.".to_string(),
			)?;
		}

		for event in events {
			let erc20_event = ERC20TransferEvent::from(event);
			let amount: BalanceOf<T> = erc20_event.amount.clone().as_u128().saturated_into();

			let results = signer.send_signed_transaction(|_account| {
				Call::claim {
					tx_hash: erc20_event.transaction_hash,
					log_index: erc20_event.log_index,
					eth_address: erc20_event.from.clone(),
					amount,
				}
			});

			for (_, res) in &results {
				match res {
					Ok(()) => log::info!("Send transaction successfully"),
					Err(e) => {
						let message = format!("Failed to submit transaction: {:?}", e);
						return Err(message.clone());
					},
				}
			}
		}

		let latest_processed_block_key = StorageValueRef::persistent(b"evm_bridge::latest_processed_block");
		latest_processed_block_key.mutate(|last_send: Result<Option<U256>, StorageRetrievalError>| {
			match last_send {
				Ok(Some(block)) if block >= to_block => Err(block),
				_ => Ok(to_block),
			}
		}).map_err(|_| "StorageRetrievalError")?;

		Ok(())
	}
}

#[derive(Debug, Clone, serde::Deserialize, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct JsonEventEntity {
	pub address: String,
	pub topics: Vec<String>,
	pub data: String,
	pub block_number: String,
	pub transaction_hash: String,
	pub transaction_index: String,
	pub block_hash: String,
	pub log_index: String,
	pub removed: bool,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, TypeInfo)]
pub struct ERC20TransferEvent {
	pub address: EvmAddress,
	pub from: EvmAddress,
	pub to: EvmAddress,
	pub amount: U256,
	pub block_number: U256,
	pub transaction_hash: H256,
	pub transaction_index: U256,
	pub block_hash: H256,
	pub log_index: U256,
	pub removed: bool,
}

impl From<JsonEventEntity> for ERC20TransferEvent {
	fn from(raw: JsonEventEntity) -> Self {
		Self {
			address: hex_to_address(raw.address),
			from: topic_to_address(raw.topics[1].clone()),
			to: topic_to_address(raw.topics[2].clone()),
			amount: hex_to_u256(raw.data),
			block_number: hex_to_u256(raw.block_number),
			transaction_hash: hex_to_h256(raw.transaction_hash),
			transaction_index: hex_to_u256(raw.transaction_index),
			block_hash: hex_to_h256(raw.block_hash),
			log_index: hex_to_u256(raw.log_index),
			removed: raw.removed,
		}
	}
}

fn hex_to_address(v: String) -> EvmAddress {
    let s = &mut v[2..].as_bytes().to_vec();
    if s.len() % 2 != 0 {
        s.push(b'0');
    }
    let b = hex::decode(&s).unwrap();
    EvmAddress::from_slice(&b)
}

fn topic_to_address(v: String) -> EvmAddress {
    let s = &mut v[26..].as_bytes().to_vec();
    if s.len() % 2 != 0 {
        s.push(b'0');
    }
    let b = hex::decode(&s).unwrap();
    EvmAddress::from_slice(&b)
}

fn hex_to_h256(v: String) -> H256 {
	let s = &mut v[2..].as_bytes().to_vec();
	if s.len() % 2 != 0 {
		s.push(b'0');
	}
	let b = hex::decode(&s).unwrap();
	H256::from_slice(&b)
}

fn hex_to_u256(v: String) -> U256 {
	let s = &mut v[2..].as_bytes().to_vec();
	if s.len() % 2 != 0 {
		s.insert(0, b'0'); // big endian .. add to the first.
	}
	let b = hex::decode(&s).unwrap();
	U256::from_big_endian(b.as_slice())
}