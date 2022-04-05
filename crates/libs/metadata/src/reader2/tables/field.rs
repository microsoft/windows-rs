use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Field<'a>(pub Row<'a>);

impl<'a> Field<'a> {
    pub fn name(&self) -> &str {
        self.0.str(1)
    }
}
