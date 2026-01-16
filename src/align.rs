use std::ops::{BitOr, BitOrAssign};

/// Alignment options for cell content.
/// 
/// Supports horizontal and vertical alignment using bitflags.
/// Bitflags can be combined using bitwise OR operations.
/// If conflicting horizontal or vertical alignments are set,
/// the last one for each axis takes precedence.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Align(u8);

#[allow(non_upper_case_globals)]
impl Align {

    pub const Top    : Align = Align(1 << 0);
    pub const Bottom : Align = Align(1 << 1);
    pub const Middle : Align = Align(1 << 2);
    pub const Left   : Align = Align(1 << 3);
    pub const Right  : Align = Align(1 << 4);
    pub const Center : Align = Align(1 << 5);

    pub const H_MASK : u8 = Align::Left.0 | Align::Center.0 | Align::Right.0;
    pub const V_MASK : u8 = Align::Top.0 | Align::Middle.0 | Align::Bottom.0;

    pub(crate) fn get_h(
        &self
    ) -> Option<AlignH>
    {
        let h = self.0 & Align::H_MASK;
        if h == Align::Left.0 { return Some(AlignH::Left); }
        if h == Align::Right.0 { return Some(AlignH::Right); }
        if h == Align::Center.0 { return Some(AlignH::Center); }
        None
    }

    pub(crate) fn get_v(
        &self
    ) -> Option<AlignV>
    {
        let v = self.0 & Align::V_MASK;
        if v == Align::Top.0 { return Some(AlignV::Top); }
        if v == Align::Bottom.0 { return Some(AlignV::Bottom); }
        if v == Align::Middle.0 { return Some(AlignV::Middle); }
        None
    }

}

impl BitOr for Align {

    type Output = Align;

    fn bitor(
        self,
        rhs: Align
    ) -> Align
    {
        // Combine horizontal and vertical alignments,
        // with rhs taking precedence in case of conflicts
        let h = (self.0 & !Align::H_MASK) | (rhs.0 & Align::H_MASK);
        let v = (self.0 & !Align::V_MASK) | (rhs.0 & Align::V_MASK);
        Align(h | v)
    }

}

impl BitOrAssign for Align {

    fn bitor_assign(&mut self, rhs: Align) {
        *self = *self | rhs;
    }

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
