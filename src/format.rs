use crate::ansi::RESET_ANSI_CODE;
use crate::color::Color;
use crate::style::FontStyleFlag;

/// Applies color and style formatting to text using ANSI escape codes.
/// 
/// This function combines foreground color, background color, and font styles
/// into a single formatted string with proper ANSI escape sequences.
/// 
/// # Arguments
/// 
/// * `text` - The text to format
/// * `fg_color` - Optional foreground color
/// * `bg_color` - Optional background color
/// * `style` - Font style flags to apply
/// 
/// # Returns
/// 
/// A `String` with the text formatted using ANSI escape codes

pub fn apply_ansi_formatting(
    text: &str,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    style: FontStyleFlag,
) -> String
{
    let mut formatted_text = String::new();
    let mut is_formatted = false;

    if let Some(color) = fg_color {
        formatted_text.push_str(color.as_fg_ansi_code());
        is_formatted = true;
    }
    if let Some(bg_color) = bg_color {
        formatted_text.push_str(bg_color.as_bg_ansi_code());
        is_formatted = true;
    }
    for style in style.into_iter() {
        formatted_text.push_str(style.as_style_ansi_code());
        is_formatted = true;
    }

    formatted_text.push_str(text);

    if is_formatted {
        formatted_text.push_str(&RESET_ANSI_CODE);
    }

    formatted_text
}
