use crosswords::grid::init_grid;

#[test]
fn test_init_grid() {
    let start_grid = init_grid();
    assert_eq!(start_grid.rows(), 7);
    assert_eq!(start_grid.get_line(0), "DOG....");
    assert!(start_grid.check());
}
