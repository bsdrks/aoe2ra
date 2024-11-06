use crate::stance_type::StanceType;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Diplomacy {
    pub player_id: u8,
    // Skip 1 byte
    pub target_player_id: u8,
    // Skip 3 bytes
    pub stance_float: f32,
    pub stance: StanceType,
}
