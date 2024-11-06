use crate::player_type::PlayerType;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Player {
    dlc_id: u32,
    color_id: u32,
    selected_color: u8,
    selected_team_id: u8,
    dat_crc: [u8; 4],
    mp_game_version: u8,
    // Skip 4 bytes
    civ_id: u32,
    ai_type: String,
    ai_civ_name_index: u8,
    ai_name: String,
    name: String,
    r#type: PlayerType,
    profile_id: u32,
    // Skip 4 bytes
    player_number: i32,
    prefer_random: bool,
    custom_ai: bool,
    handicap: [u8; 8],
}
