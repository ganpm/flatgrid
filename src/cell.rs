use crate::align::{HAlign, VAlign, Align};
use crate::color::Color;
use crate::style::{FontStyle, FontStyleFlag};
use crate::format::apply_ansi_formatting;

use std::fmt::Display;


#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cell {
    data: String,
    h_align: Option<HAlign>,
    v_align: Option<VAlign>,
    fg_color: Option<Color>,
    bg_color: Option<Color>,
    font_style: FontStyleFlag,
    width: Option<usize>,
    height: Option<usize>,
}


impl Cell {

    pub fn new(
        data: String
    ) -> Self
    {
        Cell {
            data,
            h_align: None,
            v_align: None,
            fg_color: None,
            bg_color: None,
            font_style: FontStyleFlag::new(),
            width: None,
            height: None,
        }
    }

    pub fn get_data(
        &self
    ) -> &String
    {
        &self.data
    }

    pub fn get_data_mut(
        &mut self
    ) -> &mut String
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

    pub fn set_height(
        &mut self,
        height: usize
    )
    {
        self.height = Some(height);
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

    pub fn set_width(
        &mut self,
        width: usize
    )
    {
        self.width = Some(width);
    }

    pub fn set_align(
        &mut self,
        align: &str,
    )
    {
        match Align::from_str(align) {
            Some(Align::HAlign(h_align)) => {
                self.h_align = Some(h_align);
            },
            Some(Align::VAlign(v_align)) => {
                self.v_align = Some(v_align);
            },
            None => {},
        }
    }

    pub fn set_color(
        &mut self,
        color: &str,
    )
    {
        self.fg_color = Color::from_str(color);
    }

    pub fn set_highlight(
        &mut self,
        color: &str,
    )
    {
        self.bg_color = Color::from_str(color);
    }

    pub fn set_style(
        &mut self,
        style: &str,
    )
    {
        if let Some(style) = FontStyle::from_str(style) {
            self.font_style.set(style.flag());
        }
    }

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

        let pad_count = target_cell_height.saturating_sub(height);
        let pad_string = " ".repeat(target_cell_width);

        // Pre-allocate first since we know the size
        let mut lines = Vec::with_capacity(target_cell_height);

        // Add top padding
        match v_align {
            VAlign::Top => {},
            VAlign::Bottom => {
                lines.extend(std::iter::repeat(pad_string.clone()).take(pad_count));
            },
            VAlign::Middle => {
                lines.extend(std::iter::repeat(pad_string.clone()).take(pad_count / 2));
            },
        }

        // Add content lines
        for line in data_lines {
            let visible_len = visible_lens.next().unwrap_or(0);
            let formatted_line = if visible_len < target_cell_width {
                let width = target_cell_width + line.len() - visible_len;

                // Apply horizontal alignment
                let h_align = self.h_align.unwrap_or_default();
                match h_align {
                    HAlign::Left   => format!("{:<width$}", line, width = width),
                    HAlign::Right  => format!("{:>width$}", line, width = width),
                    HAlign::Center => format!("{:^width$}", line, width = width),
                }
            } else if visible_len == target_cell_width {
                line
            } else {
                // Truncate the line to fit the target width
                let truncated = &line[..target_cell_width];
                let h_align = self.h_align.unwrap_or_default();
                match h_align {
                    HAlign::Left   => format!("{:<width$}", truncated, width = target_cell_width),
                    HAlign::Right  => format!("{:>width$}", truncated, width = target_cell_width),
                    HAlign::Center => format!("{:^width$}", truncated, width = target_cell_width),
                }
            };
            lines.push(formatted_line);
        }

        // Add bottom padding
        match v_align {
            VAlign::Top => {
                lines.extend(std::iter::repeat(pad_string).take(pad_count));
            },
            VAlign::Bottom => {},
            VAlign::Middle => {
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

