use std::sync::{Arc, Mutex};
use std::thread;
use crate::io::save_grid;

// 1.grid is separated between 4 threads 
// 2.Arc (atomically reference counted) - reading threads without copies 
// 3.Mutex - securing writing into new matrix (every thread in new matrix)
// 4.Every thread - counts neighbors, applies rules, writing result in Mutex-protected matrix
// 5.join() waits for every thread
// 6.result is returned as new generation

pub fn generate_random_grid(rows: usize, cols: usize) -> Vec<Vec<u8>> {
    let mut grid = vec![vec![0; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            let value: f32 = rand::random();
            grid[r][c] = if value > 0.7 { 1 } else { 0 };
        }
    }

    grid
}

pub fn next_generation_parallel(grid: &Vec<Vec<u8>>, num_threads: usize) -> Vec<Vec<u8>> {
    let rows = grid.len();
    let cols = grid[0].len();

    let grid = Arc::new(grid.clone());
    let new_grid = Arc::new(Mutex::new(vec![vec![0; cols]; rows]));

    let chunk_size = rows / num_threads;
    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let grid_clone = Arc::clone(&grid);
        let new_grid_clone = Arc::clone(&new_grid);

        let start_row = thread_id * chunk_size;
        let end_row = if thread_id == num_threads - 1 {
            rows
        } else {
            start_row + chunk_size
        };

        let handle = thread::spawn(move || {
            for r in start_row..end_row {
                for c in 0..cols {
                    let neighbors = count_neighbors(&grid_clone, r, c);

                    let new_state = match (grid_clone[r][c], neighbors) {
                        (1, x) if x < 2 => 0,
                        (1, 2) | (1, 3) => 1,
                        (1, x) if x > 3 => 0,
                        (0, 3) => 1,
                        (state, _) => state,
                    };

                    let mut locked = new_grid_clone.lock().unwrap();
                    locked[r][c] = new_state;
                }
            }
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    Arc::try_unwrap(new_grid).unwrap().into_inner().unwrap()
}

fn count_neighbors(grid: &Vec<Vec<u8>>, r: usize, c: usize) -> u8 {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    let mut count = 0;

    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 { continue; }

            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
                count += grid[nr as usize][nc as usize];
            }
        }
    }

    count
}
