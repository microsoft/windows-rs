use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Constant<'a>(pub Row<'a>);

impl<'a> Constant<'a> {
    pub fn ty(&self) -> Type {
        todo!()
    }
    pub fn value(&self) -> Blob {
        self.0.blob(2)
    }
}
