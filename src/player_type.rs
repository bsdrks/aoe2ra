#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PlayerType {
    Absent,
    Closed,
    Human,
    Eliminiated,
    Computer,
    Cyborg,
    Spectator,
}
