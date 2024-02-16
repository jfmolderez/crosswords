use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Word {
    word: &'static str,
}

fn read_words(filename: &str) -> Vec<Word> {
    let mut res: Vec<Word> = Vec::new();
    let file = File::open(filename).expect("File not found!");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut str: String = line.trim().to_string();
                str.make_ascii_uppercase();
                let str: &'static str = Box::leak(str.into_boxed_str());
                res.push(Word { word: str })
            }
            Err(e) => println!("Error reading line: {}", e),
        }
    }
    res
}




#[derive(Debug)]
pub struct Library<'a> {
    words: Vec<Word>,
    word_map: HashMap<String, Vec<&'a str>>,
    stats: HashMap<usize, usize>,
}

impl<'a> Library<'a> {
    pub fn new(words: Vec<Word>) -> Self {
        let mut stats = HashMap::new();
        let mut word_map = HashMap::new();
        
        for w in words.clone() {
            let word = w.word;
            let count = stats.entry(word.len()).or_insert(0);
            *count += 1;

            let patterns = Self::create_pattern_hash(&w); 
            for pattern in patterns {
                let ws = word_map.entry(pattern).or_insert(Vec::new());
                ws.push(word);
            }
        }

        Self { words: words , word_map, stats } 
    }

    pub fn load(filename: &str) -> Self {
        let words = read_words(filename);
        Self::new(words)
    }

    pub fn size(&self) -> usize {
        self.words.len()
    }

    fn create_pattern_hash(w: &Word) -> Vec<String> {
        let mut res = Vec::new();

        let len = w.word.len();
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
        res.push(pattern);
        }
        res
    }

    pub fn is_word(&self, word: String) -> bool {
        let ws: Vec<String> = self.words.iter().map(|w| w.word.to_string()).collect();
        ws.contains(&word)
    }

    pub fn get_word(&self, i: usize) -> &str {
        self.words[i].word
    }

    pub fn find_word(&self, pattern: &str) -> Vec<&str> {
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


