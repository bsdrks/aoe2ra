use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Flare {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: Option<u32>,
    pub x: f32,
    pub y: f32,
    pub unknown_u8s_1: Vec<u8>,
    pub unknown_u32_2: u32,
}

// Examples:
// 01000000_14000000_73021000_FFFFFFFF_0000B242_00007442_03000001_451F2300
// 01000000_1A000000_73051600_FFFFFFFF_00E0C542_5555B340_09000100_01000100_0100C511_0E00
impl Parse for Flare {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_1 = parser.u32_opt();
        let x = parser.f32();
        let y = parser.f32();
        let selected = parser.usize8();
        let unknown_u8s_1 = parser.u8s(selected);
        let unknown_u32_2 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            x,
            y,
            unknown_u8s_1,
            unknown_u32_2,
        }
    }
}
