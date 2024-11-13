use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unknown0x06 {
    unknown_u32_1: u32,
    unknown_u32_2: u32,
    unknown_u32_3: u32,
    unknown_u32_4: u32,
    unknown_u32_5: u32,
    unknown_u8_1: u8,
    unknown_u8_2: u8,
    unknown_u32_6: u32,
    unknown_u32_7: u32,
    unknown_u32_8: u32,
    unknown_u32_9: u32,
    unknown_u32_10: u32,
    unknown_u32_11: u32,
    unknown_u32_12: u32,
    unknown_u32_13: u32,
    unknown_u32_14: u32,
    unknown_u32_15: u32,
    unknown_u32_16: u32,
    unknown_u32_17: u32,
    unknown_u32_18: u32,
}

// Examples:
// 06000000_BD982800_04000000_01000000_01000000_03000000_01010200_00000000_00002B00_0000B809_00000100_00000400_0000E30A_00002600_00000200_00000200_00000100_0000CEA4_59B105DB_7B43
// 06000000_FF741600_04000000_01000000_01000000_03000000_01010200_00000100_00006600_00001A09_00000000_00000100_0000310B_00002600_00000200_00000200_00000100_0000CEA4_59B105DB_7B43
impl Parse for Unknown0x06 {
    fn parse(parser: &mut Parser) -> Self {
        let unknown_u32_1 = parser.u32();
        let unknown_u32_2 = parser.u32();
        let unknown_u32_3 = parser.u32();
        let unknown_u32_4 = parser.u32();
        let unknown_u32_5 = parser.u32();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_6 = parser.u32();
        let unknown_u32_7 = parser.u32();
        let unknown_u32_8 = parser.u32();
        let unknown_u32_9 = parser.u32();
        let unknown_u32_10 = parser.u32();
        let unknown_u32_11 = parser.u32();
        let unknown_u32_12 = parser.u32();
        let unknown_u32_13 = parser.u32();
        let unknown_u32_14 = parser.u32();
        let unknown_u32_15 = parser.u32();
        let unknown_u32_16 = parser.u32();
        let unknown_u32_17 = parser.u32();
        let unknown_u32_18 = parser.u32();

        Self {
            unknown_u32_1,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u32_4,
            unknown_u32_5,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_6,
            unknown_u32_7,
            unknown_u32_8,
            unknown_u32_9,
            unknown_u32_10,
            unknown_u32_11,
            unknown_u32_12,
            unknown_u32_13,
            unknown_u32_14,
            unknown_u32_15,
            unknown_u32_16,
            unknown_u32_17,
            unknown_u32_18,
        }
    }
}
