use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GatherPoint {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub x: f32,
    pub y: f32,
    pub unknown_u32_2: Option<u32>,
    pub unknown_u32_3: Option<u32>,
    pub unknown_u8_3: u8,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_4: u32,
}

// Examples:
// 01000000_1D000000_78011900_01000000_AB8A7C42_5515A741_FFFFFFFF_FFFFFFFF_00B61D00_00B4DC02_00
// 01000000_1D000000_78021900_01000000_0000A742_00000E42_FFFFFFFF_FFFFFFFF_00A43800_00991517_00
// 01000000_1D000000_78021900_01000000_0000BF42_0000C441_FFFFFFFF_FFFFFFFF_00843800_00A5F318_00
impl Parse for GatherPoint {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let x = parser.f32();
        let y = parser.f32();
        let unknown_u32_2 = parser.u32_opt();
        let unknown_u32_3 = parser.u32_opt();
        let unknown_u8_3 = parser.u8();
        let unit_ids = parser.u32s(selected);
        let unknown_u32_4 = parser.u32();

        GatherPoint {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            x,
            y,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u8_3,
            unit_ids,
            unknown_u32_4,
        }
    }
}
