use crate::r#enum::StanceType;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Diplomacy {
    pub player_id: u8,
    pub target_player_id: u8,
    pub stance_float: f32,
    pub stance: StanceType,
}
