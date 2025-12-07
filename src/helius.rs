use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusWebhook {
    pub signature: String,
    #[serde(default)]
    pub events: HeliusEvents,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusEvents {
    #[serde(default)]
    pub token_transfers: Vec<TokenTransfer>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct TokenTransfer {
    pub mint: String,
    #[serde(default)]
    pub from_user_account: Option<String>,
    #[serde(default)]
    pub to_user_account: Option<String>,
    #[serde(default)]
    pub token_amount: Option<f64>,
}

impl HeliusWebhook {
    pub fn primary_mint(&self) -> Option<&str> {
        self.events
            .token_transfers
            .first()
            .map(|transfer| transfer.mint.as_str())
    }
}
