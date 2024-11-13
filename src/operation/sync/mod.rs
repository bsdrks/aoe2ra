use {
    crate::parser::{
        Parse,
        Parser,
    },
    checksum::Checksum,
};

pub mod checksum;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sync {
    pub time_increment: u32,
    pub checksum: Option<Checksum>,
}

// Examples:
// 02000000_0D000000
impl Sync {
    pub fn parse(parser: &mut Parser) -> Self {
        let time_increment = parser.u32();
        let next = parser.peek_u32();
        let checksum = (next == 0).then(|| Checksum::parse(parser));

        Self {
            time_increment,
            checksum,
        }
    }
}
