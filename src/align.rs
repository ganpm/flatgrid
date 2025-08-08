const TOP   : &str = "top";
const BOTTOM: &str = "bottom";
const MIDDLE: &str = "middle";
const LEFT  : &str = "left";
const RIGHT : &str = "right";
const CENTER: &str = "center";


#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VAlign {
    #[default]
    Top,
    Bottom,
    Middle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAlign {
    #[default]
    Left,
    Right,
    Center,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    VAlign(VAlign),
    HAlign(HAlign),
}


impl Align {

    pub fn from_str(align: &str) -> Option<Self> {
        match align {
            TOP    => Some(Self::VAlign(VAlign::Top)),
            BOTTOM => Some(Self::VAlign(VAlign::Bottom)),
            MIDDLE => Some(Self::VAlign(VAlign::Middle)),
            LEFT   => Some(Self::HAlign(HAlign::Left)),
            RIGHT  => Some(Self::HAlign(HAlign::Right)),
            CENTER => Some(Self::HAlign(HAlign::Center)),
            _      => None,
        }
    }

}
