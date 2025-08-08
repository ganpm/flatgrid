

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


#[macro_export]
macro_rules! grid {
    () => {
        $crate::Grid::default()
    };
    ($data:expr) => {
        $crate::Grid::from($data)
    };
    ($width:expr, $height:expr) => {
        $crate::Grid::new($width, $height)
    };
}
