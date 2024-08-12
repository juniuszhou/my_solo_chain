use crate::{mock::*, Error, Event, NextKittyId};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        run_to_block(1);
        run_to_block(2);
    });
}

#[test]
fn storage_usage() {
    new_test_ext().execute_with(|| {
        let next_kitty_id = NextKittyId::<Test>::get();
        assert_eq!(next_kitty_id, 0);
    });
}

#[test]
fn call_impl() {
    new_test_ext().execute_with(|| {
        let account = 0_u64;
        // call it with kitty_pallet instance constructed via construct_runtime
        let value = Kitties::random_value(&account);
        assert_eq!(value, [0_u8; 16]);
    })
}
