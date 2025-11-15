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
    ) -> Self
    {
        let cells = vec![Cell::default(); cols * rows];
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
        row: usize,
        col: usize,
        cell: T,
    )
    where
        T: Into<Cell>,
    {
        if row >= self.rows || col >= self.cols {
            panic!("Row or column index out of bounds");
        }
        self.cells[row * self.cols + col] = cell.into();
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
        let step = self.cols.max(1);
        self.cells.iter()
            .skip(col_index)
            .step_by(step)
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
        let step = self.cols.max(1);
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
        col_insert_index: usize,
        new_column: impl IntoIterator<Item = T>,
    )
    where
        T: Into<Cell>,
    {
        if col_insert_index > self.cols {
            panic!("Column index out of bounds");
        }

        let mut new_column: Vec<Cell> = new_column.into_iter().map(Into::into).collect();

        if new_column.len() < self.rows {
            panic!("New column has fewer cells than the number of rows in the grid");
        }
        if new_column.len() > self.rows {
            panic!("New column has more cells than the number of rows in the grid");
        }

        let old_cols = self.cols;
        let new_cols = self.cols + 1;
        let new_size = self.rows * new_cols;

        // Resize the existing cells to make room for the new column
        self.cells.resize(new_size, Cell::default());

        for ri in (0..self.rows).rev() {
            // Iterator for the previous column indices
            let mut cols_iter = (0..self.cols).rev();
            for ci in (0..new_cols).rev() {
                if ci == col_insert_index {
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

        self.cols = new_cols;
    }

    pub fn insert_row<T>(
        &mut self,
        row_insert_index: usize,
        new_row: impl IntoIterator<Item = T>,
    )
    where
        T: Into<Cell>,
    {
        if row_insert_index > self.rows {
            panic!("Row index out of bounds");
        }

        let mut new_row: Vec<Cell> = new_row.into_iter().map(Into::into).collect();

        if new_row.len() < self.cols {
            panic!("New row has fewer cells than the number of columns in the grid");
        }
        if new_row.len() > self.cols {
            panic!("New row has more cells than the number of columns in the grid");
        }

        let old_rows = self.rows;
        let new_rows = self.rows + 1;
        let new_size = new_rows * self.cols;

        // Resize the existing cells to make room for the new row
        self.cells.resize(new_size, Cell::default());

        for ci in (0..self.cols).rev() {
            // Iterator for the previous row indices
            let mut rows_iter = (0..old_rows).rev();
            for ri in (0..new_rows).rev() {
                if ri == row_insert_index {
                    // Insert the new cell
                    self.cells[ri * self.cols + ci] = new_row.pop().unwrap_or_default();
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
                    self.cells[ri * self.cols + ci] =
                        std::mem::take(&mut self.cells[old_ri * self.cols + ci]);
                }
            }
        }

        self.rows = new_rows;
    }

    pub fn set_col<T>(
        &mut self,
        col_index: usize,
        new_column: impl IntoIterator<Item = T>,
    )
    where
        T: Into<Cell>,
    {
        if col_index >= self.cols {
            panic!("Column index out of bounds");
        }

        let new_column: Vec<Cell> = new_column.into_iter().map(Into::into).collect();

        if new_column.len() != self.rows {
            panic!("New column has a different number of cells than the number of rows in the grid");
        }

        for (row_index, cell) in new_column.into_iter().enumerate() {
            self.cells[row_index * self.cols + col_index] = cell;
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
        if row_index >= self.rows {
            panic!("Row index out of bounds");
        }

        let new_row: Vec<Cell> = new_row.into_iter().map(Into::into).collect();

        if new_row.len() != self.cols {
            panic!("New row has a different number of cells than the number of columns in the grid");
        }

        for (col_index, cell) in new_row.into_iter().enumerate() {
            self.cells[row_index * self.cols + col_index] = cell;
        }
    }

    pub fn resize(
        &mut self,
        new_rows: usize,
        new_cols: usize,
    )
    {
        let old_rows = self.rows;
        let old_cols = self.cols;

        let mut new_cells = vec![Cell::default(); new_rows * new_cols];

        let rows = std::cmp::min(old_rows, new_rows);
        let cols = std::cmp::min(old_cols, new_cols);

        for row_index in 0..rows {
            for col_index in 0..cols {
                new_cells[row_index * new_cols + col_index] =
                    std::mem::take(&mut self.cells[row_index * old_cols + col_index]);
            }
        }

        self.cells = new_cells;
        self.rows = new_rows;
        self.cols = new_cols;
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

        writeln!(f, "{}", &top_border)?;
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
                writeln!(f, "{}", &mid_border)?;
            }
        }
        writeln!(f, "{}", &bot_border)?;
        Ok(())
    }
    
}

