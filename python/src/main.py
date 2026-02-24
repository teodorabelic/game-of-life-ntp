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
# I/O
# =====================================
def save_grid(grid, iteration, states_dir):
    os.makedirs(states_dir, exist_ok=True)
    filename = os.path.join(states_dir, f"state_{iteration:04}.txt")
    np.savetxt(filename, grid, fmt="%d")


def draw_frame(grid, iteration, frames_dir):
    os.makedirs(frames_dir, exist_ok=True)

    plt.figure(figsize=(6, 6))
    plt.imshow(grid, cmap="binary")
    plt.axis("off")

    filename = os.path.join(frames_dir, f"frame_{iteration:04}.png")
    plt.savefig(filename, bbox_inches="tight", pad_inches=0)
    plt.close()


# =====================================
# MAIN
# =====================================
def run(mode, rows, cols, iterations, workers, seed, visualize):

    base_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    outputs_dir = os.path.join(base_dir, "visualization")

    states_dir = os.path.join(outputs_dir, "states")
    frames_dir = os.path.join(outputs_dir, "frames")

    grid = initialize_grid(rows, cols, seed)

    # snimi početno stanje
    save_grid(grid, 0, states_dir)

    if visualize:
        draw_frame(grid, 0, frames_dir)

    start = time.time()

    if mode == "par":
        with Pool(processes=workers) as pool:
            for step in range(1, iterations + 1):
                grid = next_generation_parallel(grid, workers, pool)
                save_grid(grid, step, states_dir)
                if visualize:
                    draw_frame(grid, step, frames_dir)
    else:
        for step in range(1, iterations + 1):
            grid = next_generation_seq(grid)
            save_grid(grid, step, states_dir)
            if visualize:
                draw_frame(grid, step, frames_dir)

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
    parser.add_argument("--viz", action="store_true")
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
        args.viz
    )