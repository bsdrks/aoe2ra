use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Game0x03 {
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: u32,
    pub unknown_u32_3: u32,
    pub unknown_u32_4: u32,
    pub unknown_u32_5: u32,
}

// Examples:
// 01000000_14000000_67031000_10000000_03000000_00000000_00000000_A0010000
impl Parse for Game0x03 {
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
    use crate::{
        hex::hex,
        parser::{
            Parse,
            Parser,
        },
    };

    use super::*;

    #[test]
    fn test_parse_1() {
        let mut parser = Parser::new(hex("
            1000 10000000 03000000 00000000 00000000 A0010000
        "));

        assert_eq!(
            Game0x03::parse(&mut parser),
            Game0x03 {
                unknown_u8_1: 16,
                unknown_u8_2: 0,
                unknown_u32_1: 16,
                unknown_u32_2: 3,
                unknown_u32_3: 0,
                unknown_u32_4: 0,
                unknown_u32_5: 416,
            }
        );
    }
}
