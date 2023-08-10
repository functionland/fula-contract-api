use actix_cors::Cors;
use actix_web::{
    http, middleware,
    web::{self},
    App, HttpServer,
};
use args::*;
use clap::Parser;

mod args;
mod contract;
mod types;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let args = Args::parse();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .route(
                "goerli/mint",
                web::post().to(contract::goerli_contract_mint_to),
            )
            .route(
                "goerli/supply",
                web::post().to(contract::goerli_contract_total_supply),
            )
            .route(
                "goerli/allowance",
                web::post().to(contract::goerli_contract_allowance),
            )
            .route(
                "goerli/increase_allowance",
                web::post().to(contract::goerli_contract_increase_allowance),
            )
            .route(
                "goerli/decrease_allowance",
                web::post().to(contract::goerli_contract_decrease_allowance),
            )
            .route(
                "goerli/burn",
                web::post().to(contract::goerli_contract_burn_from),
            )
            .route(
                "goerli/transfer",
                web::post().to(contract::goerli_contract_transfer),
            )
            .route(
                "mumbai/mint",
                web::post().to(contract::mumbai_contract_mint_to),
            )
            .route(
                "mumbai/supply",
                web::post().to(contract::mumbai_contract_total_supply),
            )
            .route(
                "mumbai/allowance",
                web::post().to(contract::mumbai_contract_allowance),
            )
            .route(
                "mumbai/increase_allowance",
                web::post().to(contract::mumbai_contract_increase_allowance),
            )
            .route(
                "mumbai/decrease_allowance",
                web::post().to(contract::mumbai_contract_decrease_allowance),
            )
            .route(
                "mumbai/burn",
                web::post().to(contract::mumbai_contract_burn_from),
            )
            .route(
                "mumbai/transfer",
                web::post().to(contract::mumbai_contract_transfer),
            )
    })
    .bind((args.listen.host_str().unwrap(), args.listen.port().unwrap()))?
    .run()
    .await
}
