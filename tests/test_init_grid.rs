use crosswords::initial_grid::init_grid;

#[test]
fn test_init_grid() {
    let grid = init_grid();
    assert_eq!(grid.len(), 7);
    assert_eq!(grid[0],"DOG....");

    let rows = grid.len();
    let cols = grid[0].chars().count();

    for i in 0..rows {
        assert_eq!(grid[i].chars().count(), cols);
    }
}
