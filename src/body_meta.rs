use crate::{
    parser::{
        Parse,
        Parser,
    },
    r#enum::RevealMapType,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BodyMeta {
    pub log_version: Option<u32>,
    pub checksum_interval: u32,
    pub multiplayer: bool,
    pub rec_owner: u32,
    pub reveal_map: RevealMapType,
    pub use_sequence_numbers: bool,
    pub number_of_chapters: u32,
    pub aok: bool,
}

impl Parse for BodyMeta {
    fn parse(parser: &mut Parser) -> Self {
        let next = parser.peek_u32();
        let log_version = (next != 500).then(|| parser.u32());
        let checksum_interval = parser.u32();
        let multiplayer = parser.bool32();
        let rec_owner = parser.u32();
        let reveal_map = RevealMapType::parse(parser);
        let use_sequence_numbers = parser.bool32();
        let number_of_chapters = parser.u32();
        let aok = parser.peek_bool_u32();

        if aok {
            parser.skip(4);
            let p = parser.peek_u32();

            if p != 2 {
                parser.skip(8);
            }
        }

        parser.skip(4);

        Self {
            log_version,
            checksum_interval,
            multiplayer,
            rec_owner,
            reveal_map,
            use_sequence_numbers,
            number_of_chapters,
            aok,
        }
    }
}
