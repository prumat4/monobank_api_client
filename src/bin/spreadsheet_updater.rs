use std::env;
use dotenv::dotenv;

use monobank_api::api_client::{Client, MonobankClientInfo, to_abbreviation};

fn main() {
    dotenv().ok();
    let key: String = env::var("API_KEY").expect("API_KEY must be set");

    let client = Client::new(&key);

    let currencies = match client.request_currencies() {
        Ok(currencies) => {
            dbg!("Currencies: {}", currencies);
        }
        Err(e) => {
            eprintln!("Error in get_currencies: {}", e);
        }
    };
}