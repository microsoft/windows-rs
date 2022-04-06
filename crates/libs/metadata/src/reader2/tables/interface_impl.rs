use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct InterfaceImpl<'a>(pub Row<'a>);

impl<'a> InterfaceImpl<'a> {}
