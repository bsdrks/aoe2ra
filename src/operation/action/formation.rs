use crate::{
    parser::{
        Parse,
        Parser,
    },
    r#enum::FormationType,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Formation {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub formation_type: FormationType,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_18000000_17011400_03000000_04000000_8E0B0000_300E0000_490E0000_99C30E00
impl Parse for Formation {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32();
        let formation_type = FormationType::parse(parser);
        let unit_ids = parser.u32s(selected);
        let unknown_u32_1 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            formation_type,
            unit_ids,
            unknown_u32_1,
        }
    }
}
