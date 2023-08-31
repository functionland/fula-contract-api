use contract_api_types::types::Config;
use std::env;

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
        total_cyles: match env::var("TOTAL_CYCLES") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("TOTAL_CYCLES {}", panic_message),
        },
        time_between_cycles_miliseconds: match env::var("TIME_BETWEEN_CYLCES_MILISECONDS") {
            Ok(var) => var.parse::<u64>().unwrap(),
            Err(_) => panic!("TIME_BETWEEN_CYLCES_MILISECONDS {}", panic_message),
        },
    }
}
