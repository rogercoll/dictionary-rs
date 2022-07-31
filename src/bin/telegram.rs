#[macro_use]
extern crate log;

use dictionary::dictionary::application::DictionaryApplication;
use dictionary::entry::application::EntryRepository;
use dictionary::entry::repository::MySQLEntryRepository;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;
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
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

#[tokio::main]
async fn main() {
    let entry_repo = Arc::new(MySQLEntryRepository {
        pool: MYSQL_POOL.get_or_init(get_pool).await,
    });

    let dict_app = DictionaryApplication {
        entry_repo: entry_repo,
    };

    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(
        bot,
        |message: Message,
         bot: AutoSend<Bot>,
         command: Command,
         app: Arc<DictionaryApplication>| async move {
            bot.send_dice(message.chat.id).await?;
            respond(())
        },
        Command::ty(),
    )
    .await;
}
