#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Wall {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub start_x: u16,
    pub start_y: u16,
    pub end_x: u16,
    pub end_y: u16,
    pub unknown_u32_2: u32,
    pub unknown_u32_3: Option<u32>,
    pub unknown_u32_4: u32,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_5: u32,
}
