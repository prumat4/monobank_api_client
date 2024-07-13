use std::env;
use dotenv::dotenv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key: String = env::var("API_KEY").expect("API_KEY must be set");

    // match monobank_api::request_currencies() {
    //     Ok(pretty_json) => println!("Currencies: {}", pretty_json),
    //     Err(e) => eprint!("Error in get_currencies: {}", e)
    // }

    // match monobank_api::request_user_info(&api_key) {
    //     Ok(pretty_json) => println!("User: {}", pretty_json),
    //     Err(e) => eprint!("Error in request_user_info: {}", e)
    // }

    let from = "1718623708";
    let to = "1720881322";

    match monobank_api::request_payments(&api_key, from, to) {
        Ok(pretty_json) => println!("User: {}", pretty_json),
        Err(e) => eprint!("Error in request_payments: {}", e)
    }
    

    Ok(())
}
