#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Checksum {
    pub sync: u32,
    pub sequence: u32,
    pub checksum: Option<Vec<u8>>,
}
