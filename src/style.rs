use crate::ansi::*;

pub const BOLD     : &str = "bold";
pub const DIM      : &str = "dim";
pub const ITALIC   : &str = "italic";
pub const UNDERLINE: &str = "underline";
pub const BLINK    : &str = "blink";
pub const REVERSE  : &str = "reverse";
pub const HIDDEN   : &str = "hidden";
pub const STRIKE   : &str = "strike";


/// A bitfield flag structure for managing multiple font styles simultaneously.
/// 
/// This struct uses a single byte to efficiently store multiple font style flags,
/// allowing combinations of styles like bold and italic to be applied together.
/// Each bit in the flag byte represents a different font style.

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FontStyleFlag {
    flag: u8,
}

impl FontStyleFlag {

    pub const BOLD     : u8 = 1 << 0;
    pub const DIM      : u8 = 1 << 1;
    pub const ITALIC   : u8 = 1 << 2;
    pub const UNDERLINE: u8 = 1 << 3;
    pub const BLINK    : u8 = 1 << 4;
    pub const REVERSE  : u8 = 1 << 5;
    pub const HIDDEN   : u8 = 1 << 6;
    pub const STRIKE   : u8 = 1 << 7;

    pub const COUNT : usize = 8;

    pub const ALL_FLAGS: [u8; 8] = [
        FontStyleFlag::BOLD,
        FontStyleFlag::DIM,
        FontStyleFlag::ITALIC,
        FontStyleFlag::UNDERLINE,
        FontStyleFlag::BLINK,
        FontStyleFlag::REVERSE,
        FontStyleFlag::HIDDEN,
        FontStyleFlag::STRIKE,
    ];

    /// Creates a new `FontStyleFlag` with no styles set.
    /// 
    /// # Returns
    /// 
    /// A new `FontStyleFlag` instance with all style flags cleared.

    pub fn new(
    ) -> FontStyleFlag
    {
        FontStyleFlag { flag: 0 }
    }

    /// Sets the specified font style flag(s).
    /// 
    /// # Arguments
    /// 
    /// * `flag` - The flag(s) to set, can be combined with bitwise OR

    pub fn set(
        &mut self,
        flag: u8,
    )
    {
        self.flag |= flag;
    }

    /// Resets all font style flags to their default (unset) state.
    /// 
    /// This function clears all active font style flags, effectively removing any
    /// formatting that was previously applied.

    pub fn reset(
        &mut self
    )
    {
        self.flag = 0;
    }

}

impl IntoIterator for FontStyleFlag {

    type Item = FontStyle;
    type IntoIter = FontStyleFlagIter;

    fn into_iter(self) -> Self::IntoIter {
        FontStyleFlagIter {
            flag: self.flag,
            index: 0,
        }
    }

}

/// An iterator for `FontStyleFlag` that yields each active font style.
/// 
/// This iterator allows you to iterate over the individual font styles that are currently set in a `FontStyleFlag` instance.

pub struct FontStyleFlagIter {
    flag: u8,
    index: usize,
}

impl Iterator for FontStyleFlagIter {

    type Item = FontStyle;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < FontStyleFlag::COUNT {
            let flag = FontStyleFlag::ALL_FLAGS[self.index];
            self.index += 1;
            if self.flag & flag != 0 {
                return Some(FontStyle::from_flag(flag));
            }
        }
        None
    }

}

/// Represents individual font style options that can be applied to text.
/// 
/// These styles correspond to common terminal text formatting options and
/// can be combined using the `FontStyleFlag` structure for multiple effects.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontStyle {
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Reverse,
    Hidden,
    Strike,
}

impl FontStyle {

    /// Returns the bit flag value for this font style.
    /// 
    /// # Returns
    /// 
    /// A `u8` value representing the bit flag for this style.

    pub fn flag(
        &self,
    ) -> u8
    {
        match self {
            FontStyle::Bold      => FontStyleFlag::BOLD,
            FontStyle::Dim       => FontStyleFlag::DIM,
            FontStyle::Italic    => FontStyleFlag::ITALIC,
            FontStyle::Underline => FontStyleFlag::UNDERLINE,
            FontStyle::Blink     => FontStyleFlag::BLINK,
            FontStyle::Reverse   => FontStyleFlag::REVERSE,
            FontStyle::Hidden    => FontStyleFlag::HIDDEN,
            FontStyle::Strike    => FontStyleFlag::STRIKE,
        }
    }

    /// Converts a bit flag to its corresponding `FontStyle`.
    /// 
    /// # Arguments
    /// 
    /// * `flag` - The bit flag to convert
    /// 
    /// # Returns
    /// 
    /// The corresponding `FontStyle` enum variant, or `None` if the flag is invalid.
    /// 
    
    pub fn from_flag(
        flag: u8,
    ) -> FontStyle
    {
        match flag {
            FontStyleFlag::BOLD      => FontStyle::Bold,
            FontStyleFlag::DIM       => FontStyle::Dim,
            FontStyleFlag::ITALIC    => FontStyle::Italic,
            FontStyleFlag::UNDERLINE => FontStyle::Underline,
            FontStyleFlag::BLINK     => FontStyle::Blink,
            FontStyleFlag::REVERSE   => FontStyle::Reverse,
            FontStyleFlag::HIDDEN    => FontStyle::Hidden,
            FontStyleFlag::STRIKE    => FontStyle::Strike,
            _                        => panic!("Invalid font style flag"),
        }
    }

    pub fn from_str(
        style: &str,
    ) -> Option<Self>
    {
        match style {
            BOLD      => Some(Self::Bold),
            DIM       => Some(Self::Dim),
            ITALIC    => Some(Self::Italic),
            UNDERLINE => Some(Self::Underline),
            BLINK     => Some(Self::Blink),
            REVERSE   => Some(Self::Reverse),
            HIDDEN    => Some(Self::Hidden),
            STRIKE    => Some(Self::Strike),
            _         => None,
        }
    }

    /// Converts the font style to its ANSI escape code.
    /// 
    /// # Returns
    /// 
    /// A `String` containing the ANSI escape sequence for this style.

    pub fn as_style_ansi_code(
        &self,
    ) -> &str
    {
        match self {
            FontStyle::Bold      => BOLD_ANSI_CODE,
            FontStyle::Dim       => DIM_ANSI_CODE,
            FontStyle::Italic    => ITALIC_ANSI_CODE,
            FontStyle::Underline => UNDERLINE_ANSI_CODE,
            FontStyle::Blink     => BLINK_ANSI_CODE,
            FontStyle::Reverse   => REVERSE_ANSI_CODE,
            FontStyle::Hidden    => HIDDEN_ANSI_CODE,
            FontStyle::Strike    => STRIKE_ANSI_CODE,
        }
    }

}
