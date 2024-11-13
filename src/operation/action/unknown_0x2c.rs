use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unknown0x2c {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_0C000000_2C010800_01000000_F00C0000_F44E0E00
// 01000000 10000000 2C010C00 02000000 A30C0000 1F120000 A42F1500
impl Parse for Unknown0x2c {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let unit_ids = parser.u32s(selected);
        let unknown_u32_1 = parser.u32();

        Unknown0x2c {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unit_ids,
            unknown_u32_1,
        }
    }
}
