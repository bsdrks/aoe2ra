use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum OperationType {
    Action,
    Chat,
    Sync,
    ViewLock,
    Unknown0x06,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnknownOperationType {
    pub value: u32,
}

impl TryFrom<u32> for OperationType {
    type Error = UnknownOperationType;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Action),
            0x02 => Ok(Self::Sync),
            0x03 => Ok(Self::ViewLock),
            0x04 => Ok(Self::Chat),
            0x06 => Ok(Self::Unknown0x06),
            _ => Err(UnknownOperationType { value }),
        }
    }
}

impl Parse for OperationType {
    fn parse(parser: &mut Parser) -> Self {
        let cursor = parser.cursor;

        match parser.u32().try_into() {
            Ok(operation_type) => operation_type,
            Err(unknown_operation_type) => {
                panic!(
                    "{cursor} (0x{cursor:x?}): unknown operation type {}",
                    unknown_operation_type.value
                );
            }
        }
    }
}
