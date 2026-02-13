mod seq;
mod parallel;
mod visualize;
mod io;

use std::time::Instant;

fn main() {
    let rows = 50;
    let cols = 50;
    let iterations = 50;

    // ---------------------------- SEQUENTIAL --------------------------
    println!("Running sequential version...");
    let mut grid = seq::generate_random_grid(rows, cols);

    let start = Instant::now();
    for _ in 0..iterations {
        grid = seq::next_generation(&grid);
    }
    let duration = start.elapsed();
    println!("Sequential finished in: {:?}", duration);

    println!("Saving frames for visualization...");
    let mut grid = seq::generate_random_grid(rows, cols);

    for i in 0..50 {
        grid = seq::next_generation(&grid);
        visualize::draw_grid(&grid, i);
    }


    // ---------------------------- PARALLEL ----------------------------
    println!("Running sequential version...");
    let mut grid = seq::generate_random_grid(rows, cols);

    let start = Instant::now();
    for i in 0..iterations {
        grid = seq::next_generation(&grid);
        io::save_grid(&grid, i); // <-- upisujemo iteraciju u states/
    }
    let duration = start.elapsed();
    println!("Sequential finished in: {:?}", duration);

    // Vizualizacija čita iz states/
    println!("Generating frames from saved states...");
    for i in 0..iterations {
        visualize::draw_from_file(&format!("states/state_{:04}.txt", i), i);
    }

}
