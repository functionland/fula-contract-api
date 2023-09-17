use derive_more::Display;
use serde::{Deserialize, Serialize};
use sp_core::U256;
use std::ops::Div;

#[derive(Serialize, Deserialize, Debug, Display)]
#[display(fmt = "{:?} {:?}", message, description)]
pub struct RequestError {
    pub message: serde_json::Value,
    pub description: String,
}

pub fn remove_decimals_from_u256(value: U256, decimals: u32) -> u128 {
    return value.div(10_u128.pow(decimals) as u128).as_u128();
}
