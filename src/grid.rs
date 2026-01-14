use crate::cell::Cell;
use crate::border::Border;
use crate::error::{GridError, ErrorMessage};

use std::fmt::{Display, Formatter, Error};
use std::collections::{VecDeque};

/// A 2D grid of cells stored in a flat vector in row-major order.

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Grid {
    cells: Vec<Cell>,
    row_size: usize,
    col_size: usize,
}


impl Grid {

    /// Create a new grid with the specified number of rows and columns.
    /// 
    /// All cells will be initialized to the default cell.

    pub fn new(
        row_size: usize,
        col_size: usize,
    ) -> Self
    {
        let cells = vec![Cell::default(); col_size * row_size];
        Grid { cells, col_size, row_size }
    }

    /// Create a new grid from a 2D iterator.
    /// 
    /// The outer iterator represents rows, and the inner iterator represents columns.
    /// 
    /// If the rows have different lengths, the missing cells will be filled with default cells.
    /// 
    /// The number of columns will be determined by the longest row.
    /// 
    /// The number of rows will be determined by the number of rows in the outer iterator.

    pub fn from(
        data: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<Cell>>>,
    ) -> Self
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

    /// Set the entire grid's cells from an iterator.
    /// 
    /// If the iterator has fewer items than the grid size,
    /// the remaining cells will be filled with the default cell.
    /// 
    /// If the iterator has more items than the grid size,
    /// the extra items will be ignored.

    pub fn set_cells(
        &mut self,
        new_cells: impl IntoIterator<Item = impl Into<Cell>>,
    )
    {
        let mut new_cells_iter = new_cells.into_iter();
        for cell in self.cells.iter_mut() {
            *cell = new_cells_iter
                .next()
                .map(Into::into)
                .unwrap_or_default();
        }
    }

    /// Set the cell at the specified row and column indices.
    /// 
    /// Panics if the indices are out of bounds.

    pub fn set_cell(
        &mut self,
        row_index: usize,
        col_index: usize,
        cell_data: impl Into<Cell>,
    )
    {
        if row_index >= self.row_size && col_index >= self.col_size {
            panic!("{}", ErrorMessage::ROW_AND_COL_INDEX_OUT_OF_BOUNDS);
        }
        if row_index >= self.row_size {
            panic!("{}", ErrorMessage::ROW_INDEX_OUT_OF_BOUNDS);
        }
        if col_index >= self.col_size {
            panic!("{}", ErrorMessage::COL_INDEX_OUT_OF_BOUNDS);
        }
        self.cells[row_index * self.col_size + col_index] = cell_data.into();
    }

    /// Try to set the cell at the specified row and column indices.
    /// 
    /// Returns an error if the indices are out of bounds.

    pub fn try_set_cell(
        &mut self,
        row_index: usize,
        col_index: usize,
        cell_data: impl Into<Cell>,
    ) -> Result<(), GridError>
    {
        if row_index >= self.row_size && col_index >= self.col_size {
            return Err(GridError::RowAndColIndexOutOfBounds);
        }
        if row_index >= self.row_size {
            return Err(GridError::RowIndexOutOfBounds);
        }
        if col_index >= self.col_size {
            return Err(GridError::ColIndexOutOfBounds);
        }
        self.cells[row_index * self.col_size + col_index] = cell_data.into();
        Ok(())
    }

    /// Get an immutable reference to the cell at the specified row and column indices.
    /// 
    /// Returns None if the indices are out of bounds.

    pub fn get_cell(
        &self,
        row_index: usize,
        col_index: usize
    ) -> Option<&Cell>
    {
        self.cells.get(row_index * self.col_size + col_index)
    }

    /// Get a mutable reference to the cell at the specified row and column indices.
    /// 
    /// Returns None if the indices are out of bounds.

    pub fn get_cell_mut(
        &mut self,
        row_index: usize,
        col_index: usize
    ) -> Option<&mut Cell>
    {
        self.cells.get_mut(row_index * self.col_size + col_index)
    }

    /// Get an immutable iterator over the cells in the specified row.
    /// 
    /// If the row index is out of bounds, returns an empty iterator.

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

    /// Get an immutable iterator over the cells in the specified column.
    /// 
    /// If the column index is out of bounds, returns an empty iterator.

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

    /// Get a mutable iterator over the cells in the specified row.
    /// 
    /// If the row index is out of bounds, returns an empty iterator.

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

    /// Get a mutable iterator over the cells in the specified column.
    /// 
    /// If the column index is out of bounds, returns an empty iterator.

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

    /// Get an immutable iterator over all cells in the grid.
    /// 
    /// The cells are returned in row-major order.

