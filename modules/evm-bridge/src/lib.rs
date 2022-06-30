#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	ensure,
	pallet_prelude::*,
	traits::{Currency, IsType, OnKilledAccount},
	transactional,
};
use frame_system::{ensure_signed, pallet_prelude::*};

// pub use module::*;

#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
	}

	// #[pallet::event]
	// #[pallet::generate_deposit(pub(crate) fn deposit_event)]
	// pub enum Event<T: Config> {
	// }

	/// Error for evm accounts module.
	#[pallet::error]
	pub enum Error<T> {
	}

	/// The Substrate Account for EvmAddresses
	///
	/// Accounts: map EvmAddress => Option<AccountId>
	// #[pallet::storage]
	// #[pallet::getter(fn accounts)]
	// pub type Accounts<T: Config> = StorageMap<_, Twox64Concat, EvmAddress, T::AccountId, OptionQuery>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
	}
}