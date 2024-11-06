#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AiWaypoint {

    pub waypoint_count: u8,
    pub unit_ids: Vec<u32>,
    pub xs: Vec<u32>,
    pub ys: Vec<u32>,
}
