use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unknown0x83 {
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub unknown_u8_3: u8,
    pub unit_ids: Vec<u32>,
}

// Examples:
// 01000000_0D000000_83020900_01000000_02573700_0016A400_00
impl Parse for Unknown0x83 {
    fn parse(parser: &mut Parser) -> Self {
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let unit_ids = parser.u32s(selected);
        let unknown_u32_1 = parser.u32();
        let unknown_u8_3 = parser.u8();

        Self {
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unknown_u8_3,
            unit_ids,
        }
    }
}
