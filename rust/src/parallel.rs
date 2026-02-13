use crate::seq::Grid;
use std::sync::Arc;
use std::thread;

pub fn next_generation_parallel(grid: &Grid, num_threads: usize) -> Grid {
    let rows = grid.len();
    let cols = grid[0].len();

    let threads = num_threads.max(1).min(rows.max(1));
    let chunk_size = rows.div_ceil(threads);

    let shared = Arc::new(grid.clone());

    let mut handles: Vec<std::thread::JoinHandle<(usize, Grid)>> =
        Vec::with_capacity(threads);

    for thread_id in 0..threads {
        let g = Arc::clone(&shared);
        let start = thread_id * chunk_size;
        let end = ((thread_id + 1) * chunk_size).min(rows);

        if start >= end {
            continue;
        }

        handles.push(thread::spawn(move || {
            let mut chunk = vec![vec![0u8; cols]; end - start];

            for r in start..end {
                for c in 0..cols {
                    let neighbors = count_neighbors(&g, r, c);

                    chunk[r - start][c] = match (g[r][c], neighbors) {
                        (1, x) if x < 2 => 0,
                        (1, 2) | (1, 3) => 1,
                        (1, x) if x > 3 => 0,
                        (0, 3) => 1,
                        (state, _) => state,
                    };
                }
            }

            (start, chunk)
        }));
    }

    let mut result = vec![vec![0u8; cols]; rows];

    for h in handles {
        let (start, chunk) = h.join().expect("thread panic");
        for (offset, row) in chunk.into_iter().enumerate() {
            result[start + offset] = row;
        }
    }

    result
}

pub fn run_simulation_parallel(
    initial: &Grid,
    iterations: usize,
    num_threads: usize,
) -> Grid {
    let mut grid = initial.clone();
    for _ in 0..iterations {
        grid = next_generation_parallel(&grid, num_threads);
    }
    grid
}

fn count_neighbors(grid: &Grid, r: usize, c: usize) -> u8 {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }

            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                count += grid[nr as usize][nc as usize];
            }
        }
    }

    count
}
