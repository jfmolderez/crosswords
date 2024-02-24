use crosswords::grid::{read_grid, Grid, Span, Attr};
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
}

impl<'a> Solver<'a> {
    pub fn new(lib: &'a Library) -> Self {
        Self{lib}
    }

    pub fn solve(&mut self, grid: &Grid) {
        println!("Solving this grid");
        grid.print();
        self.loop_(&grid);

    }

    fn loop_(&mut self, grid: &Grid) {
        println!("loop");
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
        let num_full = full_slots.len();

        if num_partial == 0 && num_empty == 0 {
            println!("SOLUTION")
        }
        assert!(num_partial > 0);
        let slot = &partial_slots[0];
        self.commit_slot(slot, &mut grid.clone());
        //println!("loop exit - number of empty slots : {}", empty_slots.len());
        //println!("loop exit - number of partial slots : {}", partial_slots.len());
        //println!("loop exit - number of full slots : {}", full_slots.len());
    }

    fn commit_slot(&mut self, slot: &Slot, grid: &mut Grid) {
        println!("Committing slot : {}", slot.to_string());
        // println!("Possible word choices for this slot are: ");
        // println!("{:#?}", self.lib.find_word(&slot.get_pattern()));
        let words = self.lib.find_word(&slot.get_pattern());
        if words.len() > 0 {
            // println!("{}", words.to_string());
            grid.write_string(&slot.span, String::from(words.words[0].word));
            grid.print();
        } else {
            println!("No words found for this pattern");
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
