use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Speed {
    pub unknown_u16_1: u16,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: u32,
    pub unknown_u32_3: u32,
    pub unknown_u32_4: u32,
    pub unknown_u32_5: u32,
}

// Examples:
// 01000000_14000000_67011000_10000000_01000000_00000000_00000000_1B000000
// 01000000_14000000_67011000_13000000_01000000_00000000_00000000_1B000000
impl Parse for Speed {
    fn parse(parser: &mut Parser) -> Self {
        let unknown_u16_1 = parser.u16();
        let unknown_u32_1 = parser.u32();
        let unknown_u32_2 = parser.u32();
        let unknown_u32_3 = parser.u32();
        let unknown_u32_4 = parser.u32();
        let unknown_u32_5 = parser.u32();

        Speed {
            unknown_u16_1,
            unknown_u32_1,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u32_4,
            unknown_u32_5,
        }
    }
}
