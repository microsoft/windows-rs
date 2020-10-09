#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MethodKind {
    Normal,
    Get,
    Set,
    Add,
    Remove,
}
