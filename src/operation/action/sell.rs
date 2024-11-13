use crate::{
    parser::{
        Parse,
        Parser,
    },
    r#enum::ResourceType,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sell {
    pub player_id: u8,
    pub resource_type: ResourceType,
    pub amount: u8,
}

impl Parse for Sell {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let resource_type = ResourceType::parse(parser);
        let amount = parser.u8();
        parser.skip(4);

        Sell {
            player_id,
            resource_type,
            amount,
        }
    }
}
