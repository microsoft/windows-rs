use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Param<'a>(pub Row<'a>);

impl<'a> Param<'a> {
    pub fn flags(&self) -> ParamAttributes {
        ParamAttributes(self.0.usize(0))
    }

    pub fn sequence(&self) -> usize {
        self.0.usize(1)
    }

    pub fn name(&self) -> &str {
        self.0.str(2)
    }
}
