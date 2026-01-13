use crate::align::{AlignH, AlignV, Align};
use crate::color::{Foreground, Background};
use crate::style::{FontStyle, FontStyleFlag};
use crate::format::apply_ansi_formatting;

use std::fmt::Display;


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

    pub fn set_data(
        &mut self,
        new_data: String
    )
    {
        self.data = new_data;
    }

    pub fn get_data(
        &self
    ) -> &str
    {
        &self.data
    }

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

    pub fn set_height(
        &mut self,
        new_height: usize
    )
    {
        self.height = Some(new_height);
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
        new_width: usize
    )
    {
        self.width = Some(new_width);
    }

    pub fn set_align(
        &mut self,
        new_align: &str,
    )
    {
        match new_align {
            Align::TOP     => self.v_align = Some(AlignV::Top),
            Align::BOTTOM  => self.v_align = Some(AlignV::Bottom),
            Align::MIDDLE  => self.v_align = Some(AlignV::Middle),
            Align::LEFT    => self.h_align = Some(AlignH::Left),
            Align::RIGHT   => self.h_align = Some(AlignH::Right),
            Align::CENTER  => self.h_align = Some(AlignH::Center),
            _              => {/* Ignore unrecognized alignments */},
        }
    }

    pub fn set_color(
        &mut self,
        new_color: &str,
    )
    {
        self.fg_color = Foreground::from_str(new_color);
    }

    pub fn set_highlight(
        &mut self,
        new_color: &str,
    )
    {
        self.bg_color = Background::from_str(new_color);
    }

    pub fn set_style(
        &mut self,
        new_style: &str,
    )
    {
        if let Some(style) = FontStyle::from_str(new_style) {
            self.font_style.set(style.as_flag());
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

