use crate::{
    parser::{
        Parse,
        Parser,
    },
    r#enum::StanceType,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Stance {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub stance_type: StanceType,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_10000000_12020C00_01000000_02000000_A6380000_BC511800
// 01000000_10000000_12020C00_01000000_02000000_4C1D0000_F6010000
// 01000000_10000000_12020C00_01000000_03000000_4C1D0000_2A730600
impl Parse for Stance {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let stance_type = StanceType::parse(parser);
        let unit_ids = parser.u32s(selected);
        let unknown_u32_1 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            stance_type,
            unit_ids,
            unknown_u32_1,
        }
    }
}
