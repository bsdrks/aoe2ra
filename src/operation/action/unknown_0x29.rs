use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Unknown0x29 {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: u32,
    pub unknown_u32_3: u32,
}

// Examples:
// 01000000_0C000000_29010800_72060000_2B070000_A0010000
impl Parse for Unknown0x29 {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_1 = parser.u32();
        let unknown_u32_2 = parser.u32();
        let unknown_u32_3 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unknown_u32_2,
            unknown_u32_3,
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
    fn test_unknown_0x29() {
        let mut parser = Parser::new(hex("
            010800 72060000 2B070000 A0010000
        "));

        let unknown_0x29 = Unknown0x29::parse(&mut parser);

        assert_eq!(
            unknown_0x29,
            Unknown0x29 {
                player_id: 1,
                unknown_u8_1: 8,
                unknown_u8_2: 0,
                unknown_u32_1: 1650,
                unknown_u32_2: 1835,
                unknown_u32_3: 416,
            }
        );
    }
}
