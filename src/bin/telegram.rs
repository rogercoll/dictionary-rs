#[macro_use]
extern crate log;

use dictionary::dictionary::application::DictionaryApplication;
use teloxide::dptree::endpoint;

use dictionary::entry::repository::MySQLEntryRepository;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;
use std::error::Error;
use std::sync::Arc;
use teloxide::{prelude::*, utils::command::BotCommands};
use tokio::sync::OnceCell;

const DEFAULT_POOL_SIZE: u32 = 10;

const ENV_MYSQL_DSN: &str = "DATABASE_URL";
const ENV_MYSQL_POOL: &str = "DATABASE_POOL_SIZE";

static MYSQL_POOL: OnceCell<MySqlPool> = OnceCell::const_new();

async fn get_pool() -> MySqlPool {
    let mysql_dsn = env::var(ENV_MYSQL_DSN).expect("mysql url must be set");

    let mysql_pool = env::var(ENV_MYSQL_POOL)
        .map(|pool_size| pool_size.parse().unwrap())
        .unwrap_or(DEFAULT_POOL_SIZE);

    MySqlPoolOptions::new()
        .max_connections(mysql_pool)
        .connect(&mysql_dsn)
        .await
        .map(|pool| {
            info!("connection with mysql cluster established");
            pool
        })
        .map_err(|err| format!("establishing connection with {}: {}", mysql_dsn, err))
        .unwrap()
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "get an entry.")]
    Get(String),
    #[command(
        description = "add a new entry into the database.",
        parse_with = "split"
    )]
    Add(String, String),
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    app: Arc<DictionaryApplication>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let text = message.text();
    if text.is_none() {
        return Ok(());
    }
    if let Ok(command) = Command::parse(text.unwrap(), "DictionaryBot") {
        match command {
            Command::Help => {
                bot.send_message(message.chat.id, Command::descriptions().to_string())
                    .await?
            }
            Command::Get(word) => {
                let output = match app.get_definition(&word).await {
                    Ok(def) => format!("{}: {}", word, def),
                    Err(error) => format!("Error found: {}", error),
                };
                bot.send_message(message.chat.id, output).await?
            }
            Command::Add(word, definition) => {
                let output = match app.store_definition(&word, &definition).await {
                    Ok(_) => return Ok(()),
                    Err(error) => format!("Error while storing entry: {}", error),
                };
                bot.send_message(message.chat.id, output).await?
            }
        };
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let entry_repo = Arc::new(MySQLEntryRepository {
        pool: MYSQL_POOL.get_or_init(get_pool).await,
    });

    let dict_app = Arc::new(DictionaryApplication {
        entry_repo: entry_repo,
    });

    let bot = Bot::from_env().auto_send();

    let message_handler = Update::filter_message().branch(endpoint(answer));

    let handler = dptree::entry().branch(message_handler);
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![dict_app])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
