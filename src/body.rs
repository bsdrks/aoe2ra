use crate::{
    body_meta::BodyMeta,
    operation::{
        Chat,
        Operation,
    },
    parser::{
        Parse,
        Parser,
    },
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Body {
    pub meta: BodyMeta,
    pub operations: Vec<Operation>,
}

fn parse_operations(parser: &mut Parser) -> Vec<Operation> {
    let mut operations = Vec::new();

    while parser.cursor < parser.raw.len() {
        let operation = Operation::parse(parser);

        match operation {
            Operation::Chat(Chat { ref text }) if text == "\u{3}" => {
                break;
            }
            operation => operations.push(operation),
        }
    }

    operations
}

impl Parse for Body {
    fn parse(parser: &mut Parser) -> Self {
        let cursor = parser.usize32();

        parser.set_cursor(cursor);

        Self {
            meta: BodyMeta::parse(parser),
            operations: parse_operations(parser),
        }
    }
}
