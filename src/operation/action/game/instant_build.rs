use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InstantBuild {
    pub unknown_u16_1: u16,
    pub unknown_u32_1: u32,
    pub unknown_u16_2: u16,
    pub unknown_u16_3: u16,
    pub unknown_u32_2: u32,
    pub unknown_u32_3: u32,
    pub unknown_u16_4: u16,
    pub unknown_u16_5: u16,
}

// Examples:
// 01000000_14000000_67021000_0B000000_02000800_00000000_01000000_859C0200
// 01000000_14000000_67021000_0B000000_02000800_00000000_01000000_59140200
// 01000000_14000000_67021000_0B000000_02000800_00000000_01000000_1E010200
impl Parse for InstantBuild {
    fn parse(parser: &mut Parser) -> Self {
        let unknown_u16_1 = parser.u16();
        let unknown_u32_1 = parser.u32();
        let unknown_u16_2 = parser.u16();
        let unknown_u16_3 = parser.u16();
        let unknown_u32_2 = parser.u32();
        let unknown_u32_3 = parser.u32();
        let unknown_u16_4 = parser.u16();
        let unknown_u16_5 = parser.u16();

        InstantBuild {
            unknown_u16_1,
            unknown_u32_1,
            unknown_u16_2,
            unknown_u16_3,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u16_4,
            unknown_u16_5,
        }
    }
}
