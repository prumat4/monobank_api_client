use std::env;
use dotenv::dotenv;
use std::collections::BTreeSet;

use monobank_api::api_client::{Client, MonobankClientInfo, Account, Currencies, to_abbreviation};

fn get_unique_currencies(accounts: &Vec<Account>) -> BTreeSet<i32> {
    let mut unique_currencies: BTreeSet<i32> = BTreeSet::new();

    for account in accounts {
        unique_currencies.insert(*account.currency_code());
    }

    unique_currencies
}

fn get_exchange_rates(unique_currencies: &BTreeSet<i32>, currencies_exchange_rates: &Vec<Currencies>) -> Vec<(i32, i32, f32, f32)> {
    let mut exchange_rates = Vec::new();

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

fn get_total_balance_by_currency(accounts: &Vec<Account>, currency: &i32) -> f32 {
    let mut total: f32 = 0.0;
    for account in accounts {
        if account.currency_code() == currency {
            total += account.balance();
        }
    }

    total
}

fn get_total_balances(accounts: &Vec<Account>) -> Vec<(i32, f32)> {
    let mut total_balances = Vec::new();
    let unique_currencies = get_unique_currencies(accounts);

    for currency in unique_currencies {
        total_balances.push((currency, get_total_balance_by_currency(accounts, &currency)));
    }

    total_balances
}

fn convert_balances_to_currency(
    accounts: &Vec<Account>,
    target_currency: i32,
    currencies_exchange_rates: &Vec<Currencies>,
) -> f32 {
    let unique_currencies = get_unique_currencies(accounts);
    let total_balances = get_total_balances(accounts);
    let exchange_rates_list = get_exchange_rates(&unique_currencies, currencies_exchange_rates);

    let mut total: f32 = 0.0;

    for &(currency, amount) in &total_balances {
        if currency == target_currency {
            total += amount;
        } else {
            let mut conversion_rate: Option<f32> = None;
            for &(from, to, rate_buy, rate_sell) in &exchange_rates_list {
                if from == currency && to == target_currency {
                    conversion_rate = Some(rate_buy);
                    break;
                } else if to == currency && from == target_currency {
                    conversion_rate = Some(1.0 / rate_sell);
                    break;
                }
            }
            if let Some(rate) = conversion_rate {
                total += amount * rate;
            } else {
                eprintln!("No conversion rate found for currency pair ({}, {})", currency, target_currency);
            }
        }
    }

    total
}

fn main() {
    dotenv().ok();
    let uah = 980;
    let usd = 840;
    let eur = 978;

    let api_key: String = match env::var("API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("API_KEY must be set");
            return;
        },
    };

    let client = Client::new(&api_key);

    let user_info: MonobankClientInfo = match client.request_user_info() {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Failed to request user info: {:?}", e);
            return;
        },
    };
    let accounts = user_info.accounts();
    
    let currencies_exchange_rates = match client.request_currencies() {
        Ok(rates) => rates,
        Err(e) => {
            eprintln!("Failed to request currencies: {:?}", e);
            return;
        },
    };

    let total_uah = get_total_balance_by_currency(&accounts, &uah);
    let total_usd = get_total_balance_by_currency(&accounts, &usd);
    let total_eur = get_total_balance_by_currency(&accounts, &eur);

    println!("Total UAH: {}", total_uah);
    println!("Total USD: {}", total_usd);
    println!("Total EUR: {}", total_eur);

    let converted_to_usd = convert_balances_to_currency(&accounts, usd, &currencies_exchange_rates);        
    let converted_to_uah = convert_balances_to_currency(&accounts, uah, &currencies_exchange_rates);        

    println!("Total in USD: {}", converted_to_usd);
    println!("Total in UAH: {}", converted_to_uah);

    let unique_currencies = get_unique_currencies(&accounts);
    let exchange_rates = get_exchange_rates(&unique_currencies, &currencies_exchange_rates);

    let mut uah_usd_buy = 0.0;
    let mut uah_usd_sell = 0.0;
    let mut uah_eur_buy = 0.0;
    let mut uah_eur_sell = 0.0;

    for &(from, to, rate_buy, rate_sell) in &exchange_rates {
        if (from == uah && to == usd) || (from == usd && to == uah) {
            uah_usd_buy = rate_buy;
            uah_usd_sell = rate_sell;

        }
        if (from == uah && to == eur) || (from == eur && to == uah) {
            uah_eur_buy = rate_buy;
            uah_eur_sell = rate_sell;
        }
    }

    println!("Exchange rate (UAH to USD): Buy = {}, Sell = {}", uah_usd_buy, uah_usd_sell);
    println!("Exchange rate (UAH to EUR): Buy = {}, Sell = {}", uah_eur_buy, uah_eur_sell);

}
