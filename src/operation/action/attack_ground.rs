use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AttackGround {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub x: f32,
    pub y: f32,
    pub flags: Option<Vec<u8>>,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_20000000_6B051C00_03000000_5515A742_ABEA2C43_00000100_433D0000_923F0000_C7440000_BCBA1600
impl Parse for AttackGround {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let x = parser.f32();
        let y = parser.f32();
        let flags = parser.flags(4);
        let unit_ids = parser.u32s(selected);
        let unknown_u32_1 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            x,
            y,
            flags,
            unit_ids,
            unknown_u32_1,
        }
    }
}
