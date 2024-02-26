use core::panic;

use crosswords::grid::{self, read_grid, Attr, Grid, Span};
use crosswords::library::Library;


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
        println!("Solving this grid");
        grid.print();

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

        if num_partial == 0 && num_empty == 0 {
            println!("FOUND A SOLUTION");
            self.solutions.push(grid.clone());
            grid.print();

            // return;
        }
        assert!(num_partial > 0);
        let slot = &partial_slots[0];
        self.commit_slot(slot, &mut grid.clone(), depth); 

    }

    fn commit_slot(&mut self, slot: &Slot, grid: &mut Grid, depth: u32) {
        println!("Committing slot : {}", slot.to_string());
        println!("Possible word choices for this slot are: ");
        println!("{:#?}", self.lib.find_word(&slot.get_pattern()));
        let words = self.lib.find_word(&slot.get_pattern());
        if words.len() > 0 {
            for wrd in &words.words {
                println!("Attempt to commit '{}'", wrd.word);
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
                    println!("Word '{}' is not valid", wrd.word);
                    continue;
                }
                println!("Word '{}' committed", wrd.word);

                grid.print();
                self.loop_(&grid, depth);
            }    
        }  
    }
}

fn main() {
    let mut grid = read_grid("./data/initial.txt");
    grid.print_spans();

    let lib: Library = Library::load("./data/lib/top_12000.txt", grid.size());
    println!("Size of the library = {}", lib.size());

    let mut solver = Solver::new(&lib);
    solver.solve(&mut grid);

    for grid in &solver.solutions {
        grid.print();
    }


}
/* 
    let start_grid = initial_grid();
    start_grid.print();

    let s0 = start_grid.get_line(0);
    println!("{}", s0);

    let start_grid = read_grid("./data/initial.txt");
    start_grid.print();
 
    let lib = Library::load("./data/lib/top_12000.txt", start_grid.size());
    println!("Size of the library = {}", lib.size());

    lib.print_stats();

    let word = lib.get_word(876);
    println!("Word 876 = {}", word);

    let dog_result = lib.find_word("D-G");
    println!("D_G result = {:?}", dog_result);

    let p1 = Point::new(3, 4);
    println!("Point 1 = {:?}", p1);

    let s1 = Span::new(p1, 5, true);
    println!("Span 1 = {:?}", s1);

    let p = Point::new(0, 0);
    println!("{:?} : {}", p, start_grid.is_block(p.clone()));
    start_grid.print_spans();

    println!();
    let test_grid = read_grid("./data/test.txt");
    test_grid.print_spans();
*/


    

   

    // process_lib()

    /* 
    let mut s0 = String::from("DOG");
    s0.to_string().push_str(" IS HAPPY!");
    println!("{}", s0);

    println!("{:?}", s0.find("HAPPY"));
    println!("{:?}", s0.find("HAPPY").unwrap());

    println!("{:?}", s0.chars());
    println!("{:?}", s0.chars().nth(0));
    println!("{:?}", s0.chars().nth(5).unwrap_or('ı'));

    let chars: Vec<char> = s0.chars().collect();
    println!("{:?}", chars);
    println!("{:?}", chars[0]);
    println!("Length of vector chars = {}", chars.len());

    let hello = "hello".to_string();
    println!("Length of {} = {}", hello, hello.len());
    */



/* 
use std::fs::File;
use std::io::{Write, BufRead, BufReader};

fn process_lib() {
    
    let input_file = File::open("./data/lib/20k.txt").expect("File not found!");
    let mut output_file = File::create("./data/lib/top_12000.txt").expect("File not found!");

    let reader = BufReader::new(input_file);
    for (i, line) in reader.lines().enumerate() {
        if i == 11999 {
            match line {
                Ok(line) => {
                    write!(output_file, "{}", line).expect("Unable to write data");   
                },
                Err(e) => println!("Error rezading line: {}", e),
            }            
            break;
        }
        match line {
            Ok(line) => {
                writeln!(output_file, "{}", line).expect("Unable to write data");   
            },
            Err(e) => println!("Error rezading line: {}", e),
        }
    }
}
*/
