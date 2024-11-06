use crate::formation_type::FormationType;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Formation {

    pub player_id: u8,
    pub formation_type: FormationType,
    pub unit_ids: Vec<u32>,
}
