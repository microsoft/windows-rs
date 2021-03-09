#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum MethodKind {
    Normal,
    Get,
    Set,
    Add,
    Remove,
}
