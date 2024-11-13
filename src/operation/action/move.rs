use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Move {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: Option<u32>,
    pub x: f32,
    pub y: f32,
    pub unknown_u16_1: u16,
    pub flags: Option<Vec<u8>>,
    pub unit_ids: Option<Vec<u32>>,
    pub unknown_u32_2: u32,
}

// Examples:
// 01000000_28000000_03012400_FFFFFFFF_55154942_55D5C741_04000000_01010000_F9360000_F7360000_F8360000_F6360000_0E190000
// 01000000_1C000000_03011800_FFFFFFFF_55C59D42_00709442_01000000_01010000_5C380000_F3681700
// 01000000_18000000_03011400_FFFFFFFF_ABCA4C42_5525B742_FFFF0000_01010000_27691300
impl Parse for Move {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_1 = parser.u32_opt();
        let x = parser.f32();
        let y = parser.f32();
        let unknown_u16_1 = parser.u16();
        let selected = parser.usize32_opt();
        let flags = parser.flags(4);
        let unknown_u32_2 = parser.u32();
        let unit_ids = selected.map(|selected| parser.u32s(selected));

        Move {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            x,
            y,
            unknown_u16_1,
            flags,
            unit_ids,
            unknown_u32_2,
        }
    }
}
