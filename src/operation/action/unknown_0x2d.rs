use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Unknown0x2d {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: Option<u32>,
    pub x: f32,
    pub y: f32,
    pub flags: Option<Vec<u8>>,
    pub unknown_u32_2: u32,
    pub unknown_u32_3: u32,
    pub unknown_u32_4: u32,
}

// Examples:
// 01000000_1C000000_2D011800_FFFFFFFF_55858D42_00309842_01000001_01000000_130E0000_F7A61600
impl Parse for Unknown0x2d {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_1 = parser.u32_opt();
        let x = parser.f32();
        let y = parser.f32();
        let flags = parser.flags(4);
        let unknown_u32_2 = parser.u32();
        let unknown_u32_3 = parser.u32();
        let unknown_u32_4 = parser.u32();

        Unknown0x2d {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            x,
            y,
            flags,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u32_4,
        }
    }
}
