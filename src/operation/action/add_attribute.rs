use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AddAttribute {
    pub player_id: u8,
    pub attribute: u8,
    pub unknown_u8_1: u8,
    pub amount: f32,
}

impl Parse for AddAttribute {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let attribute = parser.u8();
        let unknown_u8_1 = parser.u8();
        let amount = parser.f32();

        Self {
            player_id,
            attribute,
            unknown_u8_1,
            amount,
        }
    }
}
