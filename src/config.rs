use std::env;
use dotenvy::dotenv;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct MarketCapBounds {
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub moralis_api_key: String,
    pub moralis_base_url: String,
    pub telegram_bot_token: String,
    pub telegram_chat_id: String,
    pub telegram_api_base: String,
    pub market_cap_bounds: MarketCapBounds,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        // Load .env if present so local runs pick up configuration automatically.
        dotenv().ok();

        let moralis_api_key = required_var("MORALIS_API_KEY")?;
        let telegram_bot_token = required_var("TELEGRAM_BOT_TOKEN")?;
        let telegram_chat_id = required_var("TELEGRAM_CHAT_ID")?;

        let market_cap_bounds = MarketCapBounds {
            min: parse_optional_f64("MARKET_CAP_MIN_USD")?,
            max: parse_optional_f64("MARKET_CAP_MAX_USD")?,
        };

        Ok(Self {
            moralis_base_url: env::var("MORALIS_BASE_URL")
                .unwrap_or_else(|_| "https://deep-index.moralis.io/api/v2.2".to_string()),
            telegram_api_base: env::var("TELEGRAM_API_BASE")
                .unwrap_or_else(|_| "https://api.telegram.org".to_string()),
            moralis_api_key,
            telegram_bot_token,
            telegram_chat_id,
            market_cap_bounds,
        })
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required env var {0}")]
    MissingEnv(String),
    #[error("invalid number for {key}")]
    InvalidNumber {
        key: String,
        #[source]
        source: std::num::ParseFloatError,
    },
}

fn required_var(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|_| ConfigError::MissingEnv(key.to_string()))
}

fn parse_optional_f64(key: &str) -> Result<Option<f64>, ConfigError> {
    match env::var(key) {
        Ok(value) => {
            let parsed = value
                .parse::<f64>()
                .map_err(|source| ConfigError::InvalidNumber {
                    key: key.to_string(),
                    source,
                })?;
            Ok(Some(parsed))
        }
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => Err(ConfigError::MissingEnv(key.to_string())),
    }
}
