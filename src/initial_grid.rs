/// Returns a vector of strings representing the initial grid
/// # Example:
/// ```
/// use crosswords::initial_grid::init_grid;
/// let grid = init_grid();
/// assert_eq!(grid[0], "DOG....");
/// ```
pub fn init_grid() -> Vec<String> {
    let puzzle = [
        String::from("DOG...."),
        String::from("---...."),
        String::from("----..."),
        String::from("-------"),
        String::from("...----"),
        String::from("....---"),
        String::from("....CAT"),
    ];
    puzzle.to_vec()
}