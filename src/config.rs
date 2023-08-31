use std::env;

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
    pub year_to_hours: u64,
    pub hour_to_miliseconds: u64,
}

pub fn init() -> Config {
    let panic_message: String = "enviroment variable is not set".to_string();

    Config {
        labor_token_class_id: match env::var("LABOR_TOKEN_CLASS_ID") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("LABOR_TOKEN_CLASS_ID {}", panic_message),
        },
        labor_token_asset_id: match env::var("LABOR_TOKEN_ASSET_ID") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("LABOR_TOKEN_ASSET_ID {}", panic_message),
        },
        challenge_token_class_id: match env::var("CHALLENGE_TOKEN_CLASS_ID") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("CHALLENGE_TOKEN_CLASS_ID {}", panic_message),
        },
        challenge_token_asset_id: match env::var("CHALLENGE_TOKEN_ASSET_ID") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("CHALLENGE_TOKEN_ASSET_ID {}", panic_message),
        },
        yearly_tokens: match env::var("YEARLY_TOKENS") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("YEARLY_TOKENS {}", panic_message),
        },
        cycles_advance: match env::var("NUMBER_CYCLES_TO_ADVANCE") {
            Ok(var) => var.parse::<u16>().unwrap(),
            Err(_) => panic!("NUMBER_CYCLES_TO_ADVANCE {}", panic_message),
        },
        cycles_reset: match env::var("NUMBER_CYCLES_TO_RESET") {
            Ok(var) => var.parse::<u16>().unwrap(),
            Err(_) => panic!("NUMBER_CYCLES_TO_RESET {}", panic_message),
        },
        year_to_hours: match env::var("YEAR_TO_HOURS") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("YEAR_TO_HOURS {}", panic_message),
        },
        hour_to_miliseconds: match env::var("HOUR_TO_MILISECONDS") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("HOUR_TO_MILISECONDS {}", panic_message),
        },
    }
}
