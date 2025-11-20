
pub struct Config {
    pub rpc_url: String,
    pub telegram_token: String,
    pub chat_id: u64,
    pub volume_threshold: u64,
}
pub fn load_config() -> Config {
    Config {
        rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
        telegram_token: "токен_бота".to_string(),
        chat_id: 123456789,
        volume_threshold: 100000,
    }
}
    /*
Монитор должен подключаться к блокчейну Solana с помощью RPC
Проверять крунпные транзакции мем коинов(от 100 тысяч market cap) в минутном и пятиминутном таймфрейме
Отправлять адрес контракта в телеграмм
Указать данные метрик для анализа
Продумать частоту проверок со стороны rpc*/