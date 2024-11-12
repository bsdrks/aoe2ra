#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DeQueue {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u16_1: u16,
    pub unknown_u16_2: u16,
    pub unknown_u16_3: u16,
    pub unknown_u16_4: u16,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}
