use crate::ansi::*;


pub const BLACK              : &str = "black";
pub const RED                : &str = "red";
pub const GREEN              : &str = "green";
pub const YELLOW             : &str = "yellow";
pub const BLUE               : &str = "blue";
pub const MAGENTA            : &str = "magenta";
pub const CYAN               : &str = "cyan";
pub const WHITE              : &str = "white";
pub const BRIGHT_BLACK       : &str = "bright black";
pub const BRIGHT_RED         : &str = "bright red";
pub const BRIGHT_GREEN       : &str = "bright green";
pub const BRIGHT_YELLOW      : &str = "bright yellow";
pub const BRIGHT_BLUE        : &str = "bright blue";
pub const BRIGHT_MAGENTA     : &str = "bright magenta";
pub const BRIGHT_CYAN        : &str = "bright cyan";
pub const BRIGHT_WHITE       : &str = "bright white";


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}


impl Color {

    pub fn from_str(
        color: &str
    ) -> Option<Self>
    {
        match color {
            BLACK          => Some(Self::Black),
            RED            => Some(Self::Red),
            GREEN          => Some(Self::Green),
            YELLOW         => Some(Self::Yellow),
            BLUE           => Some(Self::Blue),
            MAGENTA        => Some(Self::Magenta),
            CYAN           => Some(Self::Cyan),
            WHITE          => Some(Self::White),
            BRIGHT_BLACK   => Some(Self::BrightBlack),
            BRIGHT_RED     => Some(Self::BrightRed),
            BRIGHT_GREEN   => Some(Self::BrightGreen),
            BRIGHT_YELLOW  => Some(Self::BrightYellow),
            BRIGHT_BLUE    => Some(Self::BrightBlue),
            BRIGHT_MAGENTA => Some(Self::BrightMagenta),
            BRIGHT_CYAN    => Some(Self::BrightCyan),
            BRIGHT_WHITE   => Some(Self::BrightWhite),
            _              => None,
        }
    }

    pub(crate) fn as_fg_ansi_code(
        &self
    ) -> &str
    {
        match self {
            Color::Black         => BLACK_ANSI_CODE,
            Color::Red           => RED_ANSI_CODE,
            Color::Green         => GREEN_ANSI_CODE,
            Color::Yellow        => YELLOW_ANSI_CODE,
            Color::Blue          => BLUE_ANSI_CODE,
            Color::Magenta       => MAGENTA_ANSI_CODE,
            Color::Cyan          => CYAN_ANSI_CODE,
            Color::White         => WHITE_ANSI_CODE,
            Color::BrightBlack   => BRIGHT_BLACK_ANSI_CODE,
            Color::BrightRed     => BRIGHT_RED_ANSI_CODE,
            Color::BrightGreen   => BRIGHT_GREEN_ANSI_CODE,
            Color::BrightYellow  => BRIGHT_YELLOW_ANSI_CODE,
            Color::BrightBlue    => BRIGHT_BLUE_ANSI_CODE,
            Color::BrightMagenta => BRIGHT_MAGENTA_ANSI_CODE,
            Color::BrightCyan    => BRIGHT_CYAN_ANSI_CODE,
            Color::BrightWhite   => BRIGHT_WHITE_ANSI_CODE,
        }
    }

    pub(crate) fn as_bg_ansi_code(
        &self
    ) -> &str
    {
        match self {
            Color::Black         => ON_BLACK_ANSI_CODE,
            Color::Red           => ON_RED_ANSI_CODE,
            Color::Green         => ON_GREEN_ANSI_CODE,
            Color::Yellow        => ON_YELLOW_ANSI_CODE,
            Color::Blue          => ON_BLUE_ANSI_CODE,
            Color::Magenta       => ON_MAGENTA_ANSI_CODE,
            Color::Cyan          => ON_CYAN_ANSI_CODE,
            Color::White         => ON_WHITE_ANSI_CODE,
            Color::BrightBlack   => ON_BRIGHT_BLACK_ANSI_CODE,
            Color::BrightRed     => ON_BRIGHT_RED_ANSI_CODE,
            Color::BrightGreen   => ON_BRIGHT_GREEN_ANSI_CODE,
            Color::BrightYellow  => ON_BRIGHT_YELLOW_ANSI_CODE,
            Color::BrightBlue    => ON_BRIGHT_BLUE_ANSI_CODE,
            Color::BrightMagenta => ON_BRIGHT_MAGENTA_ANSI_CODE,
            Color::BrightCyan    => ON_BRIGHT_CYAN_ANSI_CODE,
            Color::BrightWhite   => ON_BRIGHT_WHITE_ANSI_CODE,
        }
    }

}