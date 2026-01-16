use std::ops::{BitOr, BitOrAssign};
use crate::ansi::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FontStyle(u8);

#[allow(non_upper_case_globals)]
impl FontStyle {

    // u8 representation of each font style as bit flags
    pub(crate) const BOLD      : u8 = 1 << 0;
    pub(crate) const DIM       : u8 = 1 << 1;
    pub(crate) const ITALIC    : u8 = 1 << 2;
    pub(crate) const UNDERLINE : u8 = 1 << 3;
    pub(crate) const BLINK     : u8 = 1 << 4;
    pub(crate) const REVERSE   : u8 = 1 << 5;
    pub(crate) const HIDDEN    : u8 = 1 << 6;
    pub(crate) const STRIKE    : u8 = 1 << 7;

    pub(crate) const COUNT : usize = 8;

    pub(crate) const ALL_FLAGS : [u8; FontStyle::COUNT] = [
        FontStyle::BOLD,
        FontStyle::DIM,
        FontStyle::ITALIC,
        FontStyle::UNDERLINE,
        FontStyle::BLINK,
        FontStyle::REVERSE,
        FontStyle::HIDDEN,
        FontStyle::STRIKE,
    ];

    // Constants for easy access to individual font styles
    pub const Bold      : FontStyle = FontStyle(FontStyle::BOLD);
    pub const Dim       : FontStyle = FontStyle(FontStyle::DIM);
    pub const Italic    : FontStyle = FontStyle(FontStyle::ITALIC);
    pub const Underline : FontStyle = FontStyle(FontStyle::UNDERLINE);
    pub const Blink     : FontStyle = FontStyle(FontStyle::BLINK);
    pub const Reverse   : FontStyle = FontStyle(FontStyle::REVERSE);
    pub const Hidden    : FontStyle = FontStyle(FontStyle::HIDDEN);
    pub const Strike    : FontStyle = FontStyle(FontStyle::STRIKE);

    pub fn new() -> FontStyle {
        FontStyle(0)
    }

    /// Returns the ANSI code string for the given font style.
    /// 
    /// # Returns
    /// 
    /// A string slice representing the ANSI escape code for the font style.

    pub(crate) fn as_style_ansi_code(
        &self
    ) -> &'static str
    {
        match self.0 {
            FontStyle::BOLD      => BOLD_ANSI_CODE,
            FontStyle::DIM       => DIM_ANSI_CODE,
            FontStyle::ITALIC    => ITALIC_ANSI_CODE,
            FontStyle::UNDERLINE => UNDERLINE_ANSI_CODE,
            FontStyle::BLINK     => BLINK_ANSI_CODE,
            FontStyle::REVERSE   => REVERSE_ANSI_CODE,
            FontStyle::HIDDEN    => HIDDEN_ANSI_CODE,
            FontStyle::STRIKE    => STRIKE_ANSI_CODE,
            _ => "",
        }
    }

}

impl BitOr for FontStyle {

    type Output = FontStyle;

    fn bitor(
        self,
        rhs: FontStyle
    ) -> FontStyle
    {
        FontStyle(self.0 | rhs.0)
    }

}

impl BitOrAssign for FontStyle {

    fn bitor_assign(
        &mut self,
        rhs: FontStyle
    ) {
        self.0 |= rhs.0;
    }

}


/// An iterator for `FontStyle` that yields each active font style.
/// 
/// This iterator allows you to iterate over the individual font styles that are currently set in a `FontStyleFlag` instance.

pub struct FontStyleIter {
    flag: u8,
    index: usize,
}

impl Iterator for FontStyleIter {

    type Item = FontStyle;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < FontStyle::COUNT {
            let flag = FontStyle::ALL_FLAGS[self.index];
            self.index += 1;
            if self.flag & flag != 0 {
                return Some(FontStyle(flag));
            }
        }
        None
    }

}


impl IntoIterator for FontStyle {

    type Item = FontStyle;
    type IntoIter = FontStyleIter;

    fn into_iter(self) -> Self::IntoIter {
        FontStyleIter {
            flag: self.0,
            index: 0,
        }
    }

}