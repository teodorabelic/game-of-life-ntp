mod io;
mod parallel;
mod scaling;
mod seq;
mod visualize;

use std::time::Instant;

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
    let save_states = !has_flag(&args, "--no-save");
    let do_visualize = has_flag(&args, "--viz");

    let benchmark = parse_arg_string(&args, "--benchmark", "none");
    let repeats = parse_arg_usize(&args, "--repeats", 5);

    if benchmark == "strong" {
        scaling::run_strong_scaling(rows, cols, iterations, threads, repeats)
            .expect("strong scaling failed");
        println!("Strong scaling benchmark finished (benchmarks/rust).");
        return;
    }

    if benchmark == "weak" {
        scaling::run_weak_scaling(rows, cols, iterations, threads, repeats)
            .expect("weak scaling failed");
        println!("Weak scaling benchmark finished (benchmarks/rust).");
        return;
    }

    if benchmark == "both" {
        scaling::run_strong_scaling(rows, cols, iterations, threads, repeats)
            .expect("strong scaling failed");
        scaling::run_weak_scaling(rows, cols, iterations, threads, repeats)
            .expect("weak scaling failed");
        println!("Strong + weak scaling benchmarks finished (benchmarks/rust).");
        return;
    }


    let mut grid = seq::initialize_grid(rows, cols);

    if save_states {
        io::save_grid(&grid, 0, "states");
    }

    let start = Instant::now();

    if mode == "par" {
        for step in 1..=iterations {
            grid = parallel::next_generation_parallel(&grid, threads);

            if save_states {
                io::save_grid(&grid, step, "states");
            }
        }
    } else {
        for step in 1..=iterations {
            grid = seq::next_generation(&grid);

            if save_states {
                io::save_grid(&grid, step, "states");
            }
        }
    }

    let elapsed = start.elapsed();

    println!("Mode: {mode}, rows={rows}, cols={cols}, iters={iterations}, threads={threads}");
    println!("Compute time: {:.6} s", elapsed.as_secs_f64());

    if do_visualize && save_states {
        visualize::generate_images(iterations).expect("Visualization failed");
        println!("Visualization frames generated.");
    }
}

