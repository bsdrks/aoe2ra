use crate::{
    parser::{
        Parse,
        Parser,
    },
    r#enum::RevealMapType,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BodyMeta {
    pub unknown_u32_1: u32,
    pub log_version: u32,
    pub unknown_u32_2: u32,
    pub multiplayer: bool,
    pub rec_owner: u32,
    pub reveal_map: RevealMapType,
    pub use_sequence_numbers: bool,
    pub number_of_chapters: u32,
}

// Examples:
// 05000000_F4010000_00000000_01000000_01000000_01000000_00000000_00000000
impl Parse for BodyMeta {
    fn parse(parser: &mut Parser) -> Self {
        println!("{}", parser.cursor);
        let unknown_u32_1 = parser.u32();
        let log_version = parser.u32();
        let unknown_u32_2 = parser.u32();
        let multiplayer = parser.bool32();
        let rec_owner = parser.u32();
        let reveal_map = RevealMapType::parse(parser);
        let use_sequence_numbers = parser.bool32();
        let number_of_chapters = parser.u32();

        Self {
            unknown_u32_1,
            log_version,
            unknown_u32_2,
            multiplayer,
            rec_owner,
            reveal_map,
            use_sequence_numbers,
            number_of_chapters,
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
            05000000 F4010000 00000000 01000000
            01000000 01000000 00000000 00000000
        "));

        assert_eq!(
            BodyMeta::parse(&mut parser),
            BodyMeta {
                unknown_u32_1: 5,
                log_version: 500,
                unknown_u32_2: 0,
                multiplayer: true,
                rec_owner: 1,
                reveal_map: RevealMapType::Explored,
                use_sequence_numbers: false,
                number_of_chapters: 0,
            }
        );
    }
}
