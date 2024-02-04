use crosswords::library::Library;

#[test]
fn test_size() {
    let library = Library::load("./data/lib/top_12000.txt");
    assert_eq!(library.size(), 12000);
}


#[test]
fn test_get_word() {
    let library = Library::load("./data/lib/top_12000.txt");
    assert_eq!(library.get_word(876), "THANKS".to_string());
}


#[test]
fn test_is_word() {
    let library = Library::load("./data/lib/top_12000.txt");
    assert!(library.is_word("THANKS"));
    assert!(library.is_word("THANK"));
    assert!(!library.is_word("THANKKK"))
}


    