use crosswords::grid::{initial_grid, read_grid};

#[test]
fn test_initial_grid() {
    let start_grid = initial_grid();
    assert_eq!(start_grid.rows(), 7);
    assert_eq!(start_grid.get_line(0), "DOG....");
}

#[test]
fn test_read_grid() {
    let start_grid = read_grid("./data/initial.txt");
    assert_eq!(start_grid.rows(), 7);
    assert_eq!(start_grid.get_line(0), "DOG....");
}

