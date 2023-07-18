use derive_more::Display;
use serde::{Deserialize, Serialize};
use sp_core::U256;
use std::ops::Div;
use url::Url;

#[derive(Serialize, Deserialize, Debug, Display)]
#[display(fmt = "{:?} {:?}", message, description)]
pub struct RequestError {
    pub message: serde_json::Value,
    pub description: String,
}

pub fn url_to_string(url: Url) -> String {
    let mut res = url.to_string();
    match (url.port(), url.port_or_known_default()) {
        (None, Some(port)) => {
            res.insert_str(res.len() - 1, &format!(":{}", port));
            res
        }
        _ => res,
    }
}

pub fn remove_decimals_from_u256(value: U256, decimals: u32) -> u128 {
    return value.div(10_u128.pow(decimals) as u128).as_u128();
}
