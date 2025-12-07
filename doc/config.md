# Configuration

Set the following environment variables before running the server. You can copy `.env.example` to `.env` and adjust the values.

- `MORALIS_API_KEY` (required): API key for Moralis price/market-cap queries.
- `MORALIS_BASE_URL` (optional, default `https://deep-index.moralis.io/api/v2.2`): Override the Moralis API base URL.
- `TELEGRAM_BOT_TOKEN` (required): Bot token used to send alerts.
- `TELEGRAM_CHAT_ID` (required): Chat ID that will receive alerts.
- `TELEGRAM_API_BASE` (optional, default `https://api.telegram.org`): Override Telegram API base URL.
- `MARKET_CAP_MIN_USD` (optional): Minimum market cap to pass the filter (number).
- `MARKET_CAP_MAX_USD` (optional): Maximum market cap to pass the filter (number).

Loading order:
- The app reads values directly from the environment; use a `.env` loader in your shell or a process manager if desired.
- Optional values can be omitted; bounds are applied only when set.
