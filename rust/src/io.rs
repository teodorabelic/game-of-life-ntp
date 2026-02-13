use crate::seq::Grid;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

pub fn save_grid(grid: &Grid, iteration: usize, states_dir: &str) {
    fs::create_dir_all(states_dir).expect("failed to create states directory");
    let filename = format!("{states_dir}/state_{iteration:04}.txt");
    let mut file = File::create(filename).expect("failed to create state file");

    for row in grid {
        let line = row
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(file, "{line}").expect("failed to write row");
    }
}

// reads grid from states/
pub fn load_grid(path: &str) -> Grid {
    let file = File::open(path).expect("failed to open state file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.expect("invalid line")
                .split_whitespace()
                .map(|v| v.parse::<u8>().expect("invalid cell value"))
                .collect()
        })
        .collect()
}
