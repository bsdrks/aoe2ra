use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unknown0xc4 {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: u32,
    pub unknown_u32_3: u32,
    pub unknown_u32_4: u32,
    pub unknown_u32_5: u32,
    pub unknown_u32_6: u32,
    pub unknown_u32_7: u32,
    pub unknown_u32_8: u32,
    pub unknown_u32_9: u32,
    pub unknown_u16_1: u16,
    pub unknown_u16_2: u16,
    pub unknown_u8_3: u8,
    pub unknown_u32_10: u32,
}

// Examples:
// 01000000_2D000000_C4012900_00000000_00000000_00000000_00000000_9A99993E_9A99993E_9A99993E_9A99993E_01000000_03000200_02A9FF13_00
impl Parse for Unknown0xc4 {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_1 = parser.u32();
        let unknown_u32_2 = parser.u32();
        let unknown_u32_3 = parser.u32();
        let unknown_u32_4 = parser.u32();
        let unknown_u32_5 = parser.u32();
        let unknown_u32_6 = parser.u32();
        let unknown_u32_7 = parser.u32();
        let unknown_u32_8 = parser.u32();
        let unknown_u32_9 = parser.u32();
        let unknown_u16_1 = parser.u16();
        let unknown_u16_2 = parser.u16();
        let unknown_u8_3 = parser.u8();
        let unknown_u32_10 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u32_4,
            unknown_u32_5,
            unknown_u32_6,
            unknown_u32_7,
            unknown_u32_8,
            unknown_u32_9,
            unknown_u16_1,
            unknown_u16_2,
            unknown_u8_3,
            unknown_u32_10,
        }
    }
}
