# Project Context

## Purpose
Офчейн-мониторинг мем-коинов в сети Solana.
Цель: в реальном времени ловить “выстрелы” мем-токенов и отправлять алерты в Telegram.
Реальное-временное отслеживание мем-коинов в сети Solana через Helius события.
Фильтрация по market cap — пользователь задаёт минимальный и максимальный пороги MC, и монитор присылает в приватный Telegram-бот только те мем-коины, которые входят в указанный диапазон.

## Tech Stack
- Rust 2024, async runtime Tokio
- Axum HTTP server exposing `/webhook` for Helius callbacks
- serde/serde_json для приёма и разборки входящих payload
- Telegram Bot API для отправки алертов (приватный чат с одним user_id)
- Moralis API как источник цены и market cap для токена
- Helius webhook как источник событий по Solana

## Project Conventions

### Code Style
- rustfmt + clippy по умолчанию; придерживаться идиоматичных Result/`?` цепочек
- Чёткие типы входного payload вместо `serde_json::Value`, когда структура будет зафиксирована
- Конфигурационные значения (пороги, chat id) — в одном конфиг-модуле/файле

### Architecture Patterns
- Поток: Helius Webhook → HTTP endpoint (axum) → Event Handler → Analyzer (фильтры) → Telegram Notify
- Анализатор: фильтр по типу события (swap/mint/transfer), запрос метрик в Moralis, проверка market cap и процента роста за конфигурируемое окно
- Логирование: входящие события, решение фильтра (почему прошло/не прошло), ошибки внешних API, изменения настроек
- Ожидается хранение единственного Telegram user_id (приватный канал оповещений)

### Testing Strategy
- Юнит-тесты для фильтров (market cap, рост за окно времени) и парсеров входящих webhook payload
- Интеграционные тесты/фальшивые payload Helius для проверки end-to-end цепочки до этапа оповещения
- Ручные прогоны для Telegram-уведомлений с тестовым токеном, пока нет моков API

### Git Workflow
- Основная ветка `main`; фичи — в короткоживущих ветках с rebase перед merge
- Коммиты в повелительном наклонении, небольшие и тематические

## Domain Context
- Мем-коины Solana; важно быстро ловить всплески цены/объёма
- Входящие поля Helius: signature, slot/blockTime, accountKeys, parsedInstructions/tokenTransfers, pre/postTokenBalances, status, logMessages
- Необходимо определять buy/sell по балансовым изменениям и tokenTransfers; объём в USD = количество × цена из Moralis
- Настройки из Telegram: `/start`, `/stop`, `/set_threshold <percent> <seconds>`, `/set_min_cap <usd>`, `/status`

## Important Constraints
- Почти моментальная доставка алертов; минимальная задержка на сетевые вызовы
- Надёжность: логировать ошибки внешних API (Moralis, Telegram) и сбои сервера
- Фильтры должны быть настраиваемыми по market cap и по росту за окно времени
- Логи экспортируемые в JSON/CSV; хранить причину прохождения/отклонения события фильтром

## External Dependencies
- Helius webhook (источник событий Solana)
- Moralis API (метрики токена: цена, market cap, volume)
- Telegram Bot API (отправка уведомлений в приватный чат)
