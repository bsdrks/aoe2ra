use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AlliedVictory {
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: u32,
    pub unknown_u32_3: u32,
    pub unknown_u32_4: u32,
    pub unknown_u32_5: u32,
}

// Examples:
// 01000000_14000000_67051000_10000000_05000000_00000000_00000000_08020000
impl Parse for AlliedVictory {
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
            1000 10000000 05000000 00000000 00000000 08020000
        "));

        assert_eq!(
            AlliedVictory::parse(&mut parser),
            AlliedVictory {
                unknown_u8_1: 16,
                unknown_u8_2: 0,
                unknown_u32_1: 16,
                unknown_u32_2: 5,
                unknown_u32_3: 0,
                unknown_u32_4: 0,
                unknown_u32_5: 520,
            }
        );
    }
}
