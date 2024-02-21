use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp;

#[derive(Debug, Clone)]
pub struct Point {
    row: usize,
    col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    pub fn to_string(&self) -> String {
        format!("({}, {})", self.row, self.col)
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

    pub fn to_string(&self) -> String {
        format!("[Start: {}, size: {}, vertical: {}]", self.start.to_string(), self.size, self.vertical)
    }

}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<String>,
    spans: Vec<Span>,
}

impl Grid {
    pub fn new(grid: Vec<String>) -> Self {
        let spans = Vec::new();
        let mut gr = Self { grid, spans };
        gr.check();

        gr.fill_spans();        
        gr
    }

    pub fn size(&self) -> usize {
        cmp::max(self.grid.len(), self.grid[0].len())
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

    fn next_stop_at_wrap(&self, p: Point, vertical: bool) -> (Point, bool) {
        let mut wrap = false;
        let mut row = p.row;
        let mut col = p.col;
        if vertical {
           row += 1;
           if row >= self.rows() {
               row = 0;
               col += 1;
               wrap = true;
           }
        } else {
            col += 1;
            if col >= self.cols() {
                col = 0;
                row += 1;
                wrap = true;
            }
        }
        (Point::new(row, col), wrap)
    }

    fn in_bounds(&self, p: Point) -> bool {
        p.row < self.rows() && p.col < self.cols()
    }

    fn is_block(&self, p: Point) -> bool {
        self.grid[p.row].chars().nth(p.col).unwrap() == '.'
    }

    fn is_blank(&self, p: Point) -> bool {
        self.grid[p.row].chars().nth(p.col).unwrap() == '-'
    }

    fn is_letter(&self, p: Point) -> bool {
        self.grid[p.row].chars().nth(p.col).unwrap().is_alphabetic()
    }

    fn get_string(&self, span: &Span) -> String {
        ""
    }

    fn fill_spans(&mut self) {
        assert!(self.spans.is_empty());
        self.fill_spans_dir(false);
        self.fill_spans_dir(true);
    }

    fn fill_spans_dir(&mut self, vertical: bool) {
        let mut p = Point::new(0, 0);
       
        while self.in_bounds(p.clone()) {
        
            while self.in_bounds(p.clone()) && self.is_block(p.clone()) {
                (p, _) = self.next_stop_at_wrap(p, vertical);
            }

            if !self.in_bounds(p.clone()) {
                return;
            }
            let start = p.clone();
            let mut size = 0;
            let mut wrap = false;
            loop {              
                if !self.in_bounds(p.clone()) || self.is_block(p.clone()) || wrap {
                    break;
                }
                size += 1;
                (p, wrap) = self.next_stop_at_wrap(p, vertical);
            }
            self.spans.push(Span::new(start, size, vertical));
        }
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

    pub fn print_spans(&self) {
        for span in &self.spans {
            println!("{}", span.to_string());
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



