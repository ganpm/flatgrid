use crate::cell::Cell;
use crate::border::Border;

use std::fmt::{Display, Formatter, Error};
use std::collections::{VecDeque};


#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Grid {
    cells: Vec<Cell>,
    rows: usize,
    cols: usize,
}


impl Grid {

    pub fn new(
        rows: usize,
        cols: usize,
    ) -> Self {
        let cells = Vec::with_capacity(cols * rows);
        Grid { cells, cols, rows }
    }

    pub fn from<T, U, V>(
        data: T,
    ) -> Self
    where
        T: IntoIterator<Item = U>,
        U: IntoIterator<Item = V>,
        V: Into<Cell>,
    {
        let cells_2d: Vec<Vec<Cell>> = data
            .into_iter()
            .map(|row| row.into_iter().map(Into::into).collect())
            .collect();
        let height = cells_2d.len();
        let width = cells_2d.iter().map(|row| row.len()).max().unwrap_or(0);
        // Flatten the 2d vector into a 1d vector and fill missing cells with default Cell
        let cells: Vec<Cell> = cells_2d
            .into_iter()
            .flat_map(|row| {
                let mut row_cells = row;
                if row_cells.len() < width {
                    row_cells.resize(width, Cell::default());
                }
                row_cells
            })
            .collect();
        Grid { cells, cols: width, rows: height }
    }

    pub fn get_cell(
        &self,
        row: usize,
        col: usize
    ) -> Option<&Cell>
    {
        self.cells.get(row * self.cols + col)
    }

    pub fn get_cell_mut(
        &mut self,
        row: usize,
        col: usize
    ) -> Option<&mut Cell>
    {
        self.cells.get_mut(row * self.cols + col)
    }

    pub fn row_iter(
        &self,
        row_index: usize
    ) -> impl Iterator<Item = &Cell>
    {
        let row_index = if row_index < self.rows {
            row_index
        } else {
            // Skip everything if out of bounds
            // This will return an empty iterator
            self.rows
        };
        self.cells.iter()
            .skip(row_index * self.cols)
            .take(self.cols)
    }

    pub fn col_iter(
        &self,
        col_index: usize
    ) -> impl Iterator<Item = &Cell>
    {
        let col_index = if col_index < self.cols {
            col_index
        } else {
            // Skip everything if out of bounds
            // This will return an empty iterator
            self.rows * self.cols
        };
        self.cells.iter()
            .skip(col_index)
            .step_by(self.cols)
    }

    pub fn row_iter_mut(
        &mut self,
        row_index: usize
    ) -> impl Iterator<Item = &mut Cell>
    {
        let row_index = if row_index < self.rows {
            row_index
        } else {
            // Skip everything if out of bounds
            // This will return an empty iterator
            self.rows
        };
        self.cells.iter_mut()
            .skip(row_index * self.cols)
            .take(self.cols)
    }

    pub fn col_iter_mut(
        &mut self,
        col_index: usize
    ) -> impl Iterator<Item = &mut Cell>
    {
        let col_index = if col_index < self.cols {
            col_index
        } else {
            // Skip everything if out of bounds
            // This will return an empty iterator
            self.rows * self.cols
        };
        self.cells.iter_mut()
            .skip(col_index)
            .step_by(self.cols)
    }

}


impl Display for Grid {

    fn fmt(
        &self,
        f: &mut Formatter,
    ) -> Result<(), Error>
    {
        let row_heights: Vec<usize> =
            (0..self.rows).map(|row_index|
                (0..self.cols).map(|col_index|
                    self.get_cell(row_index, col_index)
                        .unwrap_or(&Cell::default())
                        .height()
                )
                .max().unwrap_or(0)
            )
            .collect();
        let col_widths: Vec<usize> =
            (0..self.cols).map(|col_index|
                (0..self.rows).map(|row_index|
                    self.get_cell(row_index, col_index)
                        .unwrap_or(&Cell::default())
                        .width()
                )
                .max().unwrap_or(0)
            )
            .collect();

        let top_border = Border::render_top_border(&col_widths);
        let mid_border = Border::render_mid_border(&col_widths);
        let bot_border = Border::render_bot_border(&col_widths);

        writeln!(f, "{}", top_border)?;
        for row_index in 0..self.rows {
            let mut lines = Vec::with_capacity(self.cols);
            for col_index in 0..self.cols {
                if let Some(cell) = self.get_cell(row_index, col_index) {
                    let rendered_lines = cell.render_lines(row_heights[row_index], col_widths[col_index]);
                    lines.push(VecDeque::from(rendered_lines));
                } else {
                    lines.push(VecDeque::new());
                }
            }
            for _ in 0..row_heights[row_index] {
                let row_line: Vec<String> = lines.iter_mut()
                    .filter_map(|line| line.pop_front())
                    .collect();
                let row_str = Border::render_row_lines(row_line);
                writeln!(f, "{}", row_str)?;
            }
            if row_index < self.rows - 1 {
                writeln!(f, "{}", mid_border.clone())?;
            }
        }
        writeln!(f, "{}", bot_border)?;
        Ok(())
    }
    
}

