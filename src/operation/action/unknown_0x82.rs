#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unknown0x82 {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u8_3: u8,
    pub unknown_u32_1: u32,
    pub unit_ids: Vec<u32>,
}