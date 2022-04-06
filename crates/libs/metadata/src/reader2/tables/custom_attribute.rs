use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct CustomAttribute<'a>(pub Row<'a>);

impl<'a> CustomAttribute<'a> {
    pub fn definition(&self) -> CustomAttributeType {
        self.0.decode(1)
    }

    pub fn value(&self) -> Blob {
        self.0.blob(2)
    }
}
