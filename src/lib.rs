pub mod redis_manager;
pub mod state;
use ethers::{
    types::{Address, U256},
    utils::hex,
};
use eyre::Result;

pub fn uint256_string_to_u256(s: &str) -> Result<U256, Box<dyn std::error::Error>> {
    let parsed = U256::from_dec_str(s)?;
    Ok(parsed)
}

pub fn uint8_string_to_uint8(s: &str) -> Result<u8, std::num::ParseIntError> {
    let parsed = s.parse::<u8>()?;
    Ok(parsed)
}

pub fn address_string_to_address(s: &str) -> Address {
    if s.starts_with("0x") {
        let s = &s[2..];
        let s_to_address_bytes = hex::decode(s).unwrap();
        let owner_address = Address::from_slice(&s_to_address_bytes);
        println!("owner_address: {:?}", owner_address);
        owner_address
    } else {
        let s_to_address_bytes = hex::decode(s).unwrap();
        let owner_address = Address::from_slice(&s_to_address_bytes);
        println!("owner_address: {:?}", owner_address);
        owner_address
    }
}
