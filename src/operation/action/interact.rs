#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Interact {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub target_id: u32,
    pub x: f32,
    pub y: f32,
    pub flags: Option<Vec<u8>>,
    pub unknown_u16_1: u16,
    pub unit_ids: Option<Vec<u32>>,
    pub unknown_u32_1: u32,
}
