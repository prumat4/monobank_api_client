use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use dotenv::dotenv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    // match monobank_api::get_currencies() {
    //     Ok(pretty_json) => println!("Currencies: {}", pretty_json),
    //     Err(e) => eprint!("Error in get_currencies: {}", e)
    // }

    match monobank_api::req(&api_key) {
        Ok(pretty_json) => println!("Currencies: {}", pretty_json),
        Err(e) => eprint!("Error in req: {}", e)
    }

    Ok(())
}
