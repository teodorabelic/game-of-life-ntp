use crate::parallel;
use crate::seq;
use crate::visualize;
use std::fs::{self, File};
use std::io::Write;
use std::time::Instant;

#[derive(Clone, Copy)]
struct Stats {
    mean: f64,
    stddev: f64,
    outliers: usize,
}

fn thread_points(max_threads: usize) -> Vec<usize> {
    let max_t = max_threads.max(1);
    let mut points = vec![1usize];
    let mut t = 2usize;
    while t <= max_t {
        points.push(t);
        t *= 2;
    }
    if *points.last().unwrap_or(&1) != max_t {
        points.push(max_t);
    }
    points.sort_unstable();
    points.dedup();
    points
}

fn calc_stats(samples: &[f64]) -> Stats {
    let n = samples.len().max(1) as f64;
    let mean = samples.iter().sum::<f64>() / n;
    let variance = samples
        .iter()
        .map(|x| {
            let d = x - mean;
            d * d
        })
        .sum::<f64>()
        / n;
    let stddev = variance.sqrt();
    let outliers = samples
        .iter()
        .filter(|x| (**x - mean).abs() > 2.0 * stddev)
        .count();

    Stats {
        mean,
        stddev,
        outliers,
    }
}

fn measure_seq(base: &seq::Grid, iterations: usize) -> f64 {
    let mut grid = base.clone();
    let start = Instant::now();
    for _ in 0..iterations {
        grid = seq::next_generation(&grid);
    }
    start.elapsed().as_secs_f64()
}

fn measure_par(base: &seq::Grid, iterations: usize, threads: usize) -> f64 {
    let mut grid = base.clone();
    let start = Instant::now();
    for _ in 0..iterations {
        grid = parallel::next_generation_parallel(&grid, threads);
    }
    start.elapsed().as_secs_f64()
}


fn estimate_parallel_fraction(two_thread_speedup: f64) -> f64 {
    if two_thread_speedup <= 1.0 {
        return 0.0;
    }
    let p = (1.0 - (1.0 / two_thread_speedup)) / (1.0 - (1.0 / 2.0));
    p.clamp(0.0, 1.0)
}

pub fn run_strong_scaling(
    rows: usize,
    cols: usize,
    iterations: usize,
    max_threads: usize,
    repeats: usize,
) -> std::io::Result<()> {
    fs::create_dir_all("benchmarks")?;
    let points = thread_points(max_threads);
    let repeats = repeats.max(1);

    let base = seq::initialize_grid(rows, cols);

    let seq_samples: Vec<f64> = (0..repeats)
        .map(|_| measure_seq(&base, iterations))
        .collect();
    let seq_stats = calc_stats(&seq_samples);

    let mut csv = File::create("benchmarks/strong_scaling.csv")?;
    writeln!(
        csv,
        "threads,rows,cols,iters,seq_mean,par_mean,par_stddev,speedup,outliers"
    )?;

    let mut measured_speedups = Vec::new();
    let mut ideal = Vec::new();

    let mut two_thread_speedup = 1.0;

    for &t in &points {
        let par_samples: Vec<f64> = (0..repeats)
            .map(|_| measure_par(&base, iterations, t))
            .collect();
        let par_stats = calc_stats(&par_samples);
        let speedup = seq_stats.mean / par_stats.mean;

        if t == 2 {
            two_thread_speedup = speedup;
        }

        writeln!(
            csv,
            "{t},{rows},{cols},{iterations},{:.6},{:.6},{:.6},{:.6},{}",
            seq_stats.mean, par_stats.mean, par_stats.stddev, speedup, par_stats.outliers
        )?;

        measured_speedups.push((t, speedup));
        ideal.push((t, t as f64));
    }

    let p = estimate_parallel_fraction(two_thread_speedup);
    let theory: Vec<(usize, f64)> = points
        .iter()
        .map(|&n| {
            let nf = n as f64;
            let s = 1.0 / ((1.0 - p) + (p / nf));
            (n, s)
        })
        .collect();

    visualize::draw_scaling_plot(
        "Rust jako skaliranje (Amdahl)",
        "benchmarks/strong_scaling.png",
        &measured_speedups,
        &ideal,
        &theory,
        "Amdahl",
    );

    Ok(())
}

pub fn run_weak_scaling(
    base_rows: usize,
    base_cols: usize,
    iterations: usize,
    max_threads: usize,
    repeats: usize,
) -> std::io::Result<()> {
    fs::create_dir_all("benchmarks")?;
    let points = thread_points(max_threads);
    let repeats = repeats.max(1);

    let base = seq::initialize_grid(base_rows, base_cols);

    let mut csv = File::create("benchmarks/weak_scaling.csv")?;
    writeln!(
        csv,
        "threads,rows,cols,iters,seq_mean,par_mean,par_stddev,speedup,outliers"
    )?;

    let mut measured_speedups = Vec::new();
    let mut ideal = Vec::new();

    let mut alpha = 0.2;

    for &t in &points {
        let rows = base_rows * t;
        let cols = base_cols;

        let seq_samples: Vec<f64> = (0..repeats)
            .map(|_| measure_seq(&base, iterations))
            .collect();
        let par_samples: Vec<f64> = (0..repeats)
            .map(|_| measure_par(&base, iterations, t))
            .collect();

        let seq_stats = calc_stats(&seq_samples);
        let par_stats = calc_stats(&par_samples);
        let speedup = seq_stats.mean / par_stats.mean;

        if t == 2 {
            // Gustafson: S = n - alpha*(n-1) => alpha = (n-S)/(n-1)
            alpha = ((2.0 - speedup) / 1.0).clamp(0.0, 1.0);
        }

        writeln!(
            csv,
            "{t},{rows},{cols},{iterations},{:.6},{:.6},{:.6},{:.6},{}",
            seq_stats.mean, par_stats.mean, par_stats.stddev, speedup, par_stats.outliers
        )?;

        measured_speedups.push((t, speedup));
        ideal.push((t, t as f64));
    }

    let theory: Vec<(usize, f64)> = points
        .iter()
        .map(|&n| {
            let nf = n as f64;
            let s = nf - alpha * (nf - 1.0);
            (n, s)
        })
        .collect();

    visualize::draw_scaling_plot(
        "Rust slabo skaliranje (Gustafson)",
        "benchmarks/weak_scaling.png",
        &measured_speedups,
        &ideal,
        &theory,
        "Gustafson",
    );

    Ok(())
}