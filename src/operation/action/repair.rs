use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Repair {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub flags: Option<Vec<u8>>,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_2: u32,
}

// Examples:
// 01000000_28000000_6E082400_06000000_C4170000_00000001_FD170000_62780000_ED3C0000_EB3C0000_14180000_40180000_2B162300
impl Parse for Repair {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let unknown_u32_1 = parser.u32();
        let flags = parser.flags(4);
        let unit_ids = parser.u32s(selected);
        let unknown_u32_2 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            flags,
            unit_ids,
            unknown_u32_2,
        }
    }
}
