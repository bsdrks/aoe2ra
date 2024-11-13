use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Chat {
    pub text: String,
}

impl Parse for Chat {
    fn parse(parser: &mut Parser) -> Self {
        parser.skip(4);
        let length = parser.usize32();
        let text = parser.u8s(length);

        Chat {
            text: text.into_iter().map(|b| b as char).collect(),
        }
    }
}
