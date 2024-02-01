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
        let mut words = read_words(filename);
        for word in &mut words {
            word.make_ascii_uppercase();
        }
        Self::new(words)
    }

    pub fn size(&self) -> usize {
        self.words.len()
    }

    pub fn get_word(&self, i: usize) -> String {
        assert!(i < self.size());
        self.words[i].clone()
    }

    pub fn is_word(&self, word: &str) -> bool {
        self.words.contains(&word.to_string())
    }

    pub fn print_stats(&self) {
        let mut lens: Vec<usize> = self.stats.keys().cloned().collect();
        lens.sort();
        for len in lens {
            println!("[{}] {}", len, self.stats[&len]);
        }
    }
 
}