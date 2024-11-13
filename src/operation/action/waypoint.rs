use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Waypoint {
    pub player_id: u8,
    pub x: f32,
    pub y: f32,
    pub unit_ids: Option<Vec<u32>>,
}

impl Parse for Waypoint {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let selected = parser.usize8_opt();
        let x = parser.f32();
        let y = parser.f32();
        let unit_ids = selected.map(|selected| parser.u32s(selected));

        Self {
            player_id,
            x,
            y,
            unit_ids,
        }
    }
}
