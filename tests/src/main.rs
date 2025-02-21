use polkadot_sdk::sp_runtime;

// sp_runtime::AccountId32;
use sp_runtime::AccountId32;
use std::str::FromStr;

fn main() {
    let account =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    println!("Hello, world! {}", account);
}

#[test]
fn test_ok() {
    use codec::{Decode, Encode};
    let account =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    println!("code is {:?}", &account);
    let code = account.encode();
    println!("code is {:?}", code);

    assert!(true);
}
