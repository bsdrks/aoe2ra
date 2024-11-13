use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Release {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub x: f32,
    pub y: f32,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: u32,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_3: u32,
}

// Examples:
// 01000000_1C000000_6F021800_01000000_000080BF_000080BF_FFFFFFFF_00000000_84380000_B6621600
// 01000000_20000000_6F021C00_02000000_000080BF_000080BF_FFFFFFFF_00000000_8F380000_A4380000_577C1800
// 01000000_20000000_6F021C00_02000000_000080BF_000080BF_FFFFFFFF_00000000_8F380000_A4380000_A5F31800
// 01000000_1C000000_6F031800_01000000_000080BF_000080BF_03000000_01000000_C52A0000_0B0B1E00
impl Parse for Release {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let x = parser.f32();
        let y = parser.f32();
        let unknown_u32_1 = parser.u32();
        let unknown_u32_2 = parser.u32();
        let unit_ids = parser.u32s(selected);
        let unknown_u32_3 = parser.u32();

        Release {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            x,
            y,
            unknown_u32_1,
            unknown_u32_2,
            unit_ids,
            unknown_u32_3,
        }
    }
}
