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
