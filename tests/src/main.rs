use codec::{Decode, Encode};
use sp_runtime::AccountId32;
use std::str::FromStr;

fn main() {
    let account =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    println!("Hello, world!");
}

#[test]
fn test_ok() {
    let account =
        AccountId32::from_str("5G9VdMwXvzza9pS8qE8ZHJk3CheHW9uucBn9ngW4C1gmmzpv").unwrap();
    println!("code is {:?}", &account);
    let code = account.encode();
    println!("code is {:?}", code);

    assert!(true);
}
