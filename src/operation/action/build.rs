#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Build {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub x: f32,
    pub y: f32,
    pub building_type: u32,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: u32,
    pub unknown_u32_3: u32,
    pub sprite_id: u32,
    pub unit_ids: Vec<u32>,
}
