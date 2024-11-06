#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Follow {

    pub followed_unit_id: u32,
    pub unit_ids: Vec<u32>,
}
