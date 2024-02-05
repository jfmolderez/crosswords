use std::collections::HashMap;
use crate::utils::read_words;


#[derive(Debug)]
pub struct Library<'a>{
    words: Vec<String>,
    word_map: HashMap<&'a str, Vec<&'a str>>,
    stats: HashMap<usize, usize>,
}

impl<'a> Library<'a> {
    pub fn new(words: Vec<String>) -> Self {
        let words = words.clone();
        let mut stats = HashMap::new();
        let mut word_map = HashMap::new();
        
        for word in words {
            let len = word.len();
            let count = stats.entry(len).or_insert(0);
            *count += 1;

            let patterns = Self::create_pattern_hash(&word);
            for pattern in patterns {
                let words = word_map.entry(pattern.as_str()).or_insert(Vec::new());
                words.push(word.as_str());
            }
        }

        Self { words , word_map, stats }
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

    fn create_pattern_hash(w: &String) -> Vec<String> {
        let mut res = Vec::new();

        let len = w.len();
        let num_patterns = 1 << len;
        for i in 0..num_patterns {
            let mut pattern = String::new();
            for j in 0..len {
                if i & (1 << j) != 0 {
                    pattern.push(w.chars().nth(j).unwrap());
                } else {
                    pattern.push('-');
                }
            }
        res.push(pattern);
        }
        res
    }

    pub fn is_word(&self, word: String) -> bool {
        self.words.contains(&word)
    }

    pub fn get_word(&self, i: usize) -> String {
        self.words[i].to_string()
    }

    pub fn find_word(&self, pattern: &str) -> Vec<&'a str> {
        match self.word_map.get(pattern) {
            Some(words) => words.clone(),
            None => Vec::new(),
        }
    }

    pub fn print_stats(&self) {
        let mut lens: Vec<usize> = self.stats.keys().cloned().collect();
        lens.sort();
        for len in lens {
            println!("[{}] {}", len, self.stats[&len]);
        }
    }
 
}


