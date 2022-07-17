use super::domain::Dictionary;
use crate::entry::{application::EntryRepository, domain::Entry};
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

// can contain additional features as user access permissions, random entry return, caching, etc
pub struct DictionaryApplication<E: EntryRepository> {
    pub entry_repo: Arc<E>,
}

impl<E: EntryRepository> DictionaryApplication<E> {
    pub async fn store_definition(
        &self,
        word: &str,
        definition: &str,
    ) -> Result<(), Box<dyn Error>> {
        let entry_to_store = Entry::new(word, definition);
        self.entry_repo.save(&entry_to_store).await?;
        Ok(())
    }
    pub async fn get_definition(&self, word: &str) -> Result<String, Box<dyn Error>> {
        let entry = self.entry_repo.find_by_word(word).await?;
        Ok(entry.get_definition().to_string())
    }
}

#[cfg(test)]
pub mod tests {
    use super::DictionaryApplication;
    use crate::entry::{
        application::tests::EntryRepositoryMock,
        domain::{
            tests::{TEST_DEFAULT_ENTRY_DEFINITION, TEST_DEFAULT_ENTRY_WORD},
            Entry,
        },
    };
    use std::sync::Arc;

    pub fn new_dictionary_application() -> DictionaryApplication<EntryRepositoryMock> {
        let mut entry_repo = EntryRepositoryMock::new();
        DictionaryApplication {
            entry_repo: Arc::new(entry_repo),
        }
    }

    #[tokio::test]
    async fn store_definition_not_fail() {
        let mut entry_repo = EntryRepositoryMock::new();
        let app = new_dictionary_application();
        app.store_definition(TEST_DEFAULT_ENTRY_WORD, TEST_DEFAULT_ENTRY_DEFINITION)
            .await
            .unwrap();
    }
}
