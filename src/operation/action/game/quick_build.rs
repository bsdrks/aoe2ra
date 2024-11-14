use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct QuickBuild {
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: u32,
    pub unknown_u32_3: u32,
    pub unknown_u32_4: u32,
    pub unknown_u32_5: u32,
}

// Examples:
// 01000000 14000000 67041000 10000000 04000000 00000000 00000000 A0010000
impl Parse for QuickBuild {
    fn parse(parser: &mut Parser) -> Self {
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_1 = parser.u32();
        let unknown_u32_2 = parser.u32();
        let unknown_u32_3 = parser.u32();
        let unknown_u32_4 = parser.u32();
        let unknown_u32_5 = parser.u32();

        Self {
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unknown_u32_2,
            unknown_u32_3,
            unknown_u32_4,
            unknown_u32_5,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::QuickBuild,
        crate::{
            hex::hex,
            parser::{
                Parse,
                Parser,
            },
        },
    };

    #[test]
    fn test_parse_1() {
        let mut parser = Parser::new(hex("
            1000 10000000 04000000 00000000 00000000 A0010000
        "));

        assert_eq!(
            QuickBuild::parse(&mut parser),
            QuickBuild {
                unknown_u8_1: 16,
                unknown_u8_2: 0,
                unknown_u32_1: 16,
                unknown_u32_2: 4,
                unknown_u32_3: 0,
                unknown_u32_4: 0,
                unknown_u32_5: 416,
            },
        );
    }
}
