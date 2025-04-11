use serde::Deserialize;
use std::{error::Error, fs::File, io::Write, thread, time::Duration};

#[derive(Debug, Deserialize)]
struct PriceData {
    usd: f64,
}

#[derive(Debug, Deserialize)]
struct BitcoinResponse {
    bitcoin: PriceData,
}

#[derive(Debug, Deserialize)]
struct EthereumResponse {
    ethereum: PriceData,
}

#[derive(Debug)]
enum ApiResult<T> {
    Success(T),
    ApiError(String),
    NetworkError(String),
}

trait Pricing {
    fn fetch_price(&self) -> ApiResult<f64>;
    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>>;
}

// Bitcoin Struct and Implementation
#[derive(Debug)]
struct Bitcoin;

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> ApiResult<f64> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        match ureq::get(url).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<BitcoinResponse>() {
                        Ok(parsed) => ApiResult::Success(parsed.bitcoin.usd),
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", response.status()))
                }
            }
            Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
        }
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>> {
        let mut file = File::create("bitcoin_price.txt")?;
        writeln!(file, "Bitcoin Price: ${}", price)?;
        Ok(())
    }
}

// Ethereum Struct and Implementation
#[derive(Debug)]
struct Ethereum;

impl Pricing for Ethereum {
    fn fetch_price(&self) -> ApiResult<f64> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        match ureq::get(url).call() {
            Ok(response) => {
                if response.status() == 200 {
                    match response.into_json::<EthereumResponse>() {
                        Ok(parsed) => ApiResult::Success(parsed.ethereum.usd),
                        Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                    }
                } else {
                    ApiResult::ApiError(format!("HTTP error: {}", response.status()))
                }
            }
            Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
        }
    }

    fn save_to_file(&self, price: f64) -> Result<(), Box<dyn Error>> {
        let mut file = File::create("ethereum_price.txt")?;
        writeln!(file, "Ethereum Price: ${}", price)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let bitcoin = Bitcoin;
    let ethereum = Ethereum;

    println!("📈 Crypto Price Tracker");
    println!("=======================\n");

    loop {
        println!("🔄 Fetching prices...");

        // Bitcoin
        match bitcoin.fetch_price() {
            ApiResult::Success(price) => {
                bitcoin.save_to_file(price)?;
                println!("✅ Bitcoin: ${}", price);
            }
            ApiResult::ApiError(e) => println!("❌ Bitcoin API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Bitcoin Network Error: {}", e),
        }

        // Ethereum
        match ethereum.fetch_price() {
            ApiResult::Success(price) => {
                ethereum.save_to_file(price)?;
                println!("✅ Ethereum: ${}", price);
            }
            ApiResult::ApiError(e) => println!("❌ Ethereum API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Ethereum Network Error: {}", e),
        }

        println!("⏳ Waiting 10 seconds...\n");
        thread::sleep(Duration::from_secs(10));
    }
}