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
		AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
		SignedPayload, Signer, SigningTypes, SubmitTransaction,
	},
};
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
	offchain::{
		http,
		storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
		Duration,
	},
	transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
	RuntimeDebug,
};
use sp_std::vec::Vec;

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
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_block_number: T::BlockNumber) {
			log::info!("evm-bridge");

			let res = Self::get_evm_latest_block();
			if let Err(e) = res {
				log::error!("Error: {}", e);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	/// Events for the pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {}
}

impl<T: Config> Pallet<T> {
	fn get_evm_latest_block() -> Result<(), &'static str> {
		let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
		let body = serde_json::json!({
			"jsonrpc": "2.0",
			"method": "eth_blockNumber",
			"params": [],
			"id": 1,
		});
		let body = serde_json::to_vec(&body).map_err(|_| "Unknown")?;

		let url = "https://data-seed-prebsc-1-s1.binance.org:8545/";
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

		log::warn!("Got price: {} cents", &result);

		let string_result = result.to_string();
		let without_prefix = string_result.trim_start_matches("0x");
		let z = u64::from_str_radix(without_prefix, 16);

		let latest_block_key = StorageValueRef::persistent(b"evm_bridge::latest_block");
		latest_block_key.mutate(|last_send: Result<Option<u64>, StorageRetrievalError>| {
			match last_send {
				// If we already have a value in storage and the block number is recent enough
				// we avoid sending another transaction at this time.
				// Ok(Some(block)) if block >= z { Err(block) } else { Ok(z) },
				// In every other case we attempt to acquire the lock and send a transaction.
				_ => z,
			}
		});

		Ok(())
	}
}