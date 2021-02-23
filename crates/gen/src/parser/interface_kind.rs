#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum InterfaceKind {
    Default,
    NonDefault,
    Overrides,
    Statics,
    Composable,
}
