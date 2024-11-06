#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Research {
    pub building_id: u32,
    pub player_id: u8,
    pub technology_type: u32,
    pub selected_ids: Vec<u32>,
}
