# Change: Core Solana meme coin monitoring with market cap filter and Telegram alerts

## Why
- Deliver the first end-to-end alert loop for Solana meme tokens so operators can see spikes quickly.

## What Changes
- Expose `/webhook` to accept Helius events and hand them to an analyzer.
- Enrich events with token metrics via Moralis and apply configurable market cap filters.
- Send Telegram alerts for passing events; log intake, filter decisions, and external API errors.

## Impact
- Affected specs: monitoring
- Affected code: src/main.rs, new analyzer/clients modules, config/env wiring
