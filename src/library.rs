use std::collections::{HashMap, HashSet};
use crate::utils::read_words;

#[derive(Debug)] 
struct Word {
    word: String,
}

impl Word {
    pub fn new(word: String) -> Self {
        Self { word }
    }

    pub fn len(&self) -> usize {
        self.word.len()
    }

    pub fn to_string(&self) -> String {
        self.word.clone()
    }
}

#[derive(Debug)]
pub struct Library {
    words: Vec<Word>,
    stats: HashMap<usize, usize>,
}

impl Library {
    pub fn new(ws: Vec<String>) -> Self {
        let mut words: Vec<Word> = Vec::new();
        let mut stats = HashMap::new();
        for word in ws {
            words.push(Word::new(word.clone()));
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

    pub fn is_word(&self, word: Word) -> bool {
        let words_vec: Vec<String> = self.words.iter().map(|w| w.to_string()).collect();
        words_vec.contains(&word.to_string())
    }

    pub fn get_word(&self, i: usize) -> String {
        self.words[i].to_string()
    }

    pub fn print_stats(&self) {
        let mut lens: Vec<usize> = self.stats.keys().cloned().collect();
        lens.sort();
        for len in lens {
            println!("[{}] {}", len, self.stats[&len]);
        }
    }
 
}


fn create_pattern_hash(w: &Word) {
    let len = w.len();
    let num_patterns = 1 << len;
    for i in 0..num_patterns {
        let mut pattern = String::new();
        for j in 0..len {
            if i & (1 << j) != 0 {
                pattern.push(w.word.chars().nth(j).unwrap());
            } else {
                pattern.push('-');
            }
        }
    println!("{}", pattern);
    }
}