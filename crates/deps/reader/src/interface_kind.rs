#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum InterfaceKind {
    Default,
    NonDefault,
    Static,
    Composable,
    Base,

    // TODO: only used by old gen
    Extend,
    Overridable,
}
