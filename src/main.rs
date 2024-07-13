use std::env;
use dotenv::dotenv;

mod api_client;

use api_client::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let key: String = env::var("API_KEY").expect("API_KEY must be set");

    let client = Client::new(&key);

    match client.request_currencies() {
        Ok(pretty_json) => println!("Currencies: {}", pretty_json),
        Err(e) => eprintln!("Error in get_currencies: {}", e),
    }

    match client.request_user_info() {
        Ok(pretty_json) => println!("User: {}", pretty_json),
        Err(e) => eprintln!("Error in request_user_info: {}", e),
    }

    let from = "1718623708";
    let to = "1720881322";
    let account = "account_id";
    match client.request_payments(account, from, to) {
        Ok(pretty_json) => println!("Payments: {}", pretty_json),
        Err(e) => eprintln!("Error in request_payments: {}", e),
    }

    Ok(())
}
