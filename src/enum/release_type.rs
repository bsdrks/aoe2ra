use crate::parser::{
    Parse,
    Parser,
};

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReleaseType {
    All,
    InverseType,
    NotSelected,
    SameType,
    Selected,
}

impl From<u32> for ReleaseType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::All,
            3 => Self::Selected,
            4 => Self::SameType,
            5 => Self::NotSelected,
            6 => Self::InverseType,
            _ => unreachable!("Invalid ReleaseType value: {}", value),
        }
    }
}

impl Parse for ReleaseType {
    fn parse(parser: &mut Parser) -> Self {
        parser.u32().into()
    }
}
