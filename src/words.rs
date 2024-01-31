use crate::utils::read_words;

#[derive(Debug)]
pub struct Library {
    words: Vec<String>,
}

impl Library {
    pub fn new(words: Vec<String>) -> Self {
        Self { words }
    }

    pub fn load(filename: &str) -> Self {
        let words = read_words(filename);
        Self::new(words)
    }

    pub fn size(&self) -> usize {
        self.words.len()
    }
 
}