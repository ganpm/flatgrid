use crate::align::{AlignH, AlignV, Align, AlignString};
use crate::color::{Foreground, Background};
use crate::style::{FontStyle, FontStyleFlag};
use crate::format::apply_ansi_formatting;

use std::fmt::Display;

/// A single cell in the grid.
/// 
/// Cells can contain multiline text and support various formatting options
/// such as alignment, foreground and background colors, and font styles.

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cell {
    data: String,
    h_align: Option<AlignH>,
    v_align: Option<AlignV>,
    fg_color: Option<Foreground>,
    bg_color: Option<Background>,
    font_style: FontStyleFlag,
    width: Option<usize>,
    height: Option<usize>,
}


impl Cell {

    /// Creates a new cell with the given data.
    /// 
    /// Data can be any type that implements the `Into<String>` trait.

    pub fn new(
        data: impl Into<String>,
    ) -> Self
    {
        Cell {
            data: data.into(),
            h_align: None,
            v_align: None,
            fg_color: None,
            bg_color: None,
            font_style: FontStyleFlag::new(),
            width: None,
            height: None,
        }
    }

    /// Sets the data of the cell.
    /// 
    /// The data can be any type that implements the `Into<String>` trait.

    pub fn set_data(
        &mut self,
        new_data: impl Into<String>
    )
    {
        self.data = new_data.into();
    }

    /// Gets an immutable reference to the cell's data.

    pub fn get_data(
        &self
    ) -> &str
    {
        &self.data
    }

    /// Gets a mutable reference to the cell's data.

    pub fn get_data_mut(
        &mut self
    ) -> &mut str
    {
        &mut self.data
    }

    pub(crate) fn height(
        &self
    ) -> usize
    {
        if let Some(height) = self.height {
            return height;
        }
        self.data.lines().count()
    }

    /// Sets the height of the cell.
    /// 
    /// Setting a height will override the automatic height calculation
    /// based on the content.

    pub fn set_height(
        &mut self,
        new_height: usize
    )
    {
        self.height = Some(new_height);
    }

    /// Clears any manually set height, reverting to automatic height calculation.

    pub fn clear_height(
        &mut self
    )
    {
        self.height = None;
    }

    pub(crate) fn width(
        &self
    ) -> usize
    {
        if let Some(width) = self.width {
            return width;
        }
        self.data.lines()
            .map(|line| line.len())
            .max()
            .unwrap_or(0)
    }

    /// Sets the width of the cell.
    /// 
    /// Setting a width will override the automatic width calculation
    /// based on the content.

    pub fn set_width(
        &mut self,
        new_width: usize
    )
    {
        self.width = Some(new_width);
    }

    /// Clears any manually set width, reverting to automatic width calculation.

    pub fn clear_width(
        &mut self
    )
    {
        self.width = None;
    }

    /// Sets the alignment of the cell's content.
    /// 
    /// The `new_align` parameter should be one of the following strings:
    /// - "top", "bottom", "middle" for vertical alignment
    /// - "left", "right", "center" for horizontal alignment
    /// 
    /// Unrecognized alignment strings will be ignored.

    pub fn set_align_from(
        &mut self,
        new_align: &str,
    )
    {
        match new_align {
            AlignString::TOP     => self.v_align = Some(AlignV::Top),
            AlignString::BOTTOM  => self.v_align = Some(AlignV::Bottom),
            AlignString::MIDDLE  => self.v_align = Some(AlignV::Middle),
            AlignString::LEFT    => self.h_align = Some(AlignH::Left),
            AlignString::RIGHT   => self.h_align = Some(AlignH::Right),
            AlignString::CENTER  => self.h_align = Some(AlignH::Center),
            _              => {/* Ignore unrecognized alignments */},
        }
    }

    /// Sets the alignment of the cell's content.
    /// 
    /// The `new_align` parameter should be an `Align` value
    /// combining horizontal and vertical alignment options using bitwise OR.
    /// 
    /// If conflicting horizontal or vertical alignments are set,
    /// the last one for each axis takes precedence.

    pub fn set_align(
        &mut self,
        new_align: Align
    )
    {
        self.h_align = new_align.get_h();
        self.v_align = new_align.get_v();
    }

    /// Sets the foreground color of the cell's text.
    /// 
    /// The `new_color` parameter should be a valid color string
    /// that can be parsed by the `Foreground::from_str` method.
    /// 
    /// Unrecognized color strings will result in no color being set.

    pub fn set_color(
        &mut self,
        new_color: &str,
    )
    {
        self.fg_color = Foreground::from_str(new_color);
    }

    /// Sets the background color (highlight) of the cell's text.
    /// 
    /// The `new_color` parameter should be a valid color string
    /// that can be parsed by the `Background::from_str` method.
    /// 
    /// Unrecognized color strings will result in no color being set.

    pub fn set_highlight(
        &mut self,
        new_color: &str,
    )
    {
        self.bg_color = Background::from_str(new_color);
    }

    /// Sets the font style of the cell's text.
    ///
    /// The `new_style` parameter should be a valid style string
    /// that can be parsed by the `FontStyle::from_str` method.
    /// 
    /// Unrecognized style strings will result in no style being set.

    pub fn set_style_from(
        &mut self,
        new_style: &str,
    )
    {
        if let Some(style) = FontStyle::from_str(new_style) {
            self.font_style.set(style.as_flag());
        }
    }

    /// Removes all formatting from the cell, resetting it to default state.
    /// 
    /// This includes clearing alignment, colors, and font styles.

    pub fn remove_format(
        &mut self
    )
    {
        self.h_align = None;
        self.v_align = None;
        self.fg_color = None;
        self.bg_color = None;
        self.font_style.reset();
    }

    pub(crate) fn render_lines(
        &self,
        target_cell_height: usize,
        target_cell_width: usize,
    ) -> Vec<String>
    {
        let mut visible_lens = self.data.lines()
            .map(|line| line.len());
        let data_lines = self.data.lines()
            .map(|line| apply_ansi_formatting(line, self.fg_color, self.bg_color, self.font_style));

        let height = self.data.lines().count();
    
        let v_align = self.v_align.unwrap_or_default();
        let h_align = self.h_align.unwrap_or_default();

        let pad_count = target_cell_height.saturating_sub(height);
        let pad_string = " ".repeat(target_cell_width);

        // Pre-allocate first since we know the size
        let mut lines = Vec::with_capacity(target_cell_height);

        // Add top padding
        match v_align {
            AlignV::Top => {},
            AlignV::Bottom => {
                lines.extend(std::iter::repeat(pad_string.clone()).take(pad_count));
            },
            AlignV::Middle => {
                lines.extend(std::iter::repeat(pad_string.clone()).take(pad_count / 2));
            },
        }

        // Add content lines
        for line in data_lines {
            let visible_len = visible_lens.next().unwrap_or(0);
            lines.push(
                if visible_len < target_cell_width {
                    // Apply horizontal alignment
                    let width = target_cell_width + line.len() - visible_len;
                    match h_align {
                        AlignH::Left   => format!("{:<width$}", line, width = width),
                        AlignH::Right  => format!("{:>width$}", line, width = width),
                        AlignH::Center => format!("{:^width$}", line, width = width),
                    }
                } else if visible_len == target_cell_width {
                    line
                } else {
                    // Truncate the line to fit the target width
                    line[..target_cell_width].to_string()
                }
            );
        }

        // Add bottom padding
        match v_align {
            AlignV::Top => {
                lines.extend(std::iter::repeat(pad_string).take(pad_count));
            },
            AlignV::Bottom => {},
            AlignV::Middle => {
                lines.extend(std::iter::repeat(pad_string).take(pad_count - pad_count / 2));
            },
        }

        lines
    }

}


impl<T> From<T> for Cell
where 
    T: Display,
{

    fn from(value: T) -> Self {
        Cell::new(value.to_string())
    }

}

