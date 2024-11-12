#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GatherPoint {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub x: f32,
    pub y: f32,
    pub unknown_u32_2: Option<u32>,
    pub unknown_u32_3: Option<u32>,
    pub unknown_u8_3: u8,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_4: u32,
}
