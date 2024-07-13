use reqwest::{self, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};
use serde::Deserializer;
use serde_json;
use thiserror::Error;

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Currencies {
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
    currencyCode: i32,
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
    currencyCode: i32,
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

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct PaymentsInfo {
    id: String,
    time: i64,
    description: String,
    mcc: i32,
    originalMcc: i32,
    hold: bool,
    #[serde(deserialize_with = "from_cents")]
    amount: f32,
    #[serde(deserialize_with = "from_cents")]
    operationAmount: f32,
    currencyCode: i32,
    #[serde(deserialize_with = "from_cents")]
    commissionRate: f32,
    #[serde(deserialize_with = "from_cents")]
    cashbackAmount: f32,
    #[serde(deserialize_with = "from_cents")]
    balance: f32, 
    comment: Option<String>,
    receiptId: Option<String>,
    invoiceId: Option<String>,
    counterEdrpou: Option<String>,
    counterIban: Option<String>,
    counterName: Option<String>,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Invalid API key")]
    InvalidApiKey,
    #[error("Unknown error")]
    Unknown,
}

pub fn from_cents<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let cents = i32::deserialize(deserializer)?;
    Ok(cents as f32 / 100.0)
}

// pub struct MonobankAPIClient {
// }

fn build_headers(api_key: &str) -> Result<HeaderMap, ApiError> {
    let mut headers = HeaderMap::new();
    headers.insert("X-Token", HeaderValue::from_str(api_key).map_err(|_| ApiError::InvalidApiKey)?);
    Ok(headers)
}

pub fn request_currencies() -> Result<String, ApiError> {
    let url = "https://api.monobank.ua/bank/currency";
    let res = reqwest::blocking::get(url)?;
    let api_response: Vec<Currencies> = res.json()?;
    let pretty_json = serde_json::to_string_pretty(&api_response)?;

    Ok(pretty_json)
}

pub fn request_user_info(api_key: &str) -> Result<String, ApiError> {
    let url = "https://api.monobank.ua/personal/client-info";
    let headers = build_headers(api_key)?;

    let client = reqwest::blocking::Client::new();
    let res = client.get(url).headers(headers).send()?;
    let api_response: MonobankClientInfo = res.json()?;
    let pretty_json = serde_json::to_string_pretty(&api_response)?;

    Ok(pretty_json)
}

pub fn request_payments(api_key: &str, from: &str, to: &str) -> Result<String, ApiError> {
    let url = format!("https://api.monobank.ua/personal/statement/{account}/{}/{}", from, to);
    let headers = build_headers(api_key)?;

    let client = reqwest::blocking::Client::new();
    let res = client.get(&url).headers(headers).send()?;
    let api_response: Vec<PaymentsInfo> = res.json()?;
    let pretty_json = serde_json::to_string_pretty(&api_response)?;

    Ok(pretty_json)
}
