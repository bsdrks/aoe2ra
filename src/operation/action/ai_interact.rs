use crate::parser::{
    Parse,
    Parser,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AiInteract {
    pub player_id: u8,
    pub unknown_u8_1: u8,
    pub unknown_u8_2: u8,
    pub target_id: u32,
    pub x: f32,
    pub y: f32,
    pub flags: Option<Vec<u8>>,
    pub unit_ids: Option<Vec<u32>>,
    pub unknown_u32_1: u32,
}

// Examples:
// 01000000_1C000000_02021800_FB360000_E8E4C042_316C9942_01000000_01000000_C21D0000_9C130000
impl Parse for AiInteract {
    fn parse(parser: &mut Parser) -> Self {
        let player_id = parser.u8();
        let unknown_u8_1 = parser.u8();
        let unknown_u8_2 = parser.u8();
        let target_id = parser.u32();
        let x = parser.f32();
        let y = parser.f32();
        let selected = parser.usize32_opt();
        let flags = parser.flags(4);
        let unknown_u32_1 = parser.u32();
        let unit_ids = selected.map(|selected| parser.u32s(selected));

        AiInteract {
            player_id,
            unknown_u8_1,
            unknown_u8_2,
            target_id,
            x,
            y,
            flags,
            unit_ids,
            unknown_u32_1,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::AiInteract,
        crate::{
            hex::hex,
            parser::{
                Parse,
                Parser,
            },
        },
    };

    #[test]
    fn test_parse_ai_interact_1() {
        let mut parser = Parser::new(hex("
              021800 FB360000 FCE3C042 2E6A9942
            01000000 01000000 C21D0000 98130000
        "));

        assert_eq!(
            AiInteract::parse(&mut parser),
            AiInteract {
                player_id: 2,
                unknown_u8_1: 24,
                unknown_u8_2: 0,
                target_id: 14075,
                x: 96.44528,
                y: 76.70738,
                flags: Some(vec![1, 0, 0, 0]),
                unknown_u32_1: 7618,
                unit_ids: Some(vec![5016])
            }
        );
    }

    #[test]
    fn test_parse_ai_interact_2() {
        let mut parser = Parser::new(hex("
              021800 A00E0000 00004242 00009142
            01000000 01000000 993A0000 E6572C00
        "));

        assert_eq!(
            AiInteract::parse(&mut parser),
            AiInteract {
                player_id: 2,
                unknown_u8_1: 24,
                unknown_u8_2: 0,
                target_id: 3744,
                x: 48.5,
                y: 72.5,
                flags: Some(vec![1, 0, 0, 0]),
                unknown_u32_1: 15001,
                unit_ids: Some(vec![2_906_086])
            }
        );
    }
}
