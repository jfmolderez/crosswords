use crate::utils::read_words;

#[derive(Debug)]
pub struct Grid {
    grid: Vec<String>,
}

impl Grid {
    pub fn new(grid: Vec<String>) -> Self {
        let gr = Self { grid };
        gr.check();
        gr
    }

    pub fn size(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    pub fn rows(&self) -> usize {
        self.grid.len()
    }

    pub fn cols(&self) -> usize {
        self.grid[0].len()
    }

    pub fn get_line(&self, row: usize) -> &str {
        &self.grid[row]
    }

    fn check(&self) -> bool {
        let cols = self.grid[0].len();

        for row in &self.grid {
            if row.len() != cols {
                return false;
            }
        }
        true
    }

    pub fn print(&self) {
        println!("Grid size = {} x {}", self.rows(), self.cols());
        for row in &self.grid {
            println!("\t{}", row);
        }
    }
}

/// Returns a grid object containing a vector of strings 
/// representing the initial grid.
/// # Example:
/// ```
/// use crosswords::grid::initial_grid;
/// let grid = initial_grid();
/// assert_eq!(grid.get_line(0), "DOG....");
/// ```
pub fn initial_grid() -> Grid {
    let puzzle = [
        String::from("DOG...."),
        String::from("---...."),
        String::from("----..."),
        String::from("-------"),
        String::from("...----"),
        String::from("....---"),
        String::from("....CAT"),
    ];
    Grid::new(puzzle.to_vec())
}

/// Returns a grid object containing a vector of strings
/// read from a file.
/// # Example:
/// ```
/// use crosswords::grid::read_grid;
/// let grid = read_grid("./data/initial.txt");
/// assert_eq!(grid.get_line(0), "DOG....");
/// ```
pub fn read_grid(filename: &str) -> Grid {
    let puzzle: Vec<String> = read_words(filename);
    Grid::new(puzzle)
}