    pub fn flat_iter(
        &self
    ) -> impl Iterator<Item = &Cell>
    {
        self.cells.iter()
    }

    /// Get a mutable iterator over all cells in the grid.
    /// 
    /// The cells are returned in row-major order.

    pub fn flat_iter_mut(
        &mut self
    ) -> impl Iterator<Item = &mut Cell>
    {
        self.cells.iter_mut()
    }

    /// Insert a new column at the specified column index.
    /// Existing columns to the right of the inserted column will be shifted
    /// in place to the right.
    /// 
    /// Panics if the column index is out of bounds.
    /// 
    /// If the new column has fewer cells than the number of rows in the grid,
    /// the remaining cells will be filled with the default cell.
    /// 
    /// If the new column has more cells than the number of rows in the grid,
    /// the excess cells will be truncated.

    pub fn insert_col(
        &mut self,
        col_index: usize,
        new_column: impl IntoIterator<Item = impl Into<Cell>>,
    )
    {
        if col_index >= self.col_size {
            panic!("{}", ErrorMessage::COL_INDEX_OUT_OF_BOUNDS);
        }

        let mut new_column: Vec<Cell> = new_column.into_iter().map(Into::into).collect();

        // Fill or truncate the new column to match the number of rows
        new_column.resize(self.row_size, Cell::default());

        let old_col_size = self.col_size;
        let new_col_size = self.col_size + 1;
        let new_size = self.row_size * new_col_size;

        // Resize the existing cells to make room for the new column
        self.cells.resize(new_size, Cell::default());

        for ri in (0..self.row_size).rev() {
            // Iterator for the previous column indices
            let mut cols_iter = (0..self.col_size).rev();
            for ci in (0..new_col_size).rev() {
                if ci == col_index {
                    // Insert the new cell
                    self.cells[ri * new_col_size + ci] = new_column.pop().unwrap_or_default();
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
                    self.cells[ri * new_col_size + ci] =
                        std::mem::take(&mut self.cells[ri * old_col_size + old_ci]);
                }
            }
        }

        self.col_size = new_col_size;
    }


    /// Try to insert a new column at the specified column index.
    /// Existing columns to the right of the inserted column will be shifted
    /// in place to the right.
    /// 
    /// Returns an error if the column index is out of bounds.
    /// 
    /// If the new column has fewer cells than the number of rows in the grid,
    /// the remaining cells will be filled with the default cell.
    /// 
    /// If the new column has more cells than the number of rows in the grid,
    /// the excess cells will be truncated.

    pub fn try_insert_col(
        &mut self,
        col_index: usize,
        new_column: impl IntoIterator<Item = impl Into<Cell>>,
    ) -> Result<(), GridError>
    {
        if col_index > self.col_size {
            return Err(GridError::ColIndexOutOfBounds);
        }

        Ok(self.insert_col(col_index, new_column))
    }

    /// Insert a new row at the specified row index.
    /// Existing rows below the inserted row will be shifted in place downwards.
    /// 
    /// Panics if the row index is out of bounds.
    /// 
    /// If the new row has fewer cells than the number of columns in the grid,
    /// the remaining cells will be filled with the default cell.
    /// 
    /// If the new row has more cells than the number of columns in the grid,
    /// the excess cells will be truncated.

