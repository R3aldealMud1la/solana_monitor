## ADDED Requirements

### Requirement: Helius Webhook Intake
The system SHALL expose an HTTP POST `/webhook` endpoint that accepts Helius Solana events and responds with 2xx within 2 seconds, enqueueing the payload for analysis.

#### Scenario: Accept valid event
- **WHEN** Helius sends a transaction event payload to `/webhook`
- **THEN** the server responds with HTTP 2xx within 2 seconds and hands the payload to the analyzer

### Requirement: Token Metrics Enrichment via Moralis
The analyzer SHALL request token price and market cap for the event's mint address from Moralis using a configured API key.

#### Scenario: Fetch token metrics
- **WHEN** the analyzer receives an event containing a token mint
- **THEN** it requests the token's price and market cap from Moralis using the configured API key

#### Scenario: Missing metrics handled
- **WHEN** Moralis responds without market cap for the mint
- **THEN** the analyzer SHALL skip alerting for that event and log reason `missing_market_cap`

### Requirement: Market Cap Filter
The analyzer SHALL evaluate each event's token market cap against configurable minimum and maximum thresholds and determine pass/fail.

#### Scenario: Within configured range passes
- **WHEN** a token's market cap is between the configured minimum and maximum (inclusive; open-ended bounds allowed)
- **THEN** the event passes the market cap filter and continues to alerting

#### Scenario: Outside range blocked
- **WHEN** a token's market cap is below the configured minimum or above the configured maximum
- **THEN** the analyzer suppresses alerting and logs reason `out_of_cap_range`

### Requirement: Telegram Alert Delivery
The system SHALL send a Telegram message to a configured chat ID for any event that passes all filters, including token mint, transaction signature, and market cap.

#### Scenario: Send alert on pass
- **WHEN** an event passes filtering
- **THEN** the system sends a Telegram message to the configured chat ID containing the token mint, transaction signature, and market cap

### Requirement: Operational Logging
The system SHALL log structured records for intake, filter decisions, and external API/Telegram errors to support debugging and export to JSON/CSV.

#### Scenario: Log lifecycle events
- **WHEN** processing an event
- **THEN** the system logs intake, filter decision reason, and any Moralis/Telegram error in a structured format
