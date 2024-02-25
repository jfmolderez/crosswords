use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone)]
pub struct Word {
    pub word: &'static str,
}



#[derive(Debug, Clone)]
pub struct Words {
    pub words: Vec<Word>,
}

impl IntoIterator for Words {
    type Item = Word;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.words.into_iter()
    }
}

impl Words {
    pub fn new(words: Vec<Word>) -> Self {
        Self { words }
    }

    pub fn push(&mut self, word: Word) {
        self.words.push(word);
    }

    pub fn len(&self) -> usize {
        self.words.len()
    }

    pub fn to_string(&self) -> String {
        let mut res = String::from("[ ");
        for i in 0..self.words.len() {
            res.push_str(self.words[i].word);
            if i < self.words.len() - 1 {
                res.push_str(", ");
            }
        }
        res.push_str(" ]");
        res    
    }
}


fn read_words(filename: &str, max_size: usize) -> Words {
    let mut res: Words = Words::new(Vec::new());
    let file = File::open(filename).expect("File not found!");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut str: String = line.trim().to_string();
                if str.len() > max_size {
                    continue;
                }
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
pub struct Library {
    words: Words,
    word_map: HashMap<String, Words>,
    stats: HashMap<usize, usize>,
}

impl Library {
    pub fn new(words: Words) -> Self {
        let mut stats = HashMap::new();
        let mut word_map = HashMap::new();
            
        for w in words.words.iter() {
            let word = w.word;
            let count = stats.entry(word.len()).or_insert(0);
            *count += 1;

            let patterns = Self::create_pattern_hash(&w); 
            for pattern in patterns {
                let ws = word_map.entry(pattern).or_insert(Words::new(Vec::new()));
                ws.push(*w);
            }
        }

        Self { words, word_map, stats } 
    }

    pub fn load(filename: &str, max_size: usize) -> Self {
        let words = read_words(filename, max_size);
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

    pub fn is_word(&self, word: &str) -> bool {
        let mut res = false;
        for w in self.words.words.iter() {
            if w.word.to_string() == word.to_string() {
                res = true;
                break;
            }
        }
        res
    }

    pub fn get_word(&self, i: usize) -> &str {
        self.words.words[i].word
    }

    pub fn find_word(&self, pattern: &str) -> Words {
        match self.word_map.get(pattern) {
            Some(words) => words.clone(),
            None => Words::new(Vec::new()),
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


