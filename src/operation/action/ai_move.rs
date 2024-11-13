use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AiMove {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: Option<u32>,
    pub unknown_u32_3: u32,
    pub unknown_u32_4: u32,
    pub x: f32,
    pub y: f32,
    pub unknown_f32_1: f32,
    pub unknown_f32_2: f32,
    pub unknown_u32_5: u32,
    pub flags: Option<Vec<u8>>,
    pub unit_ids: Option<Vec<u32>>,
}

// Examples:
// 01000000_2C000000_0A022800_01000000_CA1D0000_FFFFFFFF_00000000_C1020000_0000A442_00009642_000080BF_0000803F_01FF0100_43000000
// 01000000_2C000000_0A022800_01000000_C71D0000_53370000_00000000_BE020000_0000C442_00008A42_00000000_0000803F_63020101_56280000
// 01000000_38000000_0A023400_03000000_481D0000_FFFFFFFF_00000000_C2020000_000080BF_000080BF_000080BF_000080BF_64FF0100_00000000_491D0000_1B380000_B8680000
impl Parse for AiMove {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let selected = parser.usize32_opt();
        let unknown_u32_1 = parser.u32();
        let unknown_u32_2 = parser.u32_opt();
        let unknown_u32_3 = parser.u32();
        let unknown_u32_4 = parser.u32();
        let x = parser.f32();
        let y = parser.f32();
        let unknown_f32_1 = parser.f32();
        let unknown_f32_2 = parser.f32();
        let unknown_u32_5 = parser.u32();
        let flags = parser.flags(4);
        let unit_ids = selected.map(|selected| parser.u32s(selected));

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u32_4,
            x,
            y,
            unknown_f32_1,
            unknown_f32_2,
            unknown_u32_5,
            flags,
            unit_ids,
        }
    }
}
