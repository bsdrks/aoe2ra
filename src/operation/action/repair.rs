#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Repair {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub flags: Option<Vec<u8>>,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_2: u32,
}
