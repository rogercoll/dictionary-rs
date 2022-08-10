use crate::entry::{application::EntryRepository, domain::Entry};
use rand::Rng;
use std::error::Error;
use std::sync::Arc;

// can contain additional features as user access permissions, random entry return, caching, etc
pub struct DictionaryApplication {
    pub entry_repo: Arc<dyn EntryRepository>,
}

impl DictionaryApplication {
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
    pub async fn get_all_definitions(&self) -> Result<Vec<Entry>, Box<dyn Error>> {
        Ok(self.entry_repo.get_all().await?)
    }
    pub async fn get_random_definition(&self) -> Result<Entry, Box<dyn Error>> {
        let mut entries = self.entry_repo.get_all().await?;
        Ok(entries.remove(rand::thread_rng().gen_range(0..entries.len())))
    }
}

#[cfg(test)]
pub mod tests {
    use super::DictionaryApplication;
    use crate::entry::{
        application::tests::EntryRepositoryMock,
        domain::tests::{TEST_DEFAULT_ENTRY_DEFINITION, TEST_DEFAULT_ENTRY_WORD},
    };
    use std::sync::Arc;

    pub fn new_dictionary_application() -> DictionaryApplication {
        let entry_repo = EntryRepositoryMock::new();
        DictionaryApplication {
            entry_repo: Arc::new(entry_repo),
        }
    }

    #[tokio::test]
    async fn store_definition_not_fail() {
        let app = new_dictionary_application();
        app.store_definition(TEST_DEFAULT_ENTRY_WORD, TEST_DEFAULT_ENTRY_DEFINITION)
            .await
            .unwrap();
    }
}
