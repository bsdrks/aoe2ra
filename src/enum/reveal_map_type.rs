use crate::parser::{
    Parse,
    Parser,
};

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RevealMapType {
    None,
    Explored,
    All,
    NoFog,
}

impl From<u32> for RevealMapType {
    fn from(value: u32) -> Self {
        match value {
            0x00 => Self::None,
            0x01 => Self::Explored,
            0x02 => Self::All,
            0x03 => Self::NoFog,
            _ => unreachable!("Invalid value for RevealMap: {}", value),
        }
    }
}

impl Parse for RevealMapType {
    fn parse(parser: &mut Parser) -> Self {
        parser.u32().into()
    }
}
