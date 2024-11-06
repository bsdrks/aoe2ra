#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Patrol {

    pub waypoints: u16,
    pub xs: Vec<f32>,
    pub ys: Vec<f32>,
    pub unit_ids: Vec<u32>,
}
