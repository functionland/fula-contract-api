use crate::state::AppState;
use actix_web::HttpResponse;
use actix_web::{error, web};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sp_core::U256;
use std::ops::Div;
use subxt::rpc::types::Health;
use url::Url;

pub async fn health_check(data: web::Data<AppState>) -> error::Result<HttpResponse> {
    let api = &data.api;
    let health: Health = api.rpc().system_health().await.map_err(map_subxt_err)?;
    Ok(HttpResponse::Ok().json(health))
}

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

pub fn map_subxt_err(e: subxt::Error) -> actix_web::Error {
    // TODO: json_err should be a json Value to improve UX
    let json_err = json!(e.to_string());
    let req_error = RequestError {
        message: json_err,
        description: "Subxt error".into(),
    };
    let req_error = serde_json::to_string_pretty(&req_error).unwrap();
    error::ErrorBadRequest(req_error)
}
