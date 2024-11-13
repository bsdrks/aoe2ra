use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Resign {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub disconnected: bool,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_05000000_0B010100_004B4A27_00
// 01000000 05000000 0B020100 00578026 00
impl Parse for Resign {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let disconnected = parser.bool8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_1 = parser.u32();

        Resign {
            player_id,
            unknown_u8_1,
            disconnected,
            unknown_u8_2,
            unknown_u32_1,
        }
    }
}
