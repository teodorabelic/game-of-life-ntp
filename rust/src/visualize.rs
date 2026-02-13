use plotters::prelude::*;
use std::path::Path;
use std::fs;

pub fn draw_from_file(filepath: &str, iteration: usize) {
    fs::create_dir_all("frames").unwrap();

    let grid = super::io::load_grid(filepath);

    let filename = format!("frames/frame_{:04}.png", iteration);
    let path = Path::new(&filename);

    let rows = grid.len();
    let cols = grid[0].len();

    let cell_size = 10;
    let image_width = (cols * cell_size) as u32;
    let image_height = (rows * cell_size) as u32;

    let root = BitMapBackend::new(path, (image_width, image_height))
        .into_drawing_area();

    root.fill(&WHITE).unwrap();

    for r in 0..rows {
        for c in 0..cols {
            let x0 = (c * cell_size) as i32;
            let y0 = (r * cell_size) as i32;
            let x1 = x0 + cell_size as i32;
            let y1 = y0 + cell_size as i32;

            let color = if grid[r][c] == 1 { &BLACK } else { &WHITE };

            root.draw(&Rectangle::new(
                [(x0, y0), (x1, y1)],
                color.filled(),
            )).unwrap();
        }
    }

    root.present().unwrap();
    println!("Saved {}", filename);
}
