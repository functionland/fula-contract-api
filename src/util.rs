use crate::config;
use derive_more::Display;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
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

fn endpoint(host: String, cmd: &'static str) -> String {
    format!("{}/{}", host.as_str(), cmd)
}

async fn req<'a, I, O>(host: String, cmd: &'static str, args: I) -> Result<O, RequestError>
where
    I: Serialize,
    O: for<'de> Deserialize<'de>,
{
    let sf_res = reqwest::Client::new()
        .post(endpoint(host, cmd))
        .json(&args)
        .send()
        .await;

    match sf_res {
        Ok(res) => {
            if let Err(err) = res.error_for_status_ref() {
                match res.json::<RequestError>().await {
                    Ok(err) => Err(err),
                    Err(_) => Err(RequestError {
                        message: json!(format!("{:#?}", err)),
                        description: "Reqwest json error.".into(),
                    }),
                }
            } else {
                match res.json().await {
                    Ok(res) => Ok(res),
                    Err(err) => Err(RequestError {
                        message: json!(format!("{:#?}", err)),
                        description: "Reqwest json error.".into(),
                    }),
                }
            }
        }
        Err(err) => Err(RequestError {
            message: json!(format!("{:#?}", err)),
            description: "Reqwest error.".into(),
        }),
    }
}

pub async fn fula_sugarfunge_req<'a, I, O>(cmd: &'static str, args: I) -> Result<O, RequestError>
where
    I: Serialize,
    O: for<'de> Deserialize<'de>,
{
    dotenv().ok();
    let env = config::call_init();
    req(env.fula_sugarfunge_api_host, cmd, args).await
}
