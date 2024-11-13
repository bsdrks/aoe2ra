use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Patrol {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u8_3: u8,
    pub unknown_u8_4: u8,
    pub unknown_u8_5: u8,
    pub unknown_u8_6: u8,
    pub xs: Vec<f32>,
    pub ys: Vec<f32>,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_60000000_15035C00_01000000_01000001_AB523C43_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_55755E42_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_E21D0000_E3380C00
impl Parse for Patrol {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let unknown_u8_3 = parser.u8();
        let unknown_u8_4 = parser.u8();
        let unknown_u8_5 = parser.u8();
        let unknown_u8_6 = parser.u8();
        let xs = parser.f32s(10);
        let ys = parser.f32s(10);
        let unit_ids = parser.u32s(selected);
        let unknown_u32_1 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u8_3,
            unknown_u8_4,
            unknown_u8_5,
            unknown_u8_6,
            xs,
            ys,
            unit_ids,
            unknown_u32_1,
        }
    }
}
