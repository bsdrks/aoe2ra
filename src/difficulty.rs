#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Difficulty {
    Hardest,
    Hard,
    Moderate,
    Standard,
    Easiest,
    Extreme,
}
