import numpy as np
import os
import time

# NumPy
def initialize_grid(rows, cols, seed=None):
    if seed is not None:
        np.random.seed(seed)    # random 1/0
    # priblizno 30% zivih celija
    # pretvara True/False u 0/1
    return (np.random.rand(rows, cols) > 0.7).astype(np.uint8)

# ne koristim np.roll jer on pravi kopije matrice
def count_neighbors(grid):
    # prazna matrica za broj suseda
    neighbors = np.zeros_like(grid, dtype=np.uint8)

    # 8 suseda
    neighbors[:-1, :] += grid[1:, :]
    neighbors[1:, :] += grid[:-1, :]
    neighbors[:, :-1] += grid[:, 1:]
    neighbors[:, 1:] += grid[:, :-1]

    neighbors[:-1, :-1] += grid[1:, 1:]
    neighbors[:-1, 1:] += grid[1:, :-1]
    neighbors[1:, :-1] += grid[:-1, 1:]
    neighbors[1:, 1:] += grid[:-1, :-1]

    return neighbors

def next_generation(grid):
    neighbors = count_neighbors(grid)

    # pravila (vektorizovana)
    # NumPy koristi C backend, pa se Python interpreter minimalno koristi; mnogo brze od klasicne petlje
    return (
        ((grid == 1) & ((neighbors == 2) | (neighbors == 3))) |
        ((grid == 0) & (neighbors == 3))
    ).astype(np.uint8)

# cuva stanja u datoteku states
def save_grid(grid, iteration, output_dir):
    os.makedirs(output_dir, exist_ok=True)
    filename = os.path.join(output_dir, f"state_{iteration:04}.txt")
    np.savetxt(filename, grid, fmt="%d")

# svaka iteracija koristi prethodno stanje, generise novo i snima rezultat
def run_simulation(rows, cols, iterations, output_dir, seed=None):
    grid = initialize_grid(rows, cols, seed)

    save_grid(grid, 0, output_dir)

    start = time.time()

    for step in range(1, iterations + 1):
        grid = next_generation(grid)
        save_grid(grid, step, output_dir)

    end = time.time()

    print("Sequential Python finished.")
    print(f"Time: {end - start:.6f} s")
