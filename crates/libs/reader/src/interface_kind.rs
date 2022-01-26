#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum InterfaceKind {
    Default,
    NonDefault,
    Static,
    Composable,
    Base,
}
