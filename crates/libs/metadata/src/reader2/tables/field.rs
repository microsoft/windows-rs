use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Field<'a>(pub Row<'a>);

impl<'a> Field<'a> {
    pub fn flags(&self) -> FieldAttributes {
        FieldAttributes(self.0.usize(0))
    }
    pub fn name(&self) -> &str {
        self.0.str(1)
    }
    pub fn signature(&self) -> Blob {
        self.0.blob(2)
    }
}
