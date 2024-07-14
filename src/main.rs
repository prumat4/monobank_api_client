use std::env;
use dotenv::dotenv;

mod api_client;

use api_client::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let client_info = match client.request_user_info() {
        Ok(client_info) => {
            dbg!("User: {}", &client_info);
        }
        Err(e) => {
            eprintln!("Error in request_user_info: {}", e);
        }
    };

    let from = "1718623708";
    let to = "1720881322";
    let account = "Nc8zgrp8lmgl17YnWmXKCA";
    
    let payments = match client.request_payments(account, from, to) {
        Ok(payments) => {
            dbg!("Payments: {}", &payments);
        } 
        Err(e) => {
            eprintln!("Error in request_payments: {}", e);
        }
    };

    Ok(())
}
