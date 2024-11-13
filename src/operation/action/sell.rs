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
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u16_1: u16,
    pub resource_type: ResourceType,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: u32,
}

// Examples:
// 01000000_0C000000_7A010800_01000100_2D0D0000_7E141C00
impl Parse for Sell {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u16_1 = parser.u16();
        let resource_type = ResourceType::parse(parser);
        let unknown_u32_1 = parser.u32();
        let unknown_u32_2 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u16_1,
            resource_type,
            unknown_u32_1,
            unknown_u32_2,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::hex::hex,
    };

    #[test]
    fn test_sell() {
        let mut parser = Parser::new(hex("
            010800 01000100 2D0D0000 7E141C00
        "));

        let sell = Sell::parse(&mut parser);

        assert_eq!(
            sell,
            Sell {
                player_id: 1,
                unknown_u8_1: 8,
                unknown_u8_2: 0,
                unknown_u16_1: 1,
                resource_type: ResourceType::Gold,
                unknown_u32_1: 3373,
                unknown_u32_2: 1_840_254,
            },
        );
    }
}
