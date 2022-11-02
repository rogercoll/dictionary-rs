# dictionary-rs

Simple key-value store and retrieval application writen in Rust.

Purpose: Personal spanish/english dictionary using a Telegram bot.

## Usage

### Telegram bot

The most simple and fastest way to run the dictionary is by using Podman-compose (aka docker-compose), you
can find the [compose.yaml](./compose.yaml) file in the root directory of the repository.

Just need to set the `MARIADB_ROOT_PASSWORD` and `TELOXIDE_TOKEN` environment
variables of the compose file.

## Storage interface

```rust
#[async_trait]
pub trait EntryRepository: Sync + Send {
    async fn find_by_word(&self, word: &str) -> Result<Entry, Box<dyn Error>>;
    async fn get_all(&self) -> Result<Vec<Entry>, Box<dyn Error>>;
    async fn save(&self, entry: &Entry) -> Result<(), Box<dyn Error>>;
    async fn delete_by_word(&self, word: &str) -> Result<(), Box<dyn Error>>;
}
```

[Current implementation](https://github.com/rogercoll/dictionary-rs/blob/main/src/entry/repository.rs): MySQL Client using sqlx.
