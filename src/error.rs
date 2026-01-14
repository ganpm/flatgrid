
pub enum GridError {
    RowIndexOutOfBounds,
    ColIndexOutOfBounds,
    RowAndColIndexOutOfBounds,
}

pub struct ErrorMessage;

impl ErrorMessage {
    pub const ROW_INDEX_OUT_OF_BOUNDS: &'static str = "Row index out of bounds";
    pub const COL_INDEX_OUT_OF_BOUNDS: &'static str = "Column index out of bounds";
    pub const ROW_AND_COL_INDEX_OUT_OF_BOUNDS: &'static str = "Row and column index out of bounds";
}
