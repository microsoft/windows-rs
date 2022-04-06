use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct GenericParam<'a>(pub Row<'a>);

impl<'a> GenericParam<'a> {}
