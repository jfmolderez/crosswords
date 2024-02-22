use crosswords::grid::{read_grid, Grid, Span, Attr};
use crosswords::library::Library;


#[derive(Debug, Clone)]
pub struct Slot<'a> {
    pub span: &'a Span,
}

impl<'a> Slot<'a> {
    pub fn new(span: &'a Span) -> Self {
        Self{span}
    }
}

pub struct Solver<'a> {
    grid : &'a Grid,
}

impl<'a> Solver<'a> {
    pub fn new(grid: &'a Grid) -> Self {
        Self{grid}
    }

    pub fn solve(&self) {
        println!("Solving this grid");
        self.grid.print();
        self.loop_();

    }

    fn loop_(&self) {
        println!("loop");
        let mut empty_slots: Vec<Slot> = Vec::new();
        let mut partial_slots: Vec<Slot> = Vec::new();
        let mut full_slots: Vec<Slot> = Vec::new();

        for span in &self.grid.spans {
            let mut attr = Attr::default();
            let tmp = self.grid.get_string(&span, &mut attr);
            if attr.is_empty() {
                empty_slots.push(Slot::new(span));
            } else if attr.is_partial() {
                partial_slots.push(Slot::new(span));
            } else if attr.is_full() {
                full_slots.push(Slot::new(span));
            }
        }
        let num_empty = empty_slots.len();
        let num_partial = partial_slots.len();
        let num_full = full_slots.len();

        if num_partial == 0 && num_empty == 0 {
            println!("SOLUTION")
        }
        assert!(num_partial > 0);
        self.commit_slot(partial_slots[0].clone());
        //println!("loop exit - number of empty slots : {}", empty_slots.len());
        //println!("loop exit - number of partial slots : {}", partial_slots.len());
        //println!("loop exit - number of full slots : {}", full_slots.len());
    }

    fn commit_slot(&self, slot: Slot) {
        println!("First partial slot : {}", slot.span.to_string());
    }
}

fn main() {
    let grid = read_grid("./data/initial.txt");
    grid.print_spans();

    let lib: Library = Library::load("./data/lib/top_12000.txt", grid.size());
    println!("Size of the library = {}", lib.size());

    let solver = Solver::new(&grid);
    solver.solve();


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
