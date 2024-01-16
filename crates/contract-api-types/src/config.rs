use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub yearly_tokens: u64,
    pub labor_token_class_id: u64,
    pub labor_token_asset_id: u64,
    pub challenge_token_class_id: u64,
    pub challenge_token_asset_id: u64,
    pub cycles_advance: u16,
    pub cycles_reset: u16,
    pub total_cyles: u64,
    pub time_between_cycles_miliseconds: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Call {
    pub fula_sugarfunge_api_host: String,
    pub amount: u128,
}
