use reqwest::{self, header::{HeaderMap, HeaderValue}};
use serde::{Deserialize, Serialize};
use serde::Deserializer;
use serde_json;
use thiserror::Error;
use log::{info, warn};
use iso_currency::Currency;


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

impl Account {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn balance(&self) -> &f32 {
        &self.balance
    }

    pub fn currency_code(&self) -> &i32 {
        &self.currencyCode
    }
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

impl MonobankClientInfo {
    pub fn accounts(&self) -> &Vec<Account> {
        &self.accounts
    }

    pub fn jars(&self) -> &Vec<Jar> {
        &self.jars
    }
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

pub struct Client {
    key: String,
    http_client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(key: &str) -> Self {
        let http_client = reqwest::blocking::Client::new();
        Self {
            key: key.to_string(),
            http_client,
        }
    }

    fn build_headers(&self) -> Result<HeaderMap, ApiError> {
        let mut headers = HeaderMap::new();
        headers.insert("X-Token", HeaderValue::from_str(&self.key).map_err(|_| ApiError::InvalidApiKey)?);
        Ok(headers)
    }

    pub fn request_currencies(&self) -> Result<Vec<Currencies>, ApiError> {
        let url = "https://api.monobank.ua/bank/currency";
        info!("Requesting currencies from {}", url);

        let res = self.http_client.get(url).send()?;
        if res.status().is_success() {
            let api_response: Vec<Currencies> = res.json()?;
            Ok(api_response)
        } else {
            warn!("Failed to request currencies: {}", res.status());
            Err(ApiError::Unknown)
        }
    }

    pub fn request_user_info(&self) -> Result<MonobankClientInfo, ApiError> {
        let url = "https://api.monobank.ua/personal/client-info";
        info!("Requesting user info from {}", url);

        let headers = self.build_headers()?;
        let res = self.http_client.get(url).headers(headers).send()?;
        if res.status().is_success() {
            let api_response: MonobankClientInfo = res.json()?;
            Ok(api_response)
        } else {
            warn!("Failed to request user info: {}", res.status());
            Err(ApiError::Unknown)
        }
    }

    pub fn request_payments(&self, account: &str, from: &str, to: &str) -> Result<Vec<PaymentsInfo>, ApiError> {
        let url = format!("https://api.monobank.ua/personal/statement/{}/{}/{}", account, from, to);
        info!("Requesting payments from {}", url);

        let headers = self.build_headers()?;
        let res = self.http_client.get(&url).headers(headers).send()?;
        if res.status().is_success() {
            let api_response: Vec<PaymentsInfo> = res.json()?;
            Ok(api_response)
        } else {
            warn!("Failed to request payments: {}", res.status());
            Err(ApiError::Unknown)
        }
    }
}

pub fn to_abbreviation(code: i32) -> String {
    match Currency::from_numeric(code.try_into().unwrap()) {
        Some(currency) => currency.code().to_string(),
        None => String::from("Unknown"),
    }
}