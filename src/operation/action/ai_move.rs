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
