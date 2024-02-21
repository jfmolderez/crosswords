use crosswords::library::Library;

#[test]
fn test_size() {
    let library = Library::load("./data/lib/top_12000.txt",7);
    assert_eq!(library.size(), 7770);
}


#[test]
fn test_get_word() {
    let library = Library::load("./data/lib/top_12000.txt", 7);
    assert_eq!(library.get_word(876), "LINES".to_string());
}


#[test]
fn test_is_word() {
    let library = Library::load("./data/lib/top_12000.txt", 7);
    assert!(library.is_word("THANKS".to_string()));
    assert!(library.is_word("THANK".to_string()));
    assert!(!library.is_word("THANKKK".to_string()))
}


    