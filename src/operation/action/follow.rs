use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Follow {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_2: u32,
}

// Examples:
// 01000000_10000000_14020C00_01000000_FF0C0000_6F0B0000_7F8A0300
impl Parse for Follow {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_1 = parser.u32();
        let unit_ids = parser.u32s(1);
        let unknown_u32_2 = parser.u32();

        Follow {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unit_ids,
            unknown_u32_2,
        }
    }
}
