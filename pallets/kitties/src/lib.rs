#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::pallet_macros::import_section;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

mod config;
mod errors;
mod events;
mod extrinsics;
mod hooks;
mod impls;

pub use pallet::*;

/// Import all sections from different files.
#[import_section(extrinsics::dispatches)]
#[import_section(errors::errors)]
#[import_section(events::events)]
#[import_section(config::config)]
#[import_section(hooks::hooks)]
#[import_section(impls::impls)]
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::Randomness;
    use frame_system::pallet_prelude::*;

    #[derive(Encode, Decode, Clone, Default, TypeInfo, MaxEncodedLen)]
    pub struct Kitty(pub [u8; 16]);

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub type NextKittyId<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, u32, Kitty>;

    // #[pallet::storage]
    // pub type KittyOwner<T: Config> = StorageMap<_, Twox64Concat, u32, T::AccountId>;
}
