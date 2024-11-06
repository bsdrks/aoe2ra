#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Guard {

    pub guarded_unit_id: u32,
    pub unit_ids: Vec<u32>,
}
