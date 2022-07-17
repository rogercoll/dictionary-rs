use super::domain::Entry;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait EntryRepository {
    async fn find_by_word(&self, word: &str) -> Result<Entry, Box<dyn Error>>;
    async fn save(&self, entry: &Entry) -> Result<(), Box<dyn Error>>;
    async fn delete(&self, entry: &Entry) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
pub mod tests {
    use super::EntryRepository;
    use crate::entry::domain::{tests::new_entry, Entry};
    use async_trait::async_trait;
    use std::error::Error;

    pub struct EntryRepositoryMock {
        pub fn_find_by_word:
            Option<fn(this: &EntryRepositoryMock, word: &str) -> Result<Entry, Box<dyn Error>>>,
        pub fn_save:
            Option<fn(this: &EntryRepositoryMock, entry: &Entry) -> Result<(), Box<dyn Error>>>,
        pub fn_delete:
            Option<fn(this: &EntryRepositoryMock, entry: &Entry) -> Result<(), Box<dyn Error>>>,
    }

    impl EntryRepositoryMock {
        pub fn new() -> Self {
            EntryRepositoryMock {
                fn_find_by_word: None,
                fn_save: None,
                fn_delete: None,
            }
        }
    }

    #[async_trait]
    impl EntryRepository for EntryRepositoryMock {
        async fn find_by_word(&self, word: &str) -> Result<Entry, Box<dyn Error>> {
            if let Some(f) = self.fn_find_by_word {
                return f(self, word);
            }
            Ok(new_entry())
        }
        async fn save(&self, entry: &Entry) -> Result<(), Box<dyn Error>> {
            if let Some(f) = self.fn_save {
                return f(self, entry);
            }
            Ok(())
        }
        async fn delete(&self, entry: &Entry) -> Result<(), Box<dyn Error>> {
            if let Some(f) = self.fn_delete {
                return f(self, entry);
            }
            Ok(())
        }
    }
}
