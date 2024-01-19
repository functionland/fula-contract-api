use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ConvertToValidatorInput {
    pub seed: String,
    pub aura_account: String,
    pub grandpa_account: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConvertToValidatorOutput {
    pub aura_account: String,
    pub grandpa_account: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefundInput {
    pub account: String,
    pub balance: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefundOutput {
    pub account: String,
}
