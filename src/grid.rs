use crate::cell::Cell;
use crate::border::Border;

use std::fmt::{Display, Formatter, Error};
use std::collections::{VecDeque};


#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Grid {
    cells: Vec<Cell>,
    row_size: usize,
    col_size: usize,
}


impl Grid {

    pub fn new(
        row_size: usize,
        col_size: usize,
    ) -> Self
    {
        let cells = vec![Cell::default(); col_size * row_size];
        Grid { cells, col_size, row_size }
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
        let row_size = cells_2d.len();
        let col_size = cells_2d.iter().map(|row| row.len()).max().unwrap_or(0);
        // Flatten the 2d vector into a 1d vector and fill missing cells with default Cell
        let cells: Vec<Cell> = cells_2d
            .into_iter()
            .flat_map(|row| {
                let mut row_cells = row;
                if row_cells.len() < col_size {
                    row_cells.resize(col_size, Cell::default());
                }
                row_cells
            })
            .collect();
        Grid { cells, col_size, row_size }
    }

    pub fn set_cells<T>(
        &mut self,
        cells: T,
    )
    where
        T: IntoIterator<Item = Cell>,
    {
        let mut iter = cells.into_iter();
        for cell in self.cells.iter_mut() {
            *cell = iter.next().unwrap_or_default();
        }
    }

    pub fn set_cell<T>(
        &mut self,
        row_index: usize,
        col_index: usize,
        cell_data: T,
    )
    where
        T: Into<Cell>,
    {
        if row_index >= self.row_size || col_index >= self.col_size {
            panic!("Row or column index out of bounds");
        }
        self.cells[row_index * self.col_size + col_index] = cell_data.into();
    }

    pub fn get_cell(
        &self,
        row_index: usize,
        col_index: usize
    ) -> Option<&Cell>
    {
        self.cells.get(row_index * self.col_size + col_index)
    }

    pub fn get_cell_mut(
        &mut self,
        row_index: usize,
        col_index: usize
    ) -> Option<&mut Cell>
    {
        self.cells.get_mut(row_index * self.col_size + col_index)
    }

    pub fn row_iter(
        &self,
        row_index: usize
    ) -> impl Iterator<Item = &Cell>
    {
        let row_index = if row_index < self.row_size {
            row_index
        } else {
            // Skip everything if out of bounds
            // This will return an empty iterator
            self.row_size
        };
        self.cells.iter()
            .skip(row_index * self.col_size)
            .take(self.col_size)
    }

    pub fn col_iter(
        &self,
        col_index: usize
    ) -> impl Iterator<Item = &Cell>
    {
        let col_index = if col_index < self.col_size {
            col_index
        } else {
            // Skip everything if out of bounds
            // This will return an empty iterator
            self.row_size * self.col_size
        };
        let step = self.col_size.max(1);
        self.cells.iter()
            .skip(col_index)
            .step_by(step)
    }

    pub fn row_iter_mut(
        &mut self,
        row_index: usize
    ) -> impl Iterator<Item = &mut Cell>
    {
        let row_index = if row_index < self.row_size {
            row_index
        } else {
            // Skip everything if out of bounds
            // This will return an empty iterator
            self.row_size
        };
        self.cells.iter_mut()
            .skip(row_index * self.col_size)
            .take(self.col_size)
    }

    pub fn col_iter_mut(
        &mut self,
        col_index: usize
    ) -> impl Iterator<Item = &mut Cell>
    {
        let col_index = if col_index < self.col_size {
            col_index
        } else {
            // Skip everything if out of bounds
            // This will return an empty iterator
            self.row_size * self.col_size
        };
        let step = self.col_size.max(1);
        self.cells.iter_mut()
            .skip(col_index)
            .step_by(step)
    }

    pub fn flat_iter(
        &self
    ) -> impl Iterator<Item = &Cell>
    {
        self.cells.iter()
    }

    pub fn flat_iter_mut(
        &mut self
    ) -> impl Iterator<Item = &mut Cell>
    {
        self.cells.iter_mut()
    }

    pub fn insert_col<T>(
        &mut self,
        col_index: usize,
        new_column: impl IntoIterator<Item = T>,
    )
    where
        T: Into<Cell>,
    {
        if col_index > self.col_size {
            panic!("Column index out of bounds");
        }

        let mut new_column: Vec<Cell> = new_column.into_iter().map(Into::into).collect();

        if new_column.len() < self.row_size {
            panic!("New column has fewer cells than the number of rows in the grid");
        }
        if new_column.len() > self.row_size {
            panic!("New column has more cells than the number of rows in the grid");
        }

        let old_cols = self.col_size;
        let new_cols = self.col_size + 1;
        let new_size = self.row_size * new_cols;

        // Resize the existing cells to make room for the new column
        self.cells.resize(new_size, Cell::default());

        for ri in (0..self.row_size).rev() {
            // Iterator for the previous column indices
            let mut cols_iter = (0..self.col_size).rev();
            for ci in (0..new_cols).rev() {
                if ci == col_index {
                    // Insert the new cell
                    self.cells[ri * new_cols + ci] = new_column.pop().unwrap_or_default();
                    if new_column.is_empty() {
                        // We can break early here because
                        // its guaranteed that the rest of the cells in the
                        // current row are already in place if the new column
                        // is empty
                        break;
                    }
                } else {
                    // Only move the iterator when we're not inserting
                    let old_ci = cols_iter.next().unwrap();
                    // Move the cells from the old index to the new index
                    self.cells[ri * new_cols + ci] =
                        std::mem::take(&mut self.cells[ri * old_cols + old_ci]);
                }
            }
        }

        self.col_size = new_cols;
    }

