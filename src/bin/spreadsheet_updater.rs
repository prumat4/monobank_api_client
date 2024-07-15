use std::env;
use dotenv::dotenv;
use std::collections::BTreeSet;

use monobank_api::api_client::{Client, MonobankClientInfo, Account, Currencies, to_abbreviation};

fn unique_currencies(accounts: &Vec<Account>) -> BTreeSet<i32> {
    let mut unique_currencies: BTreeSet<i32> = BTreeSet::<i32>::new();

    for account in accounts {
        unique_currencies.insert(*account.currency_code());
    }

    unique_currencies
}

fn exchange_rates(unique_currencies: &BTreeSet<i32>, currencies_exchange_rates: &Vec<Currencies>) -> Vec<(i32, i32, f32, f32)> {
    let mut exchange_rates = Vec::<(i32, i32, f32, f32)>::new();

    for (i, &first) in unique_currencies.iter().enumerate() {
        for &second in unique_currencies.iter().skip(i + 1) {
            let mut rate_buy = -1.0;
            let mut rate_sell = -1.0;
            for currency in currencies_exchange_rates.iter() {
                if let (Some(currencyCodeA), Some(currencyCodeB), Some(rateBuy), Some(rateSell)) = (currency.currencyCodeA, currency.currencyCodeB, currency.rateBuy, currency.rateSell) {
                    if (first == currencyCodeA && second == currencyCodeB) || (first == currencyCodeB && second == currencyCodeA) {
                        rate_buy = rateBuy as f32;
                        rate_sell = rateSell as f32;
                        break;
                    }
                }
            }
            exchange_rates.push((first, second, rate_buy, rate_sell));
        }
    }

    exchange_rates
}

fn total_currency_balance(accounts: &Vec<Account>, currency: &i32) -> f32 {
    let mut total: f32 = 0.0;
    for account in accounts {
        if account.currency_code() == currency {
            total += account.balance();
        }
    }

    total
}

fn total_of_each_currencies(accounts: &Vec<Account>) -> Vec<(i32, f32)> {
    let mut total: Vec<(i32, f32)> = Vec::<(i32, f32)>::new();
    let unique_currencies = unique_currencies(accounts);

    for currency in unique_currencies {
        total.push((currency, total_currency_balance(accounts, &currency)));
    }

    dbg!(&total);

    total
}

fn convert_to_one_currency(
    accounts: &Vec<Account>,
    currency_to_convert: i32,
    currencies_exchange_rates: &Vec<Currencies>,
) -> f32 {
    let unique_currencies = unique_currencies(accounts);
    let total_of_each_currency = total_of_each_currencies(accounts);
    let exchange_rates_list = exchange_rates(&unique_currencies, currencies_exchange_rates);

    let mut total: f32 = 0.0;

    for &(currency, amount) in &total_of_each_currency {
        if currency == currency_to_convert {
            total += amount;
        } else {
            let mut conversion_rate: Option<f32> = None;
            for &(from, to, rate_buy, rate_sell) in &exchange_rates_list {
                if from == currency && to == currency_to_convert {
                    conversion_rate = Some(rate_buy);
                    break;
                } else if to == currency && from == currency_to_convert {
                    conversion_rate = Some(1.0 / rate_sell);
                    break;
                }
            }
            if let Some(rate) = conversion_rate {
                total += amount * rate;
            } else {
                eprintln!("No conversion rate found for currency pair ({}, {})", currency, currency_to_convert);
            }
        }
    }

    total
}


fn main() {
    dotenv().ok();
    let key: String = match env::var("API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("API_KEY must be set");
            return;
        },
    };

    let client = Client::new(&key);

    let user_info: MonobankClientInfo = match client.request_user_info() {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Failed to request user info: {:?}", e);
            return;
        },
    };
    let accounts = user_info.accounts();
    let mut total_balance: Vec<(f32, i32)> = Vec::<(f32, i32)>::new();
    
    let currencies_exchange_rates = match client.request_currencies() {
        Ok(rates) => rates,
        Err(e) => {
            eprintln!("Failed to request currencies: {:?}", e);
            return;
        },
    };

    let total_in_usd = convert_to_one_currency(&accounts, 840, &currencies_exchange_rates);        
    let total_in_uah = convert_to_one_currency(&accounts, 980, &currencies_exchange_rates);        

    println!("total_in_usd: {}", total_in_usd);
    println!("total_in_uah: {}", total_in_uah);

}