
use crosswords::initial_grid::init_grid ;

fn main() {
    let grid = init_grid();

    let result = 1 + 1;
    println!("{}", result);

    let hello = "hello".to_string();
    println!("Length of {} = {}", hello, hello.len());

    println!("{:?}", grid);

    let mut s0 = grid[0].clone();
    s0.push_str(" IS HAPPY!");
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
}
