use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ModuleRef<'a>(pub Row<'a>);

impl<'a> ModuleRef<'a> {
    pub fn name(&self) -> &str {
        self.0.str(0)
    }
}
