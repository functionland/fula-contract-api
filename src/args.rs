use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 's', long, value_parser, default_value = "ws://127.0.0.1:9947")]
    pub node_server: Url,
    #[arg(short, long, value_parser, default_value = "http://127.0.0.1:4003")]
    pub listen: Url,
    #[arg(short, long, value_parser)]
    pub validator_seed: String,
    #[arg(short, long = "db-uri", value_parser)]
    pub db: Option<String>,
}
