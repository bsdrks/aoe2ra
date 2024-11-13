use crate::parser::{
    Parse,
    Parser,
};

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StanceType {
    Aggressive,
    Defensive,
    StandGround,
    Passive,
}

impl From<u32> for StanceType {
    fn from(value: u32) -> Self {
        match value {
            0x00 => Self::Aggressive,
            0x01 => Self::Defensive,
            0x02 => Self::StandGround,
            0x03 => Self::Passive,
            _ => unreachable!("Invalid value for StanceType: {}", value),
        }
    }
}

impl Parse for StanceType {
    fn parse(parser: &mut Parser) -> Self {
        parser.u32().into()
    }
}
