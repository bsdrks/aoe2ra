#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Repair {

    pub repaired_id: u32,
    pub flags: Option<Vec<u8>>,
    pub unit_ids: Vec<u32>,
}
