use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BackToWork {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: u32,
}

// Examples:
// 01000000_08000000_80020400_520B0000_5D3A0300
// 01000000_08000000_80020400_F00B0000_68AB0700
// 01000000_08000000_80020400_F00B0000_CCC40700
// 01000000_08000000_80010400_04120000_D4E81600
impl Parse for BackToWork {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let unknown_u32_1 = parser.u32();
        let unknown_u32_2 = parser.u32();

        Self {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            unknown_u32_1,
            unknown_u32_2,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            hex::hex,
            parser::Parser,
        },
    };

    #[test]
    fn test_parse_1() {
        let mut parser = Parser::new(hex("
            020400_F00B0000_68AB0700
        "));

        assert_eq!(
            BackToWork::parse(&mut parser),
            BackToWork {
                player_id: 2,
                unknown_u8_1: 4,
                unknown_u8_2: 0,
                unknown_u32_1: 3056,
                unknown_u32_2: 502_632
            }
        );
    }

    #[test]
    fn test_parse_2() {
        let mut parser = Parser::new(hex("
            020400_F00B0000_CCC40700
        "));

        assert_eq!(
            BackToWork::parse(&mut parser),
            BackToWork {
                player_id: 2,
                unknown_u8_1: 4,
                unknown_u8_2: 0,
                unknown_u32_1: 3056,
                unknown_u32_2: 509_132
            }
        );
    }

    #[test]
    fn test_parse_3() {
        let mut parser = Parser::new(hex("
            010400_04120000_D4E81600
        "));

        assert_eq!(
            BackToWork::parse(&mut parser),
            BackToWork {
                player_id: 1,
                unknown_u8_1: 4,
                unknown_u8_2: 0,
                unknown_u32_1: 4612,
                unknown_u32_2: 1_501_396
            }
        );
    }
}
