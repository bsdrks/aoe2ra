use crate::parser::{
    Parse,
    Parser,
};

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResourceType {
    Food,
    Gold,
    Stone,
    Wood,
    Unknown,
}

impl From<u32> for ResourceType {
    fn from(value: u32) -> Self {
        match value {
            0x00 => Self::Food,
            0x01 => Self::Wood,
            0x02 => Self::Stone,
            0x03 => Self::Gold,
            _ => Self::Unknown,
        }
    }
}

impl Parse for ResourceType {
    fn parse(parser: &mut Parser) -> Self {
        parser.u32().into()
    }
}
