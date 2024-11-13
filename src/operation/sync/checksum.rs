use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Checksum {
    pub sync: u32,
    pub sequence: u32,
    pub checksum: Option<Vec<u8>>,
}

impl Parse for Checksum {
    fn parse(parser: &mut Parser) -> Self {
        parser.skip(8);
        let sync = parser.u32();
        parser.skip(4);
        let sequence = parser.u32();
        let checksum = (sequence > 0).then(|| parser.u8s(332));
        parser.skip(8);

        Checksum {
            sync,
            sequence,
            checksum,
        }
    }
}
