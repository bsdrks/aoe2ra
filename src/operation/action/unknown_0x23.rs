use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Unknown0x23 {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub x: f32,
    pub y: f32,
    pub unknown_u32_2: u32,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_3: u32,
}

// Examples:
// 01000000_54000000_23025000_3F1D0000_0000AC42_0000C841_0F000000_01000000_A6380000_BA380000_C1380000_C6380000_CF380000_D6380000_DD380000_F5380000_FC380000_0B390000_0C390000_12390000_13390000_1E390000_1F390000_30AA1100
impl Parse for Unknown0x23 {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_1 = parser.u32();
        let x = parser.f32();
        let y = parser.f32();
        let selected = parser.usize32();
        let unknown_u32_2 = parser.u32();
        let unit_ids = parser.u32s(selected);
        let unknown_u32_3 = parser.u32();

        Unknown0x23 {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            x,
            y,
            unknown_u32_2,
            unit_ids,
            unknown_u32_3,
        }
    }
}
