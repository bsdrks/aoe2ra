use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Action0x8c {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub unknown_u16_1: u16,
    pub unknown_u16_2: u16,
}

// Examples:
// 01000000_08000000_8C050400_DB170000_7B041E00
impl Parse for Action0x8c {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_1 = parser.u32();
        let unknown_u16_1 = parser.u16();
        let unknown_u16_2 = parser.u16();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unknown_u16_1,
            unknown_u16_2,
        }
    }
}
