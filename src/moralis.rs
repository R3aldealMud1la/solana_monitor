use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;

#[derive(Clone)]
pub struct MoralisClient {
    http: Client,
    api_key: String,
    base_url: String,
}

impl MoralisClient {
    pub fn new(api_key: String, base_url: String) -> Self {
        let http = reqwest::Client::builder()
            .no_proxy()
            .build()
            .expect("failed to build Moralis HTTP client");

        Self {
            http,
            api_key,
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub async fn fetch_token_metrics(&self, mint: &str) -> Result<TokenMetrics, MoralisError> {
        let url = format!("{}/tokens/{mint}/price?chain=solana", self.base_url);
        let response = self
            .http
            .get(url)
            .header("X-API-Key", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(MoralisError::HttpStatus(response.status()));
        }

        let payload: MoralisPriceResponse = response.json().await?;
        Ok(TokenMetrics {
            price_usd: payload.usd_price,
            market_cap_usd: payload.market_cap_usd,
        })
    }
}

#[derive(Debug, Clone)]
pub struct TokenMetrics {
    pub price_usd: Option<f64>,
    pub market_cap_usd: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct MoralisPriceResponse {
    #[serde(alias = "usdPrice", alias = "usd_price")]
    usd_price: Option<f64>,
    #[serde(alias = "marketCapUsd", alias = "market_cap_usd")]
    market_cap_usd: Option<f64>,
}

#[derive(Debug, Error)]
pub enum MoralisError {
    #[error("moralis request failed: {0}")]
    Transport(#[from] reqwest::Error),
    #[error("moralis returned non-success status {0}")]
    HttpStatus(reqwest::StatusCode),
}
