use crate::stance_type::StanceType;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Stance {

    pub stance_type: StanceType,
    pub unit_ids: Vec<u32>,
}
