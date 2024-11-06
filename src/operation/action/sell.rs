use crate::resource_type::ResourceType;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sell {
    pub player_id: u8,
    pub resource_type: ResourceType,
    pub amount: u8,
}
