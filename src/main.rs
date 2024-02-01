use crosswords::grid::{initial_grid, read_grid};
use crosswords::words::Library;

fn main() {
    let start_grid = initial_grid();
    start_grid.print();

    let s0 = start_grid.get_line(0);
    println!("{}", s0);

    let start_grid = read_grid("./data/initial.txt");
    start_grid.print();

    let lib = Library::load("./data/lib/top_12000.txt");
    println!("Size of the library = {}", lib.size());

    lib.print_stats();

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

}

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
