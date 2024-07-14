use std::env;
use dotenv::dotenv;

use monobank_api::api_client::{Client, MonobankClientInfo, to_abbreviation};

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

    let user_info: MonobankClientInfo = client.request_user_info().unwrap();
    let accounts = user_info.accounts();
    for account in accounts {
        let id = account.id();
        let balance = account.balance();
        let currency = to_abbreviation(*account.currency_code());

        println!("id: {}, balance: {:>10}, currency: {}", id, balance, currency);
    }
    
    let from = "1718623708";
    let to = "1720881322";
    let account = "account";
    
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
