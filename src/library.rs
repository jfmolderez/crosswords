use std::collections::{HashMap, HashSet};
use crate::utils::read_words;

#[derive(Debug)]
pub struct Library {
    words: HashSet<String>,
    stats: HashMap<usize, usize>,
}

impl Library {
    pub fn new(ws: Vec<String>) -> Self {
        let mut words = HashSet::new();
        let mut stats = HashMap::new();
        for word in &ws {
            words.insert(word.clone());
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

    pub fn is_word(&self, word: &str) -> bool {
        self.words.contains(&word.to_string())
    }

    pub fn find_words(&self, pattern: &str) -> Vec<String> {
        let mut res : Vec<String> = Vec::new();
        let length = pattern.len();
        for word in &self.words {
            let mut tmp = word.clone();
            if tmp.len() == length {
                for i in 0..length {
                    if pattern.chars().nth(i).unwrap() == '-' {
                        tmp.replace_range(i..i+1, "-");
                    }
                }
                if tmp == pattern {
                    res.push(word.clone());
                }
            }
        }
        res
    }

    pub fn print_stats(&self) {
        let mut lens: Vec<usize> = self.stats.keys().cloned().collect();
        lens.sort();
        for len in lens {
            println!("[{}] {}", len, self.stats[&len]);
        }
    }
 
}