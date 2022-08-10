use super::{application::EntryRepository, domain::Entry};
use crate::constants;
use async_trait::async_trait;
use futures::TryStreamExt;
use sqlx::mysql::MySqlPool;
use sqlx::Row;
use std::error::Error;

const QUERY_FIND_ENTRY_BY_WORD: &str = "SELECT word, definition FROM entries WHERE word = ?";
const QUERY_GET_ALL: &str = "SELECT word, definition FROM entries";
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

    async fn get_all(&self) -> Result<Vec<Entry>, Box<dyn Error>> {
        let mut rows = sqlx::query(QUERY_GET_ALL).fetch(self.pool);
        let mut entries = Vec::new();

        while let Some(row) = rows.try_next().await? {
            let row_word: &str = row.try_get("word")?;
            let row_definition: &str = row.try_get("definition")?;
            entries.push(Entry {
                word: row_word.to_string(),
                definition: row_definition.to_string(),
            });
        }

        Ok(entries)
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
