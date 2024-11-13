use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Build {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub x: f32,
    pub y: f32,
    pub building_type: u32,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: u32,
    pub unknown_u32_3: u32,
    pub sprite_id: u32,
    pub unit_ids: Vec<u32>,
}

// Examples:
// 01000000_28000000_66012400_02000000_00005842_00007041_46000000_FFFFFFFF_FFFFFFFF_00000001_C41D0000_C51D0000_C9150000
// 01000000_24000000_66022000_01000000_0000C442_00008A42_46000000_00000000_FFFFFFFF_00000001_C81D0000_49280000
impl Parse for Build {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let x = parser.f32();
        let y = parser.f32();
        let building_type = parser.u32();
        let unknown_u32_1 = parser.u32();
        let unknown_u32_2 = parser.u32();
        let unknown_u32_3 = parser.u32();
        let sprite_id = parser.u32();
        let unit_ids = parser.u32s(selected);

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            x,
            y,
            building_type,
            unknown_u32_1,
            unknown_u32_2,
            unknown_u32_3,
            sprite_id,
            unit_ids,
        }
    }
}
