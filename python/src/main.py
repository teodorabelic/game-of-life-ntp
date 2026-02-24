import numpy as np
import os
import argparse
import time
from multiprocessing import Pool, cpu_count
import matplotlib.pyplot as plt
import scaling


# =====================================
# Inicijalizacija
# =====================================
def initialize_grid(rows, cols, seed=None):
    if seed is not None:
        np.random.seed(seed)
    return (np.random.rand(rows, cols) > 0.7).astype(np.uint8)


# =====================================
# Sekvencijalna verzija
# =====================================
def count_neighbors_seq(grid):
    neighbors = np.zeros_like(grid, dtype=np.uint8)

    neighbors[:-1, :] += grid[1:, :]
    neighbors[1:, :] += grid[:-1, :]
    neighbors[:, :-1] += grid[:, 1:]
    neighbors[:, 1:] += grid[:, :-1]

    neighbors[:-1, :-1] += grid[1:, 1:]
    neighbors[:-1, 1:] += grid[1:, :-1]
    neighbors[1:, :-1] += grid[:-1, 1:]
    neighbors[1:, 1:] += grid[:-1, :-1]

    return neighbors


def next_generation_seq(grid):
    neighbors = count_neighbors_seq(grid)

    return (
        ((grid == 1) & ((neighbors == 2) | (neighbors == 3))) |
        ((grid == 0) & (neighbors == 3))
    ).astype(np.uint8)


# =====================================
# Paralelna verzija
# =====================================
def process_block(args):
    block, start_row, end_row = args

    neighbors = count_neighbors_seq(block)

    new_block = (
        ((block == 1) & ((neighbors == 2) | (neighbors == 3))) |
        ((block == 0) & (neighbors == 3))
    ).astype(np.uint8)

    return new_block, start_row, end_row


def next_generation_parallel(grid, workers, pool):
    rows = grid.shape[0]
    workers = min(workers, rows)
    chunk_size = rows // workers

    tasks = []
    start = 0

    for i in range(workers):
        end = start + chunk_size
        if i == workers - 1:
            end = rows

        block = grid[start:end]
        tasks.append((block, start, end))
        start = end

    results = pool.map(process_block, tasks)

    new_grid = np.zeros_like(grid)

    for block_result, start_row, end_row in results:
        new_grid[start_row:end_row] = block_result

    return new_grid


# =====================================
# MAIN
# =====================================
def run(mode, rows, cols, iterations, workers, seed, visualize):

    grid = initialize_grid(rows, cols, seed)

    start = time.time()

    if mode == "par":
        with Pool(processes=workers) as pool:
            for _ in range(iterations):
                grid = next_generation_parallel(grid, workers, pool)
    else:
        for _ in range(iterations):
            grid = next_generation_seq(grid)

    end = time.time()

    print(f"Mode: {mode}")
    print(f"Time: {end - start:.6f} s")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()

    parser.add_argument("--mode", type=str, default="seq")
    parser.add_argument("--rows", type=int, default=1000)
    parser.add_argument("--cols", type=int, default=1000)
    parser.add_argument("--iters", type=int, default=100)
    parser.add_argument("--workers", type=int, default=cpu_count())
    parser.add_argument("--seed", type=int, default=42)
    parser.add_argument("--benchmark", type=str, default="none")
    parser.add_argument("--repeats", type=int, default=5)

    args = parser.parse_args()

    if args.benchmark == "strong":
        scaling.run_strong_scaling(
            args.rows,
            args.cols,
            args.iters,
            args.workers,
            args.repeats
        )
        exit()

    if args.benchmark == "weak":
        scaling.run_weak_scaling(
            args.rows,
            args.cols,
            args.iters,
            args.workers,
            args.repeats
        )
        exit()

    run(
        args.mode,
        args.rows,
        args.cols,
        args.iters,
        args.workers,
        args.seed,
        False
    )
