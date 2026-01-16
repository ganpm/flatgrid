# flatgrid

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

`flatgrid` is a small Rust crate for rendering terminal-friendly tables using a flat vector storage model.

## Features

- Multiline cell content (cells can contain `\n`)
- Per-cell horizontal + vertical alignment
- Optional ANSI formatting (foreground, background, and common font styles)
- Can set cell width and cell height optionally (will truncate and pad automatically)
- Basic grid mutations (resize, set cell/row/col, insert row/col, iterate row/col/flat)
- Unicode box-drawing symbols for borders
- Quick table construction by converting a nested `Vec`
- Zero dependencies

## Quickstart

The following example is the primary use case for this library.

```rust
use flatgrid::*;

fn main() {
    // Uneven rows are allowed: missing cells are filled with defaults.
    // Multiline content is supported via `\n`.
    let data = vec![
        vec!["Name", "Age", "City", "Nickname"],
        vec!["Ms.\nAlice\nSmith", "26", "New York", "Allie"],
        vec!["Mr.\nBob\nJohnson", "27", "Los Angeles"],
        vec!["Mr.\nCharlie\nWilliams", "35", "Chicago"],
    ];

    let mut grid = Grid::from(data);

    // Format the Age column.
    for cell in grid.col_iter_mut(1) {
        cell.set_width(5); // Set width
        cell.set_align(Align::Right);  // Right-align
    }

    // Format the top row.
    for cell in grid.row_iter_mut(0) {
        cell.set_height(3);  // Set height
        cell.set_align(Align::Center | Align::Middle);  // Center middle align
    }

    // Print
    println!("{}", grid);
}
```

Example output:

```
 ┌──────────┬───────┬─────────────┬──────────┐
 │          │       │             │          │
 │   Name   │  Age  │    City     │ Nickname │
 │          │       │             │          │
 ├──────────┼───────┼─────────────┼──────────┤
 │ Ms.      │    26 │ New York    │ Allie    │
 │ Alice    │       │             │          │
 │ Smith    │       │             │          │
 ├──────────┼───────┼─────────────┼──────────┤
 │ Mr.      │    27 │ Los Angeles │          │
 │ Bob      │       │             │          │
 │ Johnson  │       │             │          │
 ├──────────┼───────┼─────────────┼──────────┤
 │ Mr.      │    35 │ Chicago     │          │
 │ Charlie  │       │             │          │
 │ Williams │       │             │          │
 └──────────┴───────┴─────────────┴──────────┘
```

## Formatting Cells

Each cell can be formatted independently:

- `Cell::set_align(new_align)` sets vertical and horizontal alignment
- `Cell::set_color(new_color)` sets foreground color
- `Cell::set_highlight(new_color)` sets background color
- `Cell::set_style(new_style)` applies styles like bold/underline
- `Cell::set_height(new_height)`/`Cell::set_width(new_width)` when set, ignores automatic height calculation and truncates/pads the cell to the new size

Example:
```rust
use flatgrid::*;

fn main() {
    let data = vec![
        vec!["User", "Status"],
        vec!["Alice", "Online"],
        vec!["Bob", "Offline"],
        vec!["Charlie", "Away"],
        vec!["Diana", "Busy"],
    ];

    let mut grid = Grid::from(data);

    // Set header row to bold and center-aligned
    for cell in grid.row_iter_mut(0) {
        cell.set_style(FontStyle::Bold);
        cell.set_align(Align::Center);
    }

    // Set different styles and colors for each user's status

    // Online - Green Bold
    grid.get_cell_mut(1, 1).map(|cell| {
        cell.set_style(FontStyle::Bold);
        cell.set_color(Color::GREEN);
    });

    // Offline - Red Bold
    grid.get_cell_mut(2, 1).map(|cell| {
        cell.set_style(FontStyle::Bold);
        cell.set_color(Color::RED);
    });

    // Away - Yellow Italic
    grid.get_cell_mut(3, 1).map(|cell| {
        cell.set_style(FontStyle::Italic);
        cell.set_color(Color::YELLOW);
    });

    // Busy - Magenta Underline
    grid.get_cell_mut(4, 1).map(|cell| {
        cell.set_style(FontStyle::Underline);
        cell.set_color(Color::MAGENTA);
    });

    // Print
    println!("{}", grid);
}
```

Notes:

- Formatting is applied using ANSI escape codes and is reset after each formatted segment.
- Your terminal must support ANSI (Windows Terminal / modern PowerShell do).
- Invalid colors and codes will be ignored and will have no effect on the output format.

## Overview

### Construction

- `Grid::new(row_size, col_size)` creates a fixed-size grid of default cells
- `Grid::from(data)` accepts any nested iterator of values convertible into `Cell` (any data type can be converted into a `Cell` if it also implements `Display`)

`flatgrid` also exports a small convenience macro `grid!()`:

```rust
use flatgrid::{grid, Grid};

let empty: Grid = grid!();
let from_data = grid!([["a", "b"], ["c", "d"]]);
let sized = grid!(3, 4);
```

### Accessing Cells

- `Grid::get_cell(row_index, col_index)`
- `Grid::get_cell_mut(row_index, col_index)`

Out-of-bounds indices return `None`

### Accessing Rows and Columns

- `Grid::row_iter(row_index)`
- `Grid::row_iter_mut(row_index)`
- `Grid::col_iter(col_index)`
- `Grid::col_iter_mut(col_index)`

Out-of-bounds indices will return empty iterators.

### Accessing All Cells

- `Grid::flat_iter()`
- `Grid::flat_iter_mut()`

The flattened iterators are in row-major order.

### Mutation

- `set_cell(row_index, col_index, cell_data)`
- `set_row(row_index, new_row)`
- `set_col(col_index, new_column)`
- `insert_row(row_index, new_row)` (in-place shift)
- `insert_col(col_index, new_column)` (in-place shift)
- `set_cells(new_cells)`
- `resize(new_rows, new_cols)` (preserves the top-left overlap)

Panics if indices are out of bounds. For multi-cell mutators, mismatched input dimensions are truncated or padded with empty cells to fit grid dimensions.

The following variants of the above functions return `Result` instead of panicking.

- `try_set_cell(row_index, col_index, cell_data)`
- `try_set_row(row_index, new_row)`
- `try_set_col(col_index, new_column)`
- `try_insert_row(row_index, new_row)`
- `try_insert_col(col_index, new_column)`

## Limitations

These are currently planned future improvements:

- Width calculation currently uses `str::len()`. This means wide Unicode graphemes (CJK, emoji) may misalign.
- `Cell` truncation is byte-based when a line is wider than the target width.
- Table borders are Unicode box-drawing characters; if your font doesn’t support them, the output may look odd.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.