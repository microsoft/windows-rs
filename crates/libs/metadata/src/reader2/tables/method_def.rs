use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MethodDef<'a>(pub Row<'a>);

impl<'a> MethodDef<'a> {
    pub fn name(&self) -> &str {
        self.0.str(3)
    }
}
