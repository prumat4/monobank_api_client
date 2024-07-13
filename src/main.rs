use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use dotenv::dotenv;

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
struct ApiCurrencyJson {
    currencyCodeA: Option<i32>,
    currencyCodeB: Option<i32>,
    date: Option<i64>,
    rateSell: Option<f32>,
    rateBuy: Option<f32>,
    rateCross: Option<f32>,
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let url = format!("https://api.monobank.ua/bank/currency");

    let res = reqwest::blocking::get(&url)?;
    let api_response: Vec<ApiCurrencyJson> = res.json()?;

    let pretty_json = serde_json::to_string_pretty(&api_response)?;
    println!("API Response: {}", pretty_json);

    Ok(())
}
