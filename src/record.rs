#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]

pub struct Record {
    pub cursor: usize,
    pub raw: Vec<u8>,
}

impl Record {
    #[must_use]
    pub const fn new(raw: Vec<u8>) -> Self {
        Self { cursor: 0, raw }
    }
}
