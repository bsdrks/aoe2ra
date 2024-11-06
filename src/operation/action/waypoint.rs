#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Waypoint {
    pub player_id: u8,
    pub x: f32,
    pub y: f32,
    pub selected_ids: Option<Vec<u32>>,
}
