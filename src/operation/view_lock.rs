use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ViewLock {
    pub x: f32,
    pub y: f32,
    pub player_id: u8,
}

// Examples:
// 03000000_F1D89542_06C08442_01000000
impl Parse for ViewLock {
    fn parse(parser: &mut Parser) -> Self {
        let x = parser.f32();
        let y = parser.f32();
        let player_id = parser.u8();
        parser.skip(3);

        Self { x, y, player_id }
    }
}
