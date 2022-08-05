use super::domain::Entry;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait EntryRepository: Sync + Send {
    async fn find_by_word(&self, word: &str) -> Result<Entry, Box<dyn Error>>;
    async fn get_all(&self) -> Result<Vec<Entry>, Box<dyn Error>>;
    async fn save(&self, entry: &Entry) -> Result<(), Box<dyn Error>>;
    async fn delete_by_word(&self, word: &str) -> Result<(), Box<dyn Error>>;
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
        pub fn_get_all:
            Option<fn(this: &EntryRepositoryMock) -> Result<Vec<Entry>, Box<dyn Error>>>,
        pub fn_save:
            Option<fn(this: &EntryRepositoryMock, entry: &Entry) -> Result<(), Box<dyn Error>>>,
        pub fn_delete_by_word:
            Option<fn(this: &EntryRepositoryMock, word: &str) -> Result<(), Box<dyn Error>>>,
    }

    impl EntryRepositoryMock {
        pub fn new() -> Self {
            EntryRepositoryMock {
                fn_find_by_word: None,
                fn_get_all: None,
                fn_save: None,
                fn_delete_by_word: None,
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
        async fn get_all(&self) -> Result<Vec<Entry>, Box<dyn Error>> {
            if let Some(f) = self.fn_get_all {
                return f(self);
            }
            Ok(Vec::new())
        }
        async fn save(&self, entry: &Entry) -> Result<(), Box<dyn Error>> {
            if let Some(f) = self.fn_save {
                return f(self, entry);
            }
            Ok(())
        }
        async fn delete_by_word(&self, word: &str) -> Result<(), Box<dyn Error>> {
            if let Some(f) = self.fn_delete_by_word {
                return f(self, word);
            }
            Ok(())
        }
    }
}
