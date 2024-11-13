use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unknown0x29 {}

impl Parse for Unknown0x29 {
    fn parse(parser: &mut Parser) -> Self {
        parser.skip(3);

        Self {}
    }
}
