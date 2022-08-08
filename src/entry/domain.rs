pub struct Entry {
    pub(super) word: String,
    pub(super) definition: String,
}

impl Entry {
    pub fn new(word: &str, definition: &str) -> Entry {
        Entry {
            word: word.to_string(),
            definition: definition.to_string(),
        }
    }
    pub fn get_definition(&self) -> &str {
        &self.definition
    }
    pub fn get_word(&self) -> &str {
        &self.word
    }
}

#[cfg(test)]
pub mod tests {
    use super::Entry;

    pub const TEST_DEFAULT_ENTRY_WORD: &str = "hello";
    pub const TEST_DEFAULT_ENTRY_DEFINITION: &str = "hola";

    pub fn new_entry() -> Entry {
        Entry {
            word: TEST_DEFAULT_ENTRY_WORD.to_string(),
            definition: TEST_DEFAULT_ENTRY_DEFINITION.to_string(),
        }
    }
}
