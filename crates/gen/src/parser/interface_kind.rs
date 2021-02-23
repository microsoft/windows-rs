#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum InterfaceKind {
    NonDefault,
    Default,
    Overridable,
    Static,
    Composable,
}
