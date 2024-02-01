use crosswords::words::Library;

#[test]
fn test_size() {
    let library = Library::load("./data/lib/top_12000.txt");
    assert_eq!(library.size(), 12000);
}

#[test]
fn test_get_word() {
    let library = Library::load("./data/lib/top_12000.txt");
    assert_eq!(library.get_word(876), "thanks".to_string());
}
    