use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Operation0x06 {
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
// 06000000_BD982800_04000000_01000000_01000000_03000000_0101_02000000_00000000__________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________2B000000_B8090000_01000000_04000000_E30A0000_26000000_02000000_02000000_01000000_CEA459B1_05DB7B43
// 06000000_FF741600_04000000_01000000_01000000_03000000_0101_02000000_01000000__________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________66000000_1A090000_00000000_01000000_310B0000_26000000_02000000_02000000_01000000_CEA459B1_05DB7B43
// 06000000_65602C00_04000000_01000000_01000000_03000000_0000_00000000_0E000000________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________02000000_02000000_01000000_CEA459B1_05DB7B43
// 06000000_31590B00_04000000_01000000_01000000_03000000_0000_00000000_0E000000________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________02000000_02000000_01000000_CEA459B1_05DB7B43
// 06000000_4B4A2700_04000000_01000000_01000000_03000000_0000_00000000_0E000000________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________________02000000_02000000_01000000_CEA459B1_05DB7B43
// 06000000_57802600_04000000_01000000_02000000_03000000_0100_08000000_04000000_FFFFFFFF_14050000_06000000_FFFFFFFF_02080000_00000000_D7070000_6E060000_07000000_A9020000_6C070000_03000000_FFFFFFFF_91070000_01000000_FFFFFFFF_66070000_02000000_FFFFFFFF_BA080000_05000000_FFFFFFFF_33070000_04000000_0101_08000000_04000000_73010000_E0060000_06000000_01000000_59080000_00000000_76000000_46070000_07000000_90020000_9E060000_03000000_E9030000_6A060000_01000000_9E010000_D4060000_02000000_30000000_A0070000_05000000_3E010000_F0060000_D8000000_02000000_02000000_01000000_CEA459B1_05DB7B43
impl Parse for Operation0x06 {
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
