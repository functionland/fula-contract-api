use crate::config;
use crate::types::*;
use crate::util::*;
use actix_web::{error, web, HttpResponse};
use contract_integration::calls::*;
use dotenv::dotenv;
use serde_json::json;
use sp_core::U256;

pub async fn goerli_contract_mint_to(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = goerli_mint_to(req.account_address.as_str(), U256::from(req.amount)).await;
    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::MintTo"),
            description: format!(""),
        })),
    }
}

pub async fn goerli_contract_burn_from(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = goerli_burn_from(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::BurnFrom"),
            description: format!(""),
        })),
    }
}

pub async fn goerli_contract_transfer(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = goerli_transfer(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::Transfer"),
            description: format!(""),
        })),
    }
}

pub async fn goerli_contract_total_supply() -> error::Result<HttpResponse> {
    let result = goerli_total_supply().await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(ContractTotalSupplyOutput {
            total_supply: remove_decimals_from_u256(event.total_supply, 18),
        })),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::TotalSupply"),
            description: format!(""),
        })),
    }
}

pub async fn goerli_contract_allowance(
    req: web::Json<ContractAllowanceInput>,
) -> error::Result<HttpResponse> {
    let result = goerli_allowance(req.owner_address.as_str(), req.spender_address.as_str()).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(ContractAllowanceOutput {
            allowance: remove_decimals_from_u256(event.allowance, 18),
        })),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::Allowance"),
            description: format!(""),
        })),
    }
}

pub async fn goerli_contract_increase_allowance(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result =
        goerli_increase_allowance(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::IncreaseAllowance"),
            description: format!(""),
        })),
    }
}

pub async fn goerli_contract_decrease_allowance(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result =
        goerli_decrease_allowance(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::DecreaseAllowance"),
            description: format!(""),
        })),
    }
}

pub async fn mumbai_contract_mint_to(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = mumbai_mint_to(req.account_address.as_str(), U256::from(req.amount)).await;
    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::MintTo"),
            description: format!(""),
        })),
    }
}

pub async fn mumbai_contract_burn_from(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = mumbai_burn_from(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::BurnFrom"),
            description: format!(""),
        })),
    }
}

pub async fn mumbai_contract_transfer(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result = mumbai_transfer(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::Transfer"),
            description: format!(""),
        })),
    }
}

pub async fn mumbai_contract_total_supply() -> error::Result<HttpResponse> {
    let result = mumbai_total_supply().await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(ContractTotalSupplyOutput {
            total_supply: remove_decimals_from_u256(event.total_supply, 18),
        })),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::TotalSupply"),
            description: format!(""),
        })),
    }
}

pub async fn mumbai_contract_allowance(
    req: web::Json<ContractAllowanceInput>,
) -> error::Result<HttpResponse> {
    let result = mumbai_allowance(req.owner_address.as_str(), req.spender_address.as_str()).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(ContractAllowanceOutput {
            allowance: remove_decimals_from_u256(event.allowance, 18),
        })),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::Allowance"),
            description: format!(""),
        })),
    }
}

pub async fn mumbai_contract_increase_allowance(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result =
        mumbai_increase_allowance(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::IncreaseAllowance"),
            description: format!(""),
        })),
    }
}

pub async fn mumbai_contract_decrease_allowance(
    req: web::Json<ContractTransactionInput>,
) -> error::Result<HttpResponse> {
    let result =
        mumbai_decrease_allowance(req.account_address.as_str(), U256::from(req.amount)).await;

    match result {
        Ok(event) => Ok(HttpResponse::Ok().json(event)),
        Err(_) => Ok(HttpResponse::BadRequest().json(RequestError {
            message: json!("Failed to execute the contract_event::DecreaseAllowance"),
            description: format!(""),
        })),
    }
}

pub async fn setup() -> error::Result<HttpResponse> {
    dotenv().ok();
    let env = config::init();

    Ok(HttpResponse::Ok().json(env))
}

pub async fn refund() -> error::Result<HttpResponse> {
    dotenv().ok();
    let env = config::refund();

    Ok(HttpResponse::Ok().json(env))
}
