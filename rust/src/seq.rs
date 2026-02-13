pub type Grid = Vec<Vec<u8>>;

pub fn initialize_grid(rows: usize, cols: usize) -> Grid {
    generate_random_grid(rows, cols)
}

pub fn generate_random_grid(rows: usize, cols: usize) -> Grid {
    let mut grid = vec![vec![0; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            let value: f32 = rand::random();
            grid[r][c] = if value > 0.7 { 1 } else { 0 };
        }
    }

    grid
}

pub fn next_generation(grid: &Grid) -> Grid {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut new_grid = vec![vec![0; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            let neighbors = count_neighbors(grid, r, c);

            new_grid[r][c] = match (grid[r][c], neighbors) {
                (1, x) if x < 2 => 0,
                (1, 2) | (1, 3) => 1,
                (1, x) if x > 3 => 0,
                (0, 3) => 1,
                (state, _) => state,
            };
        }
    }

    new_grid
}

pub fn run_simulation(initial: &Grid, iterations: usize) -> Grid {
    let mut grid = initial.clone();
    for _ in 0..iterations {
        grid = next_generation(&grid);
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
