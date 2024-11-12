use crate::formation_type::FormationType;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Formation {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub formation_type: FormationType,
    pub unit_ids: Vec<u32>,
    pub unknown_u32_1: u32,
}
