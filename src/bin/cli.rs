#[macro_use]
extern crate log;

use argh::FromArgs;
use dictionary::dictionary::application::DictionaryApplication;
use dictionary::entry::repository::MySQLEntryRepository;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;
use std::sync::Arc;
use tokio::sync::OnceCell;

const DEFAULT_POOL_SIZE: u32 = 10;

const ENV_MYSQL_DSN: &str = "DATABASE_URL";
const ENV_MYSQL_POOL: &str = "DATABASE_POOL_SIZE";

static MYSQL_POOL: OnceCell<MySqlPool> = OnceCell::const_new();

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
struct TopLevel {
    #[argh(subcommand)]
    nested: MySubCommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum MySubCommandEnum {
    One(GetEntry),
    Two(AddEntry),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Get subcommand.
#[argh(subcommand, name = "get")]
struct GetEntry {
    #[argh(option, short = 'w')]
    /// entry word
    word: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Get subcommand.
#[argh(subcommand, name = "add")]
struct AddEntry {
    #[argh(option, short = 'w')]
    /// entry word
    word: String,

    #[argh(option, short = 'd')]
    /// entry definition
    definition: String,
}

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

#[tokio::main]
async fn main() {
    let config: TopLevel = argh::from_env();
    let entry_repo = Arc::new(MySQLEntryRepository {
        pool: MYSQL_POOL.get_or_init(get_pool).await,
    });

    let dict_app = DictionaryApplication {
        entry_repo: entry_repo,
    };

    match config.nested {
        MySubCommandEnum::One(get_config) => {
            let definition = dict_app.get_definition(&get_config.word).await.unwrap();
            println!("{}", definition);
        }
        MySubCommandEnum::Two(add_config) => {
            dict_app
                .store_definition(&add_config.word, &add_config.definition)
                .await
                .unwrap();
        }
    }
}
