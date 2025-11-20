use axum::{
    routing::post,
    extract::Json,
    Router,
};
use serde_json::Value;
use std::net::SocketAddr;

async fn webhook_handler(Json(payload): Json<Value>) {
    println!("Got webhook payload: {payload}");
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/webhook", post(webhook_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}


    /*роли: система, пользователь
    //
    //Как пользователь,
     я хочу получать в Telegram-боте уведомления
      об необычных транзакциях в блокчейне Solana,
       отфильтрованные по рыночной капитализации токена, 
       чтобы быстро реагировать на значимые события.
    //
    задачи:
1.Порог по marcet cap в мем коинах должен быть опциональным рыночная капитализация в долларах сша
2.Необычным считать рост мем коина если он вырос в цене на n процентов от своей стоимости,чтобы это опять же можно было бы настроить.
3.Поддержка конфигурационых правил нужна,чтобы выставлять промежутки времени за который выросла монета.Чтобы была возможность настроить время в рамках которого вырастет монета.То есть я хочу например в один день искать монеты которые вырастут в цене за 4 секунды,может за 20 или вообще за минуту и больше
Также хочу чтобы можно было фильтровать актив по рыночной капитализации,например сегодня я включаю трекер и хочу чтобы сегодня он отслеживал монеты с крупной рыночной капитализацией,например от миллиона долларов или больше
4.Уведомления присылать в приватный чат со мной.
5.Я хочу чтобы уведомления приходили почти моментально
6.Предпочтения по источнику этих данных-axiom,gmgn
7.Как только что-то произошло, хочу об этом знать.
8.Подумать про логи,dockercontainer
9.Возможность экспортировать логи в JSON/CSV.

Поддержка команд в Telegram:

/start — запустить мониторинг

/stop — остановить

/set_threshold <percent> <seconds>

/set_min_cap <usd>

/status — показать текущие настройки
Источник информации: Helius webhook
Helius Webhook  →  HTTPS endpoint (Rust server)  →  Event Handler  →  Analyzer → Telegram Notify
                                    ЛОГИ
                                    Где логировать (обязательно)

Входящие события от Helius

тип события (SWAP, MINT, TRANSFER),

адрес токена (mint_address),

сумма/объём,

временная метка (timestamp),

tx_signature.
✅ нужно для контроля корректности webhook и аналитики.

Решение фильтра

записывать, почему токен прошёл или не прошёл фильтр:
reason: "market_cap < 100000" или "passed volatility filter".
✅ нужно для настройки фильтров и понимания, почему алерты не приходят.

Ошибки и сбои

запрос к API GMGN не вернул данные,

Telegram-бот не смог отправить сообщение,

сервер недоступен.
✅ критично для стабильности.

Изменения настроек в Telegram

log: "user changed min_cap=100000 max_cap=500000"
✅ нужно, чтобы понимать текущие фильтры и историю их изменений.

                                            TELEGRAM
                                            Храни один Telegram ID (user_id) в конфиге.
                                            РАБОТА САМОГО СКРИПТА
                                            Helius Webhook (слушает Solana события)
                                                           +
                                            Конфигуриремый профиль по market cap               
    ↓
   Rust скрипт:
                                       ├ фильтрует по типу события (swap, mint, transfer)
                                       ├ берет mint_address токена
                                       ├ запрашивает GMGN API (цена, market cap, volume)
                                       ├ применяет фильтр по market cap и % роста
  ↓
                                       Отправка алерта в Telegram (mint + ссылка на график)
                                       ОЦЕНКА ВОЛАТИЛЬНОСТИ
                                       Что значит «Helius даёт on-chain контекст (кто что купил/продал)»

Что приходит в payload (важные поля):

signature — подпись транзакции (идентификатор).

slot, blockTime — позиция в цепочке и время.

transaction:

message → accountKeys — список вовлечённых аккаунтов (ключей).

instructions и/или parsedInstructions — инструкции в транзакции (swap, transfer, mint, approve и т.д.).

innerInstructions — инструкции, выполненные внутри транзакции (полезно для DEX).

meta:

preTokenBalances / postTokenBalances — балансы токенов до и после (по владельцу и mint).

status (succeeded/failed).

logMessages — текстовые логи программ.

Дополнительно (Helius enhanced): tokenTransfers, events (предопарсенные swap/mint/transfer события).

Как по этим данным понять «кто купил/продал»:

Идентификация участника: первым в accountKeys обычно идёт подписавший транзакцию (signer). Иногда в parsedInstruction есть поле authority / owner.

Что именно куплено/продано: ищем parsedInstructions типа swap или transfer либо tokenTransfers — они содержат mint (адрес токена) и amount.

Направление (buy/sell):

Для простого transfer: сравнить preTokenBalances и postTokenBalances у конкретных аккаунтов — если баланс токена у подписанта ↑ → покупка, ↓ → продажа.

Для DEX swap: в parsedInstruction/tokenTransfers обычно видно, какие mints ушли, а какие пришли; сопоставляя с ценой пулов/quote token, можно определить buy vs sell.

Объём в USD: взять количество токена × цена (из GMGN/Axiom или DEX pool) → получить USD-эквивалент крупной сделки.

Детали для анализа: innerInstructions + logMessages объясняют сложные сценарии (маркет-мейкеры, несколько swap подряд).

Примечание: Helius часто предоставляет «enhanced parsed» события (готовые tokenTransfers, events), что ускоряет логику и уменьшает количество собственных парсеров.

*/
