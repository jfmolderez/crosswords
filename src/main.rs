

fn main() {
    let puzzle = [
        String::from("DOG...."),
        String::from("---...."),
        String::from("----..."),
        String::from("-------"),
        String::from("...----"),
        String::from("....---"),
        String::from("....CAT"),
    ];

    let result = 1 + 1;
    println!("{}", result);

    let hello = "hello".to_string();
    println!("Length of {} = {}", hello, hello.len())  ; 

    println!("{:?}", puzzle);

    let mut s0 = puzzle[0].clone();
    s0.push_str(" IS HAPPY!");
    println!("{}", s0);

    println!("{:?}", s0.find("HAPPY"));
    println!("{:?}", s0.find("HAPPY").unwrap());

    println!("{:?}", s0.chars());
    println!("{:?}", s0.chars().nth(0));
    println!("{:?}", s0.chars().nth(0).unwrap_or('ı'));
}
