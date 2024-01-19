use crate::{
    config,
    util::{fula_sugarfunge_req, RequestError},
    Args,
};
use actix_web::{error, web, HttpResponse};
use clap::Parser;
use contract_api_types::calls::*;
use dotenv::dotenv;
use sugarfunge_api_types::account::{SetBalanceInput, SetBalanceOutput};
use sugarfunge_api_types::validator::{
    AddValidatorInput, AddValidatorOutput, SetKeysInput, SetKeysOutput,
};

pub async fn setup() -> error::Result<HttpResponse> {
    dotenv().ok();
    let env = config::config_init();

    Ok(HttpResponse::Ok().json(env))
}

pub async fn refund(req: web::Json<RefundInput>) -> error::Result<HttpResponse> {
    let args = Args::parse();
    dotenv().ok();

    let _ = set_balance(SetBalanceInput {
        seed: args.validator_seed.into(),
        to: req.account.clone().into(),
        amount: req.balance.into(),
    })
    .await;
    Ok(HttpResponse::Ok().json(RefundOutput {
        account: req.account.clone(),
    }))
}

pub async fn convert_to_validator(
    req: web::Json<ConvertToValidatorInput>,
) -> error::Result<HttpResponse> {
    dotenv().ok();
    let env = config::parameters_init();
    let args = Args::parse();

    let _ = set_balance(SetBalanceInput {
        seed: args.validator_seed.clone().into(),
        to: req.aura_account.clone().into(),
        amount: env.initial_balance.into(),
    })
    .await;

    let _ = set_keys(SetKeysInput {
        seed: req.seed.clone().into(),
        aura: req.aura_account.clone(),
        grandpa: req.grandpa_account.clone(),
    })
    .await;

    let _ = add_validator(AddValidatorInput {
        seed: args.validator_seed.clone().into(),
        validator_id: req.aura_account.clone().into(),
    })
    .await;

    Ok(HttpResponse::Ok().json(ConvertToValidatorOutput {
        aura_account: req.aura_account.clone(),
        grandpa_account: req.grandpa_account.clone(),
    }))
}

async fn set_keys(input: SetKeysInput) -> Result<SetKeysOutput, RequestError> {
    let result: Result<SetKeysOutput, _> = fula_sugarfunge_req("validator/set_keys", input).await;
    return result;
}

async fn add_validator(input: AddValidatorInput) -> Result<AddValidatorOutput, RequestError> {
    let result: Result<AddValidatorOutput, _> =
        fula_sugarfunge_req("validator/add_validator", input).await;
    return result;
}

async fn set_balance(input: SetBalanceInput) -> Result<SetBalanceOutput, RequestError> {
    let result: Result<SetBalanceOutput, _> =
        fula_sugarfunge_req("account/set_balance", input).await;
    return result;
}
