use super::{application::EntryRepository, domain::Entry};
use crate::constants;
use async_trait::async_trait;
use sqlx::mysql::MySqlPool;
use std::error::Error;

const QUERY_FIND_ENTRY_BY_WORD: &str = "SELECT word, definition FROM entries WHERE word = ?";
const QUERY_DELETE_BY_WORD: &str = "DELETE FROM entries WHERE word = ?";
const QUERY_UPDATE_ENTRY: &str = "INSERT INTO entries (word, definition) VALUES (?, ?)";

type EntryRow = (String, String); //word, definition

pub struct MySQLEntryRepository<'a> {
    pub pool: &'a MySqlPool,
}

#[async_trait]
impl<'a> EntryRepository for MySQLEntryRepository<'a> {
    async fn find_by_word(&self, word: &str) -> Result<Entry, Box<dyn Error>> {
        let row: EntryRow = {
            sqlx::query_as(QUERY_FIND_ENTRY_BY_WORD)
                .bind(word)
                .fetch_one(self.pool)
                .await?
        };

        if row.0 == "" {
            return Err(constants::ERR_NOT_FOUND.into());
        }

        Ok(Entry {
            word: row.0.clone(),
            definition: row.1.clone(),
        })
    }

    async fn save(&self, entry: &Entry) -> Result<(), Box<dyn Error>> {
        sqlx::query(QUERY_UPDATE_ENTRY)
            .bind(&entry.word)
            .bind(&entry.definition)
            .execute(self.pool)
            .await?;

        Ok(())
    }
    async fn delete_by_word(&self, word: &str) -> Result<(), Box<dyn Error>> {
        sqlx::query(QUERY_DELETE_BY_WORD)
            .bind(word)
            .execute(self.pool)
            .await?;

        Ok(())
    }
}
