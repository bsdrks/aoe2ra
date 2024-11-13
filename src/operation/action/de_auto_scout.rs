use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeAutoScout {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_0C000000_26020800_01000000_96130000_59921600
impl Parse for DeAutoScout {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unit_ids = parser.u32s(1);
        let unknown_u32_1 = parser.u32();

        DeAutoScout {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unit_ids,
            unknown_u32_1,
        }
    }
}
