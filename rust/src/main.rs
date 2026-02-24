mod io;
mod parallel;
mod scaling;
mod seq;
mod visualize;

use std::time::Instant;
use std::fs;

fn parse_arg_usize(args: &[String], key: &str, default: usize) -> usize {
    args.windows(2)
        .find(|w| w[0] == key)
        .and_then(|w| w[1].parse::<usize>().ok())
        .unwrap_or(default)
}

fn parse_arg_string(args: &[String], key: &str, default: &str) -> String {
    args.windows(2)
        .find(|w| w[0] == key)
        .map(|w| w[1].clone())
        .unwrap_or_else(|| default.to_string())
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|a| a == flag)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let rows = parse_arg_usize(&args, "--rows", 50);
    let cols = parse_arg_usize(&args, "--cols", 50);
    let iterations = parse_arg_usize(&args, "--iters", 50);
    let threads = parse_arg_usize(&args, "--threads", 4);
    let mode = parse_arg_string(&args, "--mode", "seq");
    let repeats = parse_arg_usize(&args, "--repeats", 5);

    let benchmark = parse_arg_string(&args, "--benchmark", "none");

    let demo = has_flag(&args, "--demo");
    let full_demo = has_flag(&args, "--full-demo");

    let states_dir = "visualization/states";
    let frames_dir = "visualization/frames";

    // =========================
    // BENCHMARK MODES
    // =========================
    if benchmark == "strong" {
        scaling::run_strong_scaling(rows, cols, iterations, threads, repeats)
            .expect("strong scaling failed");
        println!("Strong scaling benchmark finished.");
        return;
    }

    if benchmark == "weak" {
        scaling::run_weak_scaling(rows, cols, iterations, threads, repeats)
            .expect("weak scaling failed");
        println!("Weak scaling benchmark finished.");
        return;
    }

    if benchmark == "both" {
        scaling::run_strong_scaling(rows, cols, iterations, threads, repeats)
            .expect("strong scaling failed");
        scaling::run_weak_scaling(rows, cols, iterations, threads, repeats)
            .expect("weak scaling failed");
        println!("Strong + weak scaling finished.");
        return;
    }

    // =========================
    // DEMO MODES
    // =========================
    if demo || full_demo {
        println!("=== DEMO MODE ===");

        // obriši stare rezultate
        let _ = fs::remove_dir_all("visualization");
        let _ = fs::remove_dir_all("benchmarks");

        let demo_repeats = if full_demo { 30 } else { 3 };
        let demo_rows = if full_demo { rows } else { 200 };
        let demo_cols = if full_demo { cols } else { 200 };
        let demo_iters = if full_demo { iterations } else { 50 };
        let demo_threads = threads.min(8);

        // 1) Simulacija
        let mut grid = seq::initialize_grid(demo_rows, demo_cols);
        io::save_grid(&grid, 0, states_dir);

        for step in 1..=demo_iters {
            grid = parallel::next_generation_parallel(&grid, demo_threads);
            io::save_grid(&grid, step, states_dir);
        }

        // 2) Vizualizacija
        visualize::generate_images(demo_iters)
            .expect("Visualization failed");

        // 3) Strong scaling
        scaling::run_strong_scaling(
            demo_rows,
            demo_cols,
            demo_iters,
            demo_threads,
            demo_repeats,
        ).expect("Strong scaling failed");

        // 4) Weak scaling
        scaling::run_weak_scaling(
            demo_rows,
            demo_cols,
            demo_iters,
            demo_threads,
            demo_repeats,
        ).expect("Weak scaling failed");

        println!("Demo finished.");
        println!("Check visualization/ and benchmarks/ folders.");
        return;
    }

    // =========================
    // NORMAL RUN MODE
    // =========================

    let mut grid = seq::initialize_grid(rows, cols);
    io::save_grid(&grid, 0, states_dir);

    let start = Instant::now();

    if mode == "par" {
        for step in 1..=iterations {
            grid = parallel::next_generation_parallel(&grid, threads);
            io::save_grid(&grid, step, states_dir);
        }
    } else {
        for step in 1..=iterations {
            grid = seq::next_generation(&grid);
            io::save_grid(&grid, step, states_dir);
        }
    }

    let elapsed = start.elapsed();

    println!(
        "Mode: {mode}, rows={rows}, cols={cols}, iters={iterations}, threads={threads}"
    );
    println!("Compute time: {:.6} s", elapsed.as_secs_f64());
}
