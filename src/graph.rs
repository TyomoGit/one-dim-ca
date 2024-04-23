use plotters::{backend::BitMapBackend, coord::Shift, drawing::{DrawingArea, IntoDrawingArea}, style::{BLACK, WHITE}};

use crate::{cell::Cell, rule::Rule};

pub fn make_graph(out_file_name: &str, cells_history: &[Vec<Cell>], rule: Rule) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(out_file_name, (cells_history[0].len() as u32, cells_history.len() as u32)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let root = root.titled(&format!("{} CA", rule), ("sans-serif", 40))?;
    draw_cells_history(cells_history, &root);

    root.present()?;

    Ok(())
}

fn draw_cells_history(cells_history: &[Vec<Cell>], drawing_area: &DrawingArea<BitMapBackend, Shift>) {
    for (y, row) in cells_history.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell.is_alive {
                drawing_area.draw_pixel((x as i32, y as i32), &WHITE).unwrap();
            } else {
                drawing_area.draw_pixel((x as i32, y as i32), &BLACK).unwrap();
            }
        }
    }
}
