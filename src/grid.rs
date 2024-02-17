use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Point {
    row: usize,
    col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug)]
pub struct Span {
    start: Point,
    size: usize,
    vertical: bool, // false = horizontal, true = vertical
}

impl Span {
    pub fn new(start: Point, size: usize, vertical: bool) -> Self {
        Self { start, size, vertical }
    }
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<String>,
    spans: Vec<Span>,
}

impl Grid {
    pub fn new(grid: Vec<String>) -> Self {
        let spans = get_spans(&grid);
        let gr = Self { grid, spans };
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

    fn in_bounds(&self, p: &Point) -> bool {
        p.row < self.rows() && p.col < self.cols()
    }

    pub fn next(&self, p: &mut Point, vertical: bool) -> bool {
        if vertical {
           p.row += 1;
           if p.row >= self.rows() {
               p.row = 0;
               p.col += 1;
           }
        } else {
            p.col += 1;
            if p.col >= self.cols() {
                p.col = 0;
                p.row += 1;
            }
        }
        self.in_bounds(p)
    }

    pub fn is_block(&self, p: &Point) -> bool {
        self.grid[p.row].chars().nth(p.col).unwrap() == '.'
    }

    fn is_blank(&self, p: &Point) -> bool {
        self.grid[p.row].chars().nth(p.col).unwrap() == '-'
    }

    fn is_letter(&self, p: &Point) -> bool {
        self.grid[p.row].chars().nth(p.col).unwrap().is_alphabetic()
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

fn get_spans(grid: &Vec<String>) -> Vec<Span> {
    let mut res: Vec<Span> = Vec::new();

    res
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
    let mut res: Vec<String> = Vec::new();
    let file = File::open(filename).expect("File not found!");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => res.push(line.trim().to_string()),
            Err(e) => println!("Error reading line: {}", e),
        }
    }
    Grid::new(res)
}



