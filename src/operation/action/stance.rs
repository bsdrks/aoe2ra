use crate::stance_type::StanceType;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Stance {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub stance_type: StanceType,
    pub unknown_u32_1: u32,
    pub unit_ids: Vec<u32>,
}
