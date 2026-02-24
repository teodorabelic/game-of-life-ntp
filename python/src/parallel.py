import numpy as np
import os
import time
from multiprocessing import Pool, cpu_count


# =========================
# Inicijalizacija
# =========================
def initialize_grid(rows, cols, seed=None):
    if seed is not None:
        np.random.seed(seed)
    return (np.random.rand(rows, cols) > 0.7).astype(np.uint8)


# =========================
# Brzi neighbors
# =========================
def count_neighbors_fast(grid):
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


# =========================
# Obrada bloka
# =========================
def process_block(args):
    block, start_row, end_row = args

    neighbors = count_neighbors_fast(block)

    new_block = (
        ((block == 1) & ((neighbors == 2) | (neighbors == 3))) |
        ((block == 0) & (neighbors == 3))
    ).astype(np.uint8)

    return new_block, start_row, end_row


# =========================
# Paralelna iteracija
# =========================
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


# =========================
# Simulacija (Persistent Pool)
# =========================
def run_simulation(rows, cols, iterations, output_dir, workers, seed=None):

    grid = initialize_grid(rows, cols, seed)
    os.makedirs(output_dir, exist_ok=True)

    start = time.time()

    with Pool(processes=workers) as pool:
        for _ in range(iterations):
            grid = next_generation_parallel(grid, workers, pool)

    end = time.time()

    print("Parallel Python finished.")
    print(f"Workers: {workers}")
    print(f"Time: {end - start:.6f} s")
