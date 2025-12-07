use crate::{
    config::{AppConfig, MarketCapBounds},
    helius::HeliusWebhook,
    moralis::{MoralisClient, MoralisError, TokenMetrics},
    telegram::{TelegramClient, TelegramError},
};
use thiserror::Error;
use tracing::{error, info, warn};

#[derive(Clone)]
pub struct Analyzer {
    moralis: MoralisClient,
    telegram: TelegramClient,
    chat_id: String,
    cap_filter: MarketCapFilter,
}

impl Analyzer {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            moralis: MoralisClient::new(
                config.moralis_api_key.clone(),
                config.moralis_base_url.clone(),
            ),
            telegram: TelegramClient::new(
                config.telegram_bot_token.clone(),
                config.telegram_api_base.clone(),
            ),
            chat_id: config.telegram_chat_id.clone(),
            cap_filter: MarketCapFilter::new(config.market_cap_bounds.clone()),
        }
    }

    pub async fn process_event(&self, event: HeliusWebhook) -> Result<(), AnalyzerError> {
        let signature = event.signature.clone();
        let maybe_mint = event.primary_mint().map(str::to_string);

        info!(
            signature = signature.as_str(),
            mint = maybe_mint.as_deref().unwrap_or(""),
            "intake_event"
        );

        let mint = match maybe_mint {
            Some(mint) => mint,
            None => {
                warn!(
                    signature = signature.as_str(),
                    reason = "missing_mint",
                    "skip_event"
                );
                return Ok(());
            }
        };

        let metrics = match self.moralis.fetch_token_metrics(&mint).await {
            Ok(metrics) => metrics,
            Err(err) => {
                error!(
                    signature = signature.as_str(),
                    mint = mint.as_str(),
                    error = ?err,
                    source = "moralis",
                    "external_api_error"
                );
                return Err(err.into());
            }
        };

        let market_cap = match metrics.market_cap_usd {
            Some(cap) => cap,
            None => {
                warn!(
                    signature = signature.as_str(),
                    mint = mint.as_str(),
                    reason = "missing_market_cap",
                    "skip_event"
                );
                return Ok(());
            }
        };

        match self.cap_filter.evaluate(market_cap) {
            FilterOutcome::Pass => {
                info!(
                    signature = signature.as_str(),
                    mint = mint.as_str(),
                    market_cap_usd = market_cap,
                    decision = "pass",
                    reason = "within_range",
                    "market_cap_filter_decision"
                );
                self.send_alert(&mint, &signature, &metrics, market_cap)
                    .await?;
            }
            FilterOutcome::Fail { reason } => {
                info!(
                    signature = signature.as_str(),
                    mint = mint.as_str(),
                    market_cap_usd = market_cap,
                    decision = "fail",
                    reason = reason,
                    "market_cap_filter_decision"
                );
            }
        }

        Ok(())
    }

    async fn send_alert(
        &self,
        mint: &str,
        signature: &str,
        metrics: &TokenMetrics,
        market_cap: f64,
    ) -> Result<(), AnalyzerError> {
        let price_line = metrics
            .price_usd
            .map(|p| format!("{p:.6}"))
            .unwrap_or_else(|| "n/a".to_string());

        let message = format!(
            "Solana token alert\nMint: {mint}\nSignature: {signature}\nMarket cap (USD): {market_cap:.2}\nPrice (USD): {price_line}"
        );

        if let Err(err) = self.telegram.send_message(&self.chat_id, &message).await {
            error!(
                signature = signature,
                mint = mint,
                error = ?err,
                source = "telegram",
                "alert_delivery_failed"
            );
            return Err(err.into());
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct MarketCapFilter {
    bounds: MarketCapBounds,
}

impl MarketCapFilter {
    pub fn new(bounds: MarketCapBounds) -> Self {
        Self { bounds }
    }

    pub fn evaluate(&self, market_cap: f64) -> FilterOutcome {
        if let Some(min) = self.bounds.min {
            if market_cap < min {
                return FilterOutcome::Fail {
                    reason: "out_of_cap_range",
                };
            }
        }

        if let Some(max) = self.bounds.max {
            if market_cap > max {
                return FilterOutcome::Fail {
                    reason: "out_of_cap_range",
                };
            }
        }

        FilterOutcome::Pass
    }
}

#[derive(Debug, PartialEq)]
pub enum FilterOutcome {
    Pass,
    Fail { reason: &'static str },
}

#[derive(Debug, Error)]
pub enum AnalyzerError {
    #[error(transparent)]
    Moralis(#[from] MoralisError),
    #[error(transparent)]
    Telegram(#[from] TelegramError),
}

#[cfg(test)]
mod tests {
    use super::{FilterOutcome, MarketCapBounds, MarketCapFilter};

    #[test]
    fn passes_when_within_range() {
        let filter = MarketCapFilter::new(MarketCapBounds {
            min: Some(1_000.0),
            max: Some(5_000_000.0),
        });

        assert_eq!(FilterOutcome::Pass, filter.evaluate(10_000.0));
        assert_eq!(FilterOutcome::Pass, filter.evaluate(1_000.0));
        assert_eq!(FilterOutcome::Pass, filter.evaluate(5_000_000.0));
    }

    #[test]
    fn blocks_below_min() {
        let filter = MarketCapFilter::new(MarketCapBounds {
            min: Some(10_000.0),
            max: Some(100_000.0),
        });

        assert_eq!(
            FilterOutcome::Fail {
                reason: "out_of_cap_range"
            },
            filter.evaluate(9_999.99)
        );
    }

    #[test]
    fn blocks_above_max() {
        let filter = MarketCapFilter::new(MarketCapBounds {
            min: Some(10_000.0),
            max: Some(100_000.0),
        });

        assert_eq!(
            FilterOutcome::Fail {
                reason: "out_of_cap_range"
            },
            filter.evaluate(100_000.01)
        );
    }

    #[test]
    fn open_lower_bound_allows_anything_below_max() {
        let filter = MarketCapFilter::new(MarketCapBounds {
            min: None,
            max: Some(1_000_000.0),
        });

        assert_eq!(FilterOutcome::Pass, filter.evaluate(0.0));
        assert_eq!(FilterOutcome::Pass, filter.evaluate(999_999.99));
    }

    #[test]
    fn open_upper_bound_allows_anything_above_min() {
        let filter = MarketCapFilter::new(MarketCapBounds {
            min: Some(5_000.0),
            max: None,
        });

        assert_eq!(
            FilterOutcome::Fail {
                reason: "out_of_cap_range"
            },
            filter.evaluate(4_999.99)
        );
        assert_eq!(FilterOutcome::Pass, filter.evaluate(5_000.0));
        assert_eq!(FilterOutcome::Pass, filter.evaluate(50_000_000.0));
    }
}
