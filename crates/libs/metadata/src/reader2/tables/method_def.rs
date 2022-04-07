use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MethodDef<'a>(pub Row<'a>);

impl<'a> MethodDef<'a> {
    pub fn impl_flags(&self) -> MethodImplAttributes {
        MethodImplAttributes(self.0.usize(0))
    }
    pub fn flags(&self) -> MethodAttributes {
        MethodAttributes(self.0.usize(1))
    }
    pub fn name(&self) -> &str {
        self.0.str(3)
    }
    pub fn signature(&self) -> Blob {
        self.0.blob(4)
    }
    pub fn params(&self) -> impl Iterator<Item = Param> {
        self.0.list(TABLE_PARAM, 4).map(Param)
    }
}
