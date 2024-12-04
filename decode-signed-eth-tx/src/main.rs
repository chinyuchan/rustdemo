#![allow(dead_code, unused)]
use ethers::prelude::*;
use ethers::types::Transaction;
use ethers::utils::hex;
use ethers::utils::rlp;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rlp_tx = hex::decode("02f872018307910d808507204d2cb1827d0094388c818ca8b9251b393131c08a736a67ccb19297880320d04823e2701c80c001a0cf024f4815304df2867a1a74e9d2707b6abda0337d2d54a4438d453f4160f190a07ac0e6b3bc9395b5b9c8b9e6d77204a236577a5b18467b9175c01de4faa208d9").unwrap();
    let tx: Transaction = rlp::decode(&rlp_tx).unwrap();
    assert_eq!(tx.rlp(), rlp_tx);
    assert_eq!(
        tx.hash,
        "0x86718885c4b4218c6af87d3d0b0d83e3cc465df2a05c048aa4db9f1a6f9de91f"
            .parse()
            .unwrap()
    );
    assert_eq!(tx.transaction_type, Some(2.into()));
    let expected = Address::from_str("0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5").unwrap();
    let sender = tx.recover_from()?;
    assert_eq!(sender, expected);
    println!("from: {:?}", sender);
    println!("to: {:?}", tx.to.unwrap());
    Ok(())
}
