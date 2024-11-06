#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StartingResources {
    Standard,
    Low,
    Medium,
    High,
}
