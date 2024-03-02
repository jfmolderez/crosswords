use criterion::{criterion_group, criterion_main, Criterion, black_box};
use crosswords::grid::read_grid;

fn simple(c: &mut Criterion) {
    
    
    c.bench_function("simple", |b| {
        b.iter( || {
            let mut grid = read_grid("./data/step.txt");
            let lib = crosswords::library::Library::load("./data/lib/top_12000.txt", grid.size());
            let mut solver = black_box(crosswords::solver::Solver::new(&lib));
            solver.solve(&mut grid);
        })
    });

}

criterion_group!(benches, simple);
criterion_main!(benches);
