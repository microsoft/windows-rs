// TODO: call MethodKind
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MethodCategory {
    Normal,
    Get,
    Set,
    Add,
    Remove,
}
