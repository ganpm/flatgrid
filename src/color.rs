use crate::ansi::*;

/// Represents standard colors for foreground and background styling.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color;

impl Color {

    pub const BLACK         : &'static str = "black";
    pub const RED           : &'static str = "red";
    pub const GREEN         : &'static str = "green";
    pub const YELLOW        : &'static str = "yellow";
    pub const BLUE          : &'static str = "blue";
    pub const MAGENTA       : &'static str = "magenta";
    pub const CYAN          : &'static str = "cyan";
    pub const WHITE         : &'static str = "white";
    pub const BRIGHT_BLACK  : &'static str = "bright black";
    pub const BRIGHT_RED    : &'static str = "bright red";
    pub const BRIGHT_GREEN  : &'static str = "bright green";
    pub const BRIGHT_YELLOW : &'static str = "bright yellow";
    pub const BRIGHT_BLUE   : &'static str = "bright blue";
    pub const BRIGHT_MAGENTA: &'static str = "bright magenta";
    pub const BRIGHT_CYAN   : &'static str = "bright cyan";
    pub const BRIGHT_WHITE  : &'static str = "bright white";

}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Foreground {
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

impl Foreground {

    pub fn from_str(
        color: &str
    ) -> Option<Self>
    {
        match color {
            Color::BLACK          => Some(Self::Black),
            Color::RED            => Some(Self::Red),
            Color::GREEN          => Some(Self::Green),
            Color::YELLOW         => Some(Self::Yellow),
            Color::BLUE           => Some(Self::Blue),
            Color::MAGENTA        => Some(Self::Magenta),
            Color::CYAN           => Some(Self::Cyan),
            Color::WHITE          => Some(Self::White),
            Color::BRIGHT_BLACK   => Some(Self::BrightBlack),
            Color::BRIGHT_RED     => Some(Self::BrightRed),
            Color::BRIGHT_GREEN   => Some(Self::BrightGreen),
            Color::BRIGHT_YELLOW  => Some(Self::BrightYellow),
            Color::BRIGHT_BLUE    => Some(Self::BrightBlue),
            Color::BRIGHT_MAGENTA => Some(Self::BrightMagenta),
            Color::BRIGHT_CYAN    => Some(Self::BrightCyan),
            Color::BRIGHT_WHITE   => Some(Self::BrightWhite),
            _                     => None,
        }
    }

    pub fn as_ansi_code(
        &self
    ) -> &str
    {
        match self {
            Self::Black         => BLACK_ANSI_CODE,
            Self::Red           => RED_ANSI_CODE,
            Self::Green         => GREEN_ANSI_CODE,
            Self::Yellow        => YELLOW_ANSI_CODE,
            Self::Blue          => BLUE_ANSI_CODE,
            Self::Magenta       => MAGENTA_ANSI_CODE,
            Self::Cyan          => CYAN_ANSI_CODE,
            Self::White         => WHITE_ANSI_CODE,
            Self::BrightBlack   => BRIGHT_BLACK_ANSI_CODE,
            Self::BrightRed     => BRIGHT_RED_ANSI_CODE,
            Self::BrightGreen   => BRIGHT_GREEN_ANSI_CODE,
            Self::BrightYellow  => BRIGHT_YELLOW_ANSI_CODE,
            Self::BrightBlue    => BRIGHT_BLUE_ANSI_CODE,
            Self::BrightMagenta => BRIGHT_MAGENTA_ANSI_CODE,
            Self::BrightCyan    => BRIGHT_CYAN_ANSI_CODE,
            Self::BrightWhite   => BRIGHT_WHITE_ANSI_CODE,
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Background {
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


impl Background {

    pub fn from_str(
        color: &str
    ) -> Option<Self>
    {
        match color {
            Color::BLACK          => Some(Self::Black),
            Color::RED            => Some(Self::Red),
            Color::GREEN          => Some(Self::Green),
            Color::YELLOW         => Some(Self::Yellow),
            Color::BLUE           => Some(Self::Blue),
            Color::MAGENTA        => Some(Self::Magenta),
            Color::CYAN           => Some(Self::Cyan),
            Color::WHITE          => Some(Self::White),
            Color::BRIGHT_BLACK   => Some(Self::BrightBlack),
            Color::BRIGHT_RED     => Some(Self::BrightRed),
            Color::BRIGHT_GREEN   => Some(Self::BrightGreen),
            Color::BRIGHT_YELLOW  => Some(Self::BrightYellow),
            Color::BRIGHT_BLUE    => Some(Self::BrightBlue),
            Color::BRIGHT_MAGENTA => Some(Self::BrightMagenta),
            Color::BRIGHT_CYAN    => Some(Self::BrightCyan),
            Color::BRIGHT_WHITE   => Some(Self::BrightWhite),
            _                     => None,
        }
    }

    pub fn as_ansi_code(
        &self
    ) -> &str
    {
        match self {
            Self::Black         => ON_BLACK_ANSI_CODE,
            Self::Red           => ON_RED_ANSI_CODE,
            Self::Green         => ON_GREEN_ANSI_CODE,
            Self::Yellow        => ON_YELLOW_ANSI_CODE,
            Self::Blue          => ON_BLUE_ANSI_CODE,
            Self::Magenta       => ON_MAGENTA_ANSI_CODE,
            Self::Cyan          => ON_CYAN_ANSI_CODE,
            Self::White         => ON_WHITE_ANSI_CODE,
            Self::BrightBlack   => ON_BRIGHT_BLACK_ANSI_CODE,
            Self::BrightRed     => ON_BRIGHT_RED_ANSI_CODE,
            Self::BrightGreen   => ON_BRIGHT_GREEN_ANSI_CODE,
            Self::BrightYellow  => ON_BRIGHT_YELLOW_ANSI_CODE,
            Self::BrightBlue    => ON_BRIGHT_BLUE_ANSI_CODE,
            Self::BrightMagenta => ON_BRIGHT_MAGENTA_ANSI_CODE,
            Self::BrightCyan    => ON_BRIGHT_CYAN_ANSI_CODE,
            Self::BrightWhite   => ON_BRIGHT_WHITE_ANSI_CODE,
        }
    }

}