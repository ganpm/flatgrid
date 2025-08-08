/// Internal utility struct for generating table borders using Unicode box-drawing characters.
/// 
/// This struct provides constants and methods for creating the various border elements
/// needed to draw table frames, including corners, edges, and intersections.

#[derive(Debug, Clone)]
pub struct Border {}

impl Border {

    const TOP_LEFT      : &'static str = " ┌─";
    const TOP_MIDDLE    : &'static str = "─┬─";
    const TOP_RIGHT     : &'static str = "─┐ ";
    const MIDDLE_LEFT   : &'static str = " ├─";
    const MIDDLE_MIDDLE : &'static str = "─┼─";
    const MIDDLE_RIGHT  : &'static str = "─┤ ";
    const BOTTOM_LEFT   : &'static str = " └─";
    const BOTTOM_MIDDLE : &'static str = "─┴─";
    const BOTTOM_RIGHT  : &'static str = "─┘ ";
    const VERTICAL      : &'static str = " │ ";
    const HORIZONTAL    : &'static str = "─";

    /// Creates a border line with the specified corner and intersection characters.
    /// 
    /// # Arguments
    /// 
    /// * `column_widths` - Vector of column widths
    /// * `horizontal_fill` - Horizontal fill character
    /// * `rightmost` - Right corner/intersection character
    /// * `middle` - Middle intersection character
    /// * `leftmost` - Left corner/intersection character
    /// 
    /// # Returns
    ///
    /// A formatted border string

    fn render_border(
        column_widths: &Vec<usize>,
        horizontal_fill: &str,
        rightmost: &str,
        middle: &str,
        leftmost: &str,
    ) -> String
    {
        let middle = column_widths.iter()
            .map(|width| horizontal_fill.repeat(*width))
            .collect::<Vec<String>>()
            .join(middle)
            .to_string();
        format!("{}{}{}", leftmost, middle, rightmost)
    }

    /// Creates the top border of the table.
    /// 
    /// # Arguments
    /// 
    /// * `column_widths` - Vector of column widths
    /// 
    /// # Returns
    /// 
    /// A formatted top border string

    pub fn render_top_border(
        column_widths: &Vec<usize>,
    ) -> String
    {
        Border::render_border(
            column_widths,
            Border::HORIZONTAL,
            Border::TOP_RIGHT,
            Border::TOP_MIDDLE,
            Border::TOP_LEFT,
        )
    }

    /// Creates a middle border (separator between rows).
    /// 
    /// # Arguments
    /// 
    /// * `column_widths` - Vector of column widths
    /// 
    /// # Returns
    /// 
    /// A formatted middle border string

    pub fn render_mid_border(
        column_widths: &Vec<usize>,
    ) -> String
    {
        Border::render_border(
            column_widths,
            Border::HORIZONTAL,
            Border::MIDDLE_RIGHT,
            Border::MIDDLE_MIDDLE,
            Border::MIDDLE_LEFT,
        )
    }

    /// Creates the bottom border of the table.
    /// 
    /// # Arguments
    /// 
    /// * `column_widths` - Vector of column widths
    /// 
    /// # Returns
    /// 
    /// A formatted bottom border string

    pub fn render_bot_border(
        column_widths: &Vec<usize>,
    ) -> String
    {
        Border::render_border(
            column_widths,
            Border::HORIZONTAL,
            Border::BOTTOM_RIGHT,
            Border::BOTTOM_MIDDLE,
            Border::BOTTOM_LEFT,
        )
    }

    /// Renders every cell in a row, line by line, with vertical separators between columns.
    /// 
    /// # Arguments
    /// 
    /// * `lines` - Vector of formatted line strings
    /// 
    /// # Returns
    /// 
    /// A formatted text row string

    pub fn render_row_lines(
        lines: Vec<String>
    ) -> String
    {
        let vertical = Border::VERTICAL.to_string();
        let text = lines
            .iter()
            .cloned()
            .collect::<Vec<String>>()
            .join(&vertical);
        format!("{}{}{}", vertical, text, vertical)
    }

}
