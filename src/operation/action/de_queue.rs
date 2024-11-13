use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeQueue {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u16_1: u16,
    pub unknown_u16_2: u16,
    pub unknown_u16_3: u16,
    pub unknown_u16_4: u16,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_14000000_81011000_01000000_00006D00_53000100_B61D0000_4A040000
// 01000000_14000000_81011000_01000000_00006D00_53000100_B61D0000_92060000
// 01000000_14000000_81021000_01000000_00005700_04000100_A4380000_C4B61300
// 01000000_14000000_81011000_01000000_00006500_C0010200_130E0000_27A61600
impl Parse for DeQueue {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let unknown_u16_1 = parser.u16();
        let unknown_u16_2 = parser.u16();
        let unknown_u16_3 = parser.u16();
        let unknown_u16_4 = parser.u16();
        let unit_ids = parser.u32s(selected);
        let unknown_u32_1 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u16_1,
            unknown_u16_2,
            unknown_u16_3,
            unknown_u16_4,
            unit_ids,
            unknown_u32_1,
        }
    }
}
