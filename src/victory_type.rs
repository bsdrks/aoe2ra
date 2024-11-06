#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VictoryType {
    Standard,
    Conquest,
    Exploration,
    Ruins,
    Artifacts,
    Discoveries,
    Gold,
    TimeLimit,
    Score,
    Standard2,
    Regicide,
    LastManStanding,
}
