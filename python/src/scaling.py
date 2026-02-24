import numpy as np
import os
import time
import csv
import matplotlib.pyplot as plt
from multiprocessing import Pool
from main import initialize_grid, next_generation_seq, next_generation_parallel


def calc_stats(samples):
    mean = np.mean(samples)
    stddev = np.std(samples)
    outliers = np.sum(np.abs(samples - mean) > 2 * stddev)
    return mean, stddev, outliers


def measure_seq(grid, iterations):
    g = grid.copy()
    start = time.time()
    for _ in range(iterations):
        g = next_generation_seq(g)
    return time.time() - start


def measure_par(grid, iterations, workers):
    g = grid.copy()
    start = time.time()

    with Pool(processes=workers) as pool:
        for _ in range(iterations):
            g = next_generation_parallel(g, workers, pool)

    return time.time() - start


# =====================================
# Strong Scaling
# =====================================
def run_strong_scaling(rows, cols, iterations, max_workers, repeats):

    base_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    bench_dir = os.path.join(base_dir, "benchmarks")
    os.makedirs(bench_dir, exist_ok=True)

    worker_points = [1, 2, 4, 8]
    worker_points = [w for w in worker_points if w <= max_workers]

    base_grid = initialize_grid(rows, cols, seed=42)

    seq_times = [measure_seq(base_grid, iterations) for _ in range(repeats)]
    seq_mean, seq_std, seq_out = calc_stats(np.array(seq_times))

    measured = []
    ideal = []

    csv_path = os.path.join(bench_dir, "strong_scaling_python.csv")

    with open(csv_path, "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["threads", "seq_mean", "par_mean", "par_std", "speedup", "outliers"])

        for w in worker_points:
            par_times = [measure_par(base_grid, iterations, w) for _ in range(repeats)]
            par_mean, par_std, outliers = calc_stats(np.array(par_times))

            speedup = seq_mean / par_mean

            writer.writerow([w, seq_mean, par_mean, par_std, speedup, outliers])

            measured.append((w, speedup))
            ideal.append((w, w))

    draw_plot(
        "Strong Scaling (Python)",
        os.path.join(bench_dir, "strong_scaling_python.png"),
        measured,
        ideal
    )


# =====================================
# Weak Scaling
# =====================================
def run_weak_scaling(base_rows, cols, iterations, max_workers, repeats):

    base_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    bench_dir = os.path.join(base_dir, "benchmarks")
    os.makedirs(bench_dir, exist_ok=True)

    worker_points = [1, 2, 4, 8]
    worker_points = [w for w in worker_points if w <= max_workers]

    measured = []
    ideal = []

    csv_path = os.path.join(bench_dir, "weak_scaling_python.csv")

    with open(csv_path, "w", newline="") as f:
        writer = csv.writer(f)
        writer.writerow(["threads", "seq_mean", "par_mean", "par_std", "speedup", "outliers"])

        for w in worker_points:
            rows = base_rows * w
            grid = initialize_grid(rows, cols, seed=42)

            seq_times = [measure_seq(grid, iterations) for _ in range(repeats)]
            par_times = [measure_par(grid, iterations, w) for _ in range(repeats)]

            seq_mean, _, _ = calc_stats(np.array(seq_times))
            par_mean, par_std, outliers = calc_stats(np.array(par_times))

            speedup = seq_mean / par_mean

            writer.writerow([w, seq_mean, par_mean, par_std, speedup, outliers])

            measured.append((w, speedup))
            ideal.append((w, w))

    draw_plot(
        "Weak Scaling (Python)",
        os.path.join(bench_dir, "weak_scaling_python.png"),
        measured,
        ideal
    )


def draw_plot(title, output_png, measured, ideal):
    plt.figure(figsize=(8, 6))

    x = [t for t, _ in measured]
    y_measured = [s for _, s in measured]
    y_ideal = [s for _, s in ideal]

    plt.plot(x, y_measured, marker='o', label="Measured")
    plt.plot(x, y_ideal, linestyle='--', label="Ideal")

    plt.xlabel("Broj procesa")
    plt.ylabel("Ubrzanje")
    plt.title(title)
    plt.legend()
    plt.grid()

    plt.savefig(output_png)
    plt.close()
