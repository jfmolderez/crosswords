use crosswords::grid::read_grid;
use crosswords::library::Library;
use crosswords::solver::Solver;

#[test]
fn test_solver_trivial() {
    let mut grid = read_grid("./data/empty.txt");
    let lib: Library = Library::load("./data/lib/top_12000.txt", grid.size());

    let mut solver = Solver::new(&lib);
    solver.solve(&mut grid);

    assert_eq!(solver.solutions.len(), 0);
}

#[test]
fn test_solver_duplicates() {
    let mut grid = read_grid("./data/duplicates.txt");
    let lib: Library = Library::load("./data/lib/top_12000.txt", grid.size());

    let mut solver = Solver::new(&lib);
    solver.solve(&mut grid);

    solver.solutions[0].print();
    assert_eq!(solver.solutions.len(), 388);
    
}

#[test]
fn test_solver_mary() {
    let mut grid = read_grid("./data/mary.txt");
    let lib: Library = Library::load("./data/lib/top_12000.txt", grid.size());

    let mut solver = Solver::new(&lib);
    solver.solve(&mut grid);

    solver.solutions[0].print();
    assert_eq!(solver.solutions.len(), 0);   
}

#[test]
fn test_solver_step() {
    let mut grid = read_grid("./data/step.txt");
    let lib: Library = Library::load("./data/lib/top_12000.txt", grid.size());

    let mut solver = Solver::new(&lib);
    solver.solve(&mut grid);

    solver.solutions[0].print();
    assert_eq!(solver.solutions.len(), 129);   
}


