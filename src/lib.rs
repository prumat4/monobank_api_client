use reqwest::{self, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};
use serde::Deserializer;
use serde_json;

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct ApiCurrencyJson {
    currencyCodeA: Option<i32>,
    currencyCodeB: Option<i32>,
    date: Option<i64>,
    rateSell: Option<f64>,
    rateBuy: Option<f64>,
    rateCross: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Account {
    id: String,
    sendId: String,
    #[serde(deserialize_with = "from_cents")]
    balance: f32,
    #[serde(deserialize_with = "from_cents")]
    creditLimit: f32,
    r#type: String,
    currencyCode: i16,
    cashbackType: Option<String>,
    maskedPan: Vec<String>,
    iban: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Jar {
    id: String,
    sendId: String,
    title: String,
    description: String,
    currencyCode: i16,
    #[serde(deserialize_with = "from_cents")]
    balance: f32,
    goal: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct MonobankClientInfo {
    clientId: String,
    name: String,
    webHookUrl: String,
    permissions: String,
    accounts: Vec<Account>,
    jars: Vec<Jar>,
}

pub fn from_cents<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let cents = i32::deserialize(deserializer)?;
    Ok(cents as f32 / 100.0)
}

pub fn get_currencies() -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://api.monobank.ua/bank/currency");
    let res = reqwest::blocking::get(&url)?;
    let api_response: Vec<ApiCurrencyJson> = res.json()?;
    let pretty_json = serde_json::to_string_pretty(&api_response)?;

    Ok(pretty_json)
}

pub fn req(api_key: &String) -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.monobank.ua/personal/client-info";
    
    let mut headers = HeaderMap::new();
    headers.insert("X-Token", HeaderValue::from_str(api_key)?);
    
    let client = reqwest::blocking::Client::new();
    let res = client.get(url).headers(headers).send()?;
    
    let api_response: MonobankClientInfo = res.json()?;
    let pretty_json = serde_json::to_string_pretty(&api_response)?;
    
    Ok(pretty_json)
}