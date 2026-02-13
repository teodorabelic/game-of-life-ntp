use std::fs::{self, File};
use std::io::{Write, BufRead, BufReader};

pub fn save_grid(grid: &Vec<Vec<u8>>, iteration: usize) {
    fs::create_dir_all("states").unwrap();
    let filename = format!("states/state_{:04}.txt", iteration);
    let mut file = File::create(filename).unwrap();

    for row in grid {
        let line = row.iter()
                      .map(|v| v.to_string())
                      .collect::<Vec<_>>()
                      .join(" ");
        writeln!(file, "{}", line).unwrap();
    }
}

pub fn load_grid(path: &str) -> Vec<Vec<u8>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|v| v.parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}
