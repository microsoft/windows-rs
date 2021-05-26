#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum InterfaceKind {
    Default,
    NonDefault,
    Overridable,
    Static,
    Composable,
    Extend,
}
