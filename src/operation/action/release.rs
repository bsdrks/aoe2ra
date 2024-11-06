use crate::release_type::ReleaseType;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Release {

    pub x: Option<f32>,
    pub y: Option<f32>,
    pub release_type: ReleaseType,
    pub release_id: Option<u32>,
    pub unit_ids: Vec<u32>,
}
