
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Align;

impl Align {
    pub const TOP    : &'static str = "top";
    pub const BOTTOM : &'static str = "bottom";
    pub const MIDDLE : &'static str = "middle";
    pub const LEFT   : &'static str = "left";
    pub const RIGHT  : &'static str = "right";
    pub const CENTER : &'static str = "center";
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum AlignV {
    #[default]
    Top,
    Bottom,
    Middle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum AlignH {
    #[default]
    Left,
    Right,
    Center,
}
