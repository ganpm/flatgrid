

mod cell;
mod grid;
mod align;
mod border;
mod ansi;
mod color;
mod style;
mod format;

pub use cell::Cell;
pub use grid::Grid;
pub use align::Align;
pub use color::Color;
pub use style::Style;


#[macro_export]
macro_rules! grid {
    () => {
        $crate::Grid::default()
    };
    ($data:expr) => {
        $crate::Grid::from($data)
    };
    ($rows:expr, $cols:expr) => {
        $crate::Grid::new($rows, $cols)
    };
}