    pub fn insert_row<T>(
        &mut self,
        row_index: usize,
        new_row: impl IntoIterator<Item = T>,
    )
    where
        T: Into<Cell>,
    {
        if row_index > self.row_size {
            panic!("Row index out of bounds");
        }

        let mut new_row: Vec<Cell> = new_row.into_iter().map(Into::into).collect();

        if new_row.len() < self.col_size {
            panic!("New row has fewer cells than the number of columns in the grid");
        }
        if new_row.len() > self.col_size {
            panic!("New row has more cells than the number of columns in the grid");
        }

        let old_rows = self.row_size;
        let new_rows = self.row_size + 1;
        let new_size = new_rows * self.col_size;

        // Resize the existing cells to make room for the new row
        self.cells.resize(new_size, Cell::default());

        for ci in (0..self.col_size).rev() {
            // Iterator for the previous row indices
            let mut rows_iter = (0..old_rows).rev();
            for ri in (0..new_rows).rev() {
                if ri == row_index {
                    // Insert the new cell
                    self.cells[ri * self.col_size + ci] = new_row.pop().unwrap_or_default();
                    if new_row.is_empty() {
                        // We can break early here because
                        // its guaranteed that the rest of the cells in the
                        // current column are already in place if the new row
                        // is empty
                        break;
                    }
                } else {
                    // Only move the iterator when we're not inserting
                    let old_ri = rows_iter.next().unwrap();
                    // Move the cells from the old index to the new index
                    self.cells[ri * self.col_size + ci] =
                        std::mem::take(&mut self.cells[old_ri * self.col_size + ci]);
                }
            }
        }

        self.row_size = new_rows;
    }

    pub fn set_col<T>(
        &mut self,
        col_index: usize,
        new_column: impl IntoIterator<Item = T>,
    )
    where
        T: Into<Cell>,
    {
        if col_index >= self.col_size {
            panic!("Column index out of bounds");
        }

        let new_column: Vec<Cell> = new_column.into_iter().map(Into::into).collect();

        if new_column.len() != self.row_size {
            panic!("New column has a different number of cells than the number of rows in the grid");
        }

        for (row_index, cell) in new_column.into_iter().enumerate() {
            self.cells[row_index * self.col_size + col_index] = cell;
        }
    }

    pub fn set_row<T>(
        &mut self,
        row_index: usize,
        new_row: impl IntoIterator<Item = T>,
    )
    where
        T: Into<Cell>,
    {
        if row_index >= self.row_size {
            panic!("Row index out of bounds");
        }

        let new_row: Vec<Cell> = new_row.into_iter().map(Into::into).collect();

        if new_row.len() != self.col_size {
            panic!("New row has a different number of cells than the number of columns in the grid");
        }

        for (col_index, cell) in new_row.into_iter().enumerate() {
            self.cells[row_index * self.col_size + col_index] = cell;
        }
    }

    pub fn resize(
        &mut self,
        new_row_size: usize,
        new_col_size: usize,
    )
    {
        let old_rows = self.row_size;
        let old_cols = self.col_size;

        let mut new_cells = vec![Cell::default(); new_row_size * new_col_size];

        let rows = std::cmp::min(old_rows, new_row_size);
        let cols = std::cmp::min(old_cols, new_col_size);

        for row_index in 0..rows {
            for col_index in 0..cols {
                new_cells[row_index * new_col_size + col_index] =
                    std::mem::take(&mut self.cells[row_index * old_cols + col_index]);
            }
        }

        self.cells = new_cells;
        self.row_size = new_row_size;
        self.col_size = new_col_size;
    }

}


impl Display for Grid {

    fn fmt(
        &self,
        f: &mut Formatter,
    ) -> Result<(), Error>
    {
        let row_heights: Vec<usize> =
            (0..self.row_size).map(|row_index|
                (0..self.col_size).map(|col_index|
                    self.get_cell(row_index, col_index)
                        .unwrap_or(&Cell::default())
                        .height()
                )
                .max().unwrap_or(0)
            )
            .collect();
        let col_widths: Vec<usize> =
            (0..self.col_size).map(|col_index|
                (0..self.row_size).map(|row_index|
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

        writeln!(f, "{}", &top_border)?;
        for row_index in 0..self.row_size {
            let mut lines = Vec::with_capacity(self.col_size);
            for col_index in 0..self.col_size {
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
            if row_index < self.row_size - 1 {
                writeln!(f, "{}", &mid_border)?;
            }
        }
        writeln!(f, "{}", &bot_border)?;
        Ok(())
    }
    
}

