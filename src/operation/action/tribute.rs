use crate::resource_type::ResourceType;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Tribute {
    pub player_id: u8,
    pub player_id_to: u8,
    pub resource_type: ResourceType,
    pub amount: f32,
    pub fee: f32,
}
