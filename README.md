# FlatGrid

A Rust library for creating and displaying tables efficiently using a flat Vector data structure.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

### Basic Examples

```rust
use flatgrid::{Grid};

fn main() {
    // Data with uneven rows and multiline content
    let data = vec![
        vec!["Name", "Age", "City", "Nickname"],
        vec!["Ms.\nAlice\nSmith", "26", "New York", "Allie"],
        vec!["Mr.\nBob\nJohnson", "27", "Los Angeles"],
        vec!["Mr.\nCharlie\nWilliams", "35", "Chicago"],
    ];

    // Construct a table
    let mut table = Grid::from(data);

    // Perform column operations
    table.col_iter_mut(1)
        .for_each(|cell| {
            cell.set_align("right");
        });

    // Perform row operations
    table.row_iter_mut(0)
        .for_each(|cell| {
            cell.set_align("center");
            cell.set_align("middle");
        });

    // Print the table
    println!("{}", &table);
}
```

Output

```
┌──────────┬─────┬─────────────┬──────────┐
│   Name   │ Age │   Country   │ Nickname │
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