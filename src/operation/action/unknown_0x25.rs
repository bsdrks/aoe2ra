use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unknown0x25 {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: Option<u32>,
    pub unknown_u32_2: u32,
    pub unknown_u8_3: u8,
}

// Examples:
// 01000000_09000000_25020500_FFFFFFFF_0080390F_00
// 01000000_09000000_25020500_FFFFFFFF_0330AA11_00
impl Parse for Unknown0x25 {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_1 = parser.u32_opt();
        let unknown_u32_2 = parser.u32();
        let unknown_u8_3 = parser.u8();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unknown_u32_2,
            unknown_u8_3,
        }
    }
}
