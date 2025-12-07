use reqwest::Client;
use serde::Serialize;
use thiserror::Error;

#[derive(Clone)]
pub struct TelegramClient {
    http: Client,
    bot_token: String,
    base_url: String,
}

impl TelegramClient {
    pub fn new(bot_token: String, base_url: String) -> Self {
        let http = reqwest::Client::builder()
            .no_proxy()
            .build()
            .expect("failed to build Telegram HTTP client");

        Self {
            http,
            bot_token,
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub async fn send_message(&self, chat_id: &str, text: &str) -> Result<(), TelegramError> {
        let url = format!("{}/bot{}/sendMessage", self.base_url, self.bot_token);
        let payload = TelegramMessage {
            chat_id,
            text,
            disable_web_page_preview: true,
        };

        let response = self.http.post(url).json(&payload).send().await?;
        if !response.status().is_success() {
            return Err(TelegramError::HttpStatus(response.status()));
        }

        Ok(())
    }
}

#[derive(Serialize)]
struct TelegramMessage<'a> {
    chat_id: &'a str,
    text: &'a str,
    disable_web_page_preview: bool,
}

#[derive(Debug, Error)]
pub enum TelegramError {
    #[error("telegram request failed: {0}")]
    Transport(#[from] reqwest::Error),
    #[error("telegram returned non-success status {0}")]
    HttpStatus(reqwest::StatusCode),
}
