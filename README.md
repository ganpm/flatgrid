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
use flatgrid::{Align, Grid};

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

    // Align the header row.
    for cell in grid.row_iter_mut(0) {
        cell.set_align(Align::Center | Align::Middle);
    }

    // Right-align the Age column.
    for cell in grid.col_iter_mut(1) {
        cell.set_align(Align::Right);
    }

    // Can print directly since Grid implements Display
    println!("{}", grid);
}
```

Example output:

```
 ┌──────────┬─────┬─────────────┬──────────┐ 
 │   Name   │ Age │    City     │ Nickname │ 
 ├──────────┼─────┼─────────────┼──────────┤ 
 │ Ms.      │  26 │ New York    │ Allie    │ 
 │ Alice    │     │             │          │ 
 │ Smith    │     │             │          │ 
 ├──────────┼─────┼─────────────┼──────────┤ 
 │ Mr.      │  27 │ Los Angeles │          │ 
 │ Bob      │     │             │          │ 
 │ Johnson  │     │             │          │ 
 ├──────────┼─────┼─────────────┼──────────┤ 
 │ Mr.      │  35 │ Chicago     │          │ 
 │ Charlie  │     │             │          │ 
 │ Williams │     │             │          │ 
 └──────────┴─────┴─────────────┴──────────┘ 
```

## Styling cells (ANSI)

Each cell can be formatted independently:

- `Cell::set_color(new_color)` sets foreground color
- `Cell::set_highlight(new_color)` sets background color
- `Cell::set_style(new_style)` applies styles like bold/underline

Use the exported constants to avoid typos:

- Colors: `flatgrid::Color::{RED, GREEN, BRIGHT_YELLOW, ...}`
- Styles: `flatgrid::Style::{BOLD, UNDERLINE, ITALIC, ...}`

```rust
use flatgrid::{Color, Grid, Style};

fn main() {
    let mut g = Grid::from([["status", "message"], ["ok", "all good"]]);

    // Make the header bold.
    for cell in g.row_iter_mut(0) {
        cell.set_style(Style::BOLD);
    }

    // Color the status.
    if let Some(cell) = g.get_cell_mut(1, 0) {
        cell.set_color(Color::GREEN);
    }

    println!("{}", g);
}
```

Notes:

- Formatting is applied using ANSI escape codes and is reset after each formatted segment.
- Your terminal must support ANSI (Windows Terminal / modern PowerShell do).
- Invalid colors and codes will be ignored and will have no effect on the output format.

## Working with the grid

### Construction

- `Grid::new(row_size, col_size)` creates a fixed-size grid of default cells
- `Grid::from(data)` accepts any nested iterator of values convertible into `Cell` (any data type can be converted into a `Cell` if it also implements `Display`)

`flatgrid` also exports a small convenience macro:

```rust
use flatgrid::{grid, Grid};

let empty: Grid = grid!();
let from_data = grid!([["a", "b"], ["c", "d"]]);
let sized = grid!(3, 4);
```

### Iteration

- `row_iter(row_index)` / `row_iter_mut(row_index)`
- `col_iter(col_index)` / `col_iter_mut(col_index)`
- `flat_iter()` / `flat_iter_mut()`

Out-of-bounds `row_iter*` / `col_iter*` return empty iterators.

### Mutation

- `set_cell(row_index, col_index, value)` (panics if out of bounds)
- `set_row(row_index, values)` / `set_col(col_index, values)` (panics if length mismatches)
- `insert_row(row_index, values)` / `insert_col(col_index, values)` (panics if length mismatches)
- `resize(new_rows, new_cols)` preserves the top-left overlap and drops the rest if the new size is smaller than the previous and pads with the default if the new size is larger

## Limitations

These are currently planned future improvements:

- Width calculation currently uses `str::len()`. This means wide Unicode graphemes (CJK, emoji) may misalign.
- `Cell` truncation is byte-based when a line is wider than the target width.
- Table borders are Unicode box-drawing characters; if your font doesn’t support them, the output may look odd.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.