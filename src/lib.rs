use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ApiCurrencyJson {
    currencyCodeA: Option<i32>,
    currencyCodeB: Option<i32>,
    date: Option<i64>,
    rateSell: Option<f32>,
    rateBuy: Option<f32>,
    rateCross: Option<f32>,
}

pub fn get_currencies() -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://api.monobank.ua/bank/currency");
    let res = reqwest::blocking::get(&url)?;
    let api_response: Vec<ApiCurrencyJson> = res.json()?;
    let pretty_json = serde_json::to_string_pretty(&api_response)?;

    Ok(pretty_json)
}
