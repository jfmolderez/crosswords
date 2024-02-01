use std::collections::HashMap;
use crate::utils::read_words;

#[derive(Debug)]
pub struct Library {
    words: Vec<String>,
    stats: HashMap<usize, usize>,
}

impl Library {
    pub fn new(words: Vec<String>) -> Self {
        let mut stats = HashMap::new();
        for word in &words {
            let len = word.len();
            let count = stats.entry(len).or_insert(0);
            *count += 1;
        }
        Self { words, stats }
    }

    pub fn load(filename: &str) -> Self {
        let words = read_words(filename);
        Self::new(words)
    }

    pub fn size(&self) -> usize {
        self.words.len()
    }

    pub fn get_word(&self, i: usize) -> String {
        assert!(i < self.size());
        self.words[i].clone()
    }
 
}