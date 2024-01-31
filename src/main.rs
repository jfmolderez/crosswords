use crosswords::grid::{initial_grid, read_grid};

fn main() {
    let start_grid = initial_grid();
    start_grid.print();

    let s0 = start_grid.get_line(0);
    println!("{}", s0);

    let start_grid = read_grid("./data/initial.txt");
    start_grid.print();

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