    pub fn insert_row(
        &mut self,
        row_index: usize,
        new_row: impl IntoIterator<Item = impl Into<Cell>>,
    )
    {
        if row_index > self.row_size {
            panic!("{}", ErrorMessage::ROW_INDEX_OUT_OF_BOUNDS);
        }

        let mut new_row: Vec<Cell> = new_row.into_iter().map(Into::into).collect();

        // Fill or truncate the new row to match the number of columns
        new_row.resize(self.col_size, Cell::default());

        let old_row_size = self.row_size;
        let new_row_size = self.row_size + 1;
        let new_size = new_row_size * self.col_size;

        // Resize the existing cells to make room for the new row
        self.cells.resize(new_size, Cell::default());

        for ci in (0..self.col_size).rev() {
            // Iterator for the previous row indices
            let mut rows_iter = (0..old_row_size).rev();
            for ri in (0..new_row_size).rev() {
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

        self.row_size = new_row_size;
    }

    /// Try to insert a new row at the specified row index.
    /// Existing rows below the inserted row will be shifted in place downwards.
    /// 
    /// Returns an error if the row index is out of bounds.
    /// 
    /// If the new row has fewer cells than the number of columns in the grid,
    /// the remaining cells will be filled with the default cell.
    /// 
    /// If the new row has more cells than the number of columns in the grid,
    /// the excess cells will be truncated.

    pub fn try_insert_row(
        &mut self,
        row_index: usize,
        new_row: impl IntoIterator<Item = impl Into<Cell>>,
    ) -> Result<(), GridError>
    {
        if row_index > self.row_size {
            return Err(GridError::RowIndexOutOfBounds);
        }

        Ok(self.insert_row(row_index, new_row))
    }

    /// Set the entire column at the specified index.
    /// 
    /// Panics if the index is out of bounds.
    /// 
    /// If the new column has fewer cells than the number of rows in the grid,
    /// the remaining cells will be filled with the default cell.
    /// 
    /// If the new column has more cells than the number of rows in the grid,
    /// the excess cells will be truncated.

    pub fn set_col(
        &mut self,
        col_index: usize,
        new_column: impl IntoIterator<Item = impl Into<Cell>>,
    )
    {
        if col_index >= self.col_size {
            panic!("{}", ErrorMessage::COL_INDEX_OUT_OF_BOUNDS);
        }

        let mut new_column_iter = new_column.into_iter();

        for cell in self.col_iter_mut(col_index) {
            *cell = new_column_iter
                .next()
                .map(Into::into)
                .unwrap_or_default();
        }
    }

    /// Try to set the entire column at the specified index.
    /// 
    /// Returns an error if the index is out of bounds.
    /// 
    /// If the new column has fewer cells than the number of rows in the grid,
    /// the remaining cells will be filled with the default cell.
    /// 
    /// If the new column has more cells than the number of rows in the grid,
    /// the excess cells will be truncated.

    pub fn try_set_col(
        &mut self,
        col_index: usize,
        new_column: impl IntoIterator<Item = impl Into<Cell>>,
    ) -> Result<(), GridError>
    {
        if col_index >= self.col_size {
            return Err(GridError::ColIndexOutOfBounds);
        }

        Ok(self.set_col(col_index, new_column))
    }

    /// Set the entire row at the specified index.
    /// Panics if the index is out of bounds.
    /// 
    /// If the new row has fewer cells than the number of columns in the grid,
    /// the remaining cells will be filled with the default cell.
    /// 
    /// If the new row has more cells than the number of columns in the grid,
    /// the excess cells will be truncated.

    pub fn set_row(
        &mut self,
        row_index: usize,
        new_row: impl IntoIterator<Item = impl Into<Cell>>,
    )
    {
        if row_index >= self.row_size {
            panic!("{}", ErrorMessage::ROW_INDEX_OUT_OF_BOUNDS);
        }

        let mut new_row_iter = new_row.into_iter();

        for cell in self.row_iter_mut(row_index) {
            *cell = new_row_iter
                .next()
                .map(Into::into)
                .unwrap_or_default();
        }
    }

    /// Try to set the entire row at the specified index.
    /// 
    /// Returns an error if the index is out of bounds.
    /// 
    /// If the new row has fewer cells than the number of columns in the grid,
    /// the remaining cells will be filled with the default cell.
    /// 
    /// If the new row has more cells than the number of columns in the grid,
    /// the excess cells will be truncated.
    
    pub fn try_set_row(
        &mut self,
        row_index: usize,
        new_row: impl IntoIterator<Item = impl Into<Cell>>,
    ) -> Result<(), GridError>
    {
        if row_index >= self.row_size {
            return Err(GridError::RowIndexOutOfBounds);
        }

        Ok(self.set_row(row_index, new_row))
    }

    /// Resize the grid to the specified number of rows and columns.
    /// 
    /// If the new size is larger than the current size, the empty space will be
    /// filled with the default cell.
    /// 
    /// If the new size is smaller than the current size, excess cells will be
    /// discarded.

    pub fn resize(
        &mut self,
        new_row_size: usize,
        new_col_size: usize,
    )
    {
        let old_row_size = self.row_size;
        let old_col_size = self.col_size;

        let mut new_cells = vec![Cell::default(); new_row_size * new_col_size];

        let rows = std::cmp::min(old_row_size, new_row_size);
        let cols = std::cmp::min(old_col_size, new_col_size);

        for row_index in 0..rows {
            for col_index in 0..cols {
                new_cells[row_index * new_col_size + col_index] =
                    std::mem::take(&mut self.cells[row_index * old_col_size + col_index]);
            }
        }

        self.cells = new_cells;
        self.row_size = new_row_size;
        self.col_size = new_col_size;
    }

    /// Removes all cells from the grid.
    /// 
    /// The grid will have zero rows and zero columns after this operation.

    pub fn clear(
        &mut self
    )
    {
        self.cells.clear();
        self.row_size = 0;
        self.col_size = 0;
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

