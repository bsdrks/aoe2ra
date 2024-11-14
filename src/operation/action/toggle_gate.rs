use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ToggleGate {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub unknown_u32_1: u32,
    pub unknown_u32_2: u32,
}

// Examples:
// 01000000_08000000_72010400_D8380000_B92B1E00
impl Parse for ToggleGate {
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
        crate::hex::hex,
    };

    #[test]
    fn test_parse_1() {
        let mut parser = Parser::new(hex("
            010400 D8380000 B92B1E00
        "));

        assert_eq!(
            ToggleGate::parse(&mut parser),
            ToggleGate {
                player_id: 1,
                unknown_u8_1: 4,
                unknown_u8_2: 0,
                unknown_u32_1: 14552,
                unknown_u32_2: 1_977_273,
            }
        );
    }
}
