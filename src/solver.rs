use crate::grid::{ Grid, Span, Attr};
use crate::library::Library;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Slot<'a> {
    span: &'a Span,
    pattern: String,
}

impl<'a> Slot<'a> {
    pub fn new(span: &'a Span, pattern: String) -> Self {
        Self{span, pattern}
    }

    pub fn to_string(&self) -> String {
        format!("{} '{}'", self.span.to_string(), self.pattern)
    }

    pub fn get_pattern(&self) -> String {
        self.pattern.clone()
    }
}

pub struct Solver<'a> {
    lib: &'a Library,
    pub solutions: Vec<Grid>,
}

impl<'a> Solver<'a> {
    pub fn new(lib: &'a Library) -> Self {
        let solutions: Vec<Grid> = Vec::new();
        Self{lib, solutions}
    }

    pub fn solve(&mut self, grid: &'a Grid) {
        //println!("Solving this grid");
        //grid.print();

        self.loop_( &grid, 0);
    }

    fn loop_(&mut self, grid: &Grid, mut depth: u32) {
        depth += 1;
        /*
        println!("loop - depth = {}", depth);
        if depth > 4 {
           println!("Max depth reached");
            return;
        }
        */


        let mut empty_slots: Vec<Slot> = Vec::new();
        let mut partial_slots: Vec<Slot> = Vec::new();
        let mut full_slots: Vec<Slot> = Vec::new();
        let spans = grid.spans.clone();
        for span in &spans {
            let mut attr = Attr::default();
            let tmp = grid.get_string(&span, &mut attr);
            if attr.is_empty() {
                empty_slots.push(Slot::new(span, tmp));
            } else if attr.is_partial() {
                partial_slots.push(Slot::new(span, tmp));
            } else if attr.is_full() {
                full_slots.push(Slot::new(span, tmp));
            }
        }
        let num_empty = empty_slots.len();
        let num_partial = partial_slots.len();
        let _num_full = full_slots.len();

        //println!("loop exit - number of empty slots : {}", empty_slots.len());
        //println!("loop exit - number of partial slots : {}", partial_slots.len());
        //println!("loop exit - number of full slots : {}", full_slots.len());

        // need to check that all words so far are valid!
        /* 
        for slot in &full_slots {
            let word = grid.get_string(&slot.span, &mut Attr::default());
            //println!("Checking word: {}", word);
            if !self.lib.is_word(&word) {
                //println!("Invalid word found: {}", word);
                return;
            }
        }
        */

        // need to check that all words are unique
        let mut words: HashSet<String> = HashSet::new();
        for slot in &full_slots {
            let word = grid.get_string(&slot.span, &mut Attr::default());
            if words.contains(&word) {
                //println!("Duplicate word found: {}", word);
                return;
            }
            words.insert(word);
        }

        if num_partial == 0 && num_empty == 0 {
            // println!("FOUND A SOLUTION");
            self.solutions.push(grid.clone());
            return;
        }

        if num_partial == 0 {
            return;
        }

        assert!(num_partial > 0);
        // TODO : optimiztion here to pick the slot with the fewest possible words
        let slot = &partial_slots[0];  // pick the first partial slot
        self.commit_slot(slot, &mut grid.clone(), depth); 

    }

    fn commit_slot(&mut self, slot: &Slot, grid: &mut Grid, depth: u32) {
        //println!("Committing slot : {}", slot.to_string());
        //println!("Possible word choices for this slot are: ");
        //println!("{:#?}", self.lib.find_word(&slot.get_pattern()));
        let words = self.lib.find_word(&slot.get_pattern());
        if words.len() > 0 {
            for wrd in &words.words {
                // println!("Attempt to commit '{}'", wrd.word);
                grid.write_string(&slot.span, String::from(wrd.word));
                // For each point in the span, check that if the span in the other direction
                // is also a valid word. a valid word means that it is in the library or that it is a
                // partial word that can be extended to a valid word. If not then we need to backtrack.
                let mut valid = true;
                for i in 0..slot.span.size {
                    let p = slot.span.get_point(i);
                    let other_span = grid.get_span(p, !slot.span.vertical);
                    let mut attr = Attr::default();
                    let s = grid.get_string(&other_span, &mut attr);
                    if attr.is_partial() {
                        let words = self.lib.find_word(&s);
                        if words.len() == 0 {
                            // println!("Invalid word found: {}", s);
                            valid = false;
                            break;
                        }
                    }
                    if attr.is_full() {
                        if !self.lib.is_word(&s) {
                            // println!("Invalid word found: {}", s);
                            valid = false;
                            break;
                        }
                    }
                }
                if !valid {
                    //println!("Word '{}' is not valid", wrd.word);
                    continue;
                }
                //println!("Word '{}' committed", wrd.word);

                //grid.print();
                self.loop_(&grid, depth);
            }    
        }  
    }
}
