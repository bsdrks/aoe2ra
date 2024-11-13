use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Wall {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub start_x: u16,
    pub start_y: u16,
    pub end_x: u16,
    pub end_y: u16,
    pub unknown_u32_2: u32,
    pub unknown_u32_3: Option<u32>,
    pub unknown_u32_4: u32,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_5: u32,
}

// Examples:
// 01000000_20000000_69021C00_01000000_32005B00_31005F00_48000000_FFFFFFFF_00000001_790D0000_9D850800
// 01000000_24000000_69012000_02000000_19001400_19001400_48000000_FFFFFFFF_00000001_830D0000_860D0000_343A0800
impl Parse for Wall {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let start_x = parser.u16();
        let start_y = parser.u16();
        let end_x = parser.u16();
        let end_y = parser.u16();
        let unknown_u32_2 = parser.u32();
        let unknown_u32_3 = parser.u32_opt();
        let unknown_u32_4 = parser.u32();
        let unit_ids = parser.u32s(selected);
        let unknown_u32_5 = parser.u32();

        Wall {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            start_x,
            start_y,
            end_x,
            end_y,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u32_4,
            unit_ids,
            unknown_u32_5,
        }
    }
}
