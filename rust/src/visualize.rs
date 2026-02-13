use crate::seq::Grid;
use plotters::prelude::*;
use std::fs;
use std::path::Path;
use crate::io;

pub fn draw_grid(grid: &Grid, iteration: usize, frames_dir: &str) {
    fs::create_dir_all(frames_dir).expect("failed to create frames dir");

    let filename = format!("{frames_dir}/frame_{iteration:04}.png");
    let path = Path::new(&filename);

    let rows = grid.len();
    let cols = grid[0].len();
    let cell_size = 10;
    let image_width = (cols * cell_size) as u32;
    let image_height = (rows * cell_size) as u32;

    let root = BitMapBackend::new(path, (image_width, image_height)).into_drawing_area();
    root.fill(&WHITE).expect("draw error");

    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            let x0 = (c * cell_size) as i32;
            let y0 = (r * cell_size) as i32;
            let x1 = x0 + cell_size as i32;
            let y1 = y0 + cell_size as i32;

            let color = if cell == 1 { &BLACK } else { &WHITE };

            root.draw(&Rectangle::new([(x0, y0), (x1, y1)], color.filled()))
                .expect("rectangle draw error");
        }
    }
    root.present().expect("save frame error");
}

pub fn draw_from_file(filepath: &str, iteration: usize, frames_dir: &str) {
    let grid = io::load_grid(filepath);
    draw_grid(&grid, iteration, frames_dir);
}

pub fn draw_scaling_plot(
    title: &str,
    output_png: &str,
    points: &[(usize, f64)],
    ideal: &[(usize, f64)],
    theory: &[(usize, f64)],
    theory_label: &str,
) {
    let root = BitMapBackend::new(output_png, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).expect("draw bg");

    let max_x = points.iter().map(|(x, _)| *x).max().unwrap_or(1) as f64;
    let max_y_points = points.iter().map(|(_, y)| *y).fold(1.0, f64::max);
    let max_y_theory = theory.iter().map(|(_, y)| *y).fold(1.0, f64::max);
    let max_y_ideal = ideal.iter().map(|(_, y)| *y).fold(1.0, f64::max);
    let max_y = max_y_points.max(max_y_theory).max(max_y_ideal) * 1.1;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 32))
        .margin(20)
        .x_label_area_size(45)
        .y_label_area_size(60)
        .build_cartesian_2d(1f64..max_x, 0f64..max_y)
        .expect("chart build");

    chart
        .configure_mesh()
        .x_desc("Broj jezgara / niti")
        .y_desc("Ubrzanje")
        .draw()
        .expect("mesh");

    chart
        .draw_series(LineSeries::new(
            points.iter().map(|(x, y)| (*x as f64, *y)),
            RED,
        ))
        .expect("points")
        .label("Merena kriva")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 15, y)], RED));

    chart
        .draw_series(LineSeries::new(
            ideal.iter().map(|(x, y)| (*x as f64, *y)),
            BLUE,
        ))
        .expect("ideal")
        .label("Idealno skaliranje")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 15, y)], BLUE));

    chart
        .draw_series(LineSeries::new(
            theory.iter().map(|(x, y)| (*x as f64, *y)),
            GREEN,
        ))
        .expect("theory")
        .label(theory_label)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 15, y)], GREEN));

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .draw()
        .expect("legend");

    root.present().expect("save plot");
}

pub fn generate_images(_iterations: usize) -> std::io::Result<()> {
    Ok(())
}

