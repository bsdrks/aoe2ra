use checksum::Checksum;

pub mod checksum;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sync {
    pub time_increment: u32,
    pub checksum: Option<Checksum>,
}
