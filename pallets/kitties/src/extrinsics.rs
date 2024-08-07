use frame_support::pallet_macros::pallet_section;

/// Define all extrinsics for the pallet.
#[pallet_section]
mod dispatches {
    /// Dispatchable functions allow users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        // #[pallet::feeless_if(|_: &OriginFor<T>| -> bool { true })]
        #[pallet::weight(Weight::from_parts(0, 0))]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let _value = Self::random_value(&who);

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(0, 0))]
        pub fn breed(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(0, 0))]
        pub fn transfer(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            Ok(())
        }
    }
}
