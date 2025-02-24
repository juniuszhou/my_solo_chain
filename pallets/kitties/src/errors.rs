use polkadot_sdk::frame_support::pallet_macros::pallet_section;

/// A [`pallet_section`] that defines the errors for a pallet.
/// This can later be imported into the pallet using [`import_section`].
#[pallet_section]
mod errors {
    #[pallet::error]
    pub enum Error<T> {
        InvalidKittyId,
        NotOwner,
        SameKittyId,
    }
}
