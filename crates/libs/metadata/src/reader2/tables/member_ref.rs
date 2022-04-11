use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MemberRef<'a>(pub Row<'a>);

impl<'a> MemberRef<'a> {
    pub fn parent(&self) -> MemberRefParent<'a> {
        self.0.decode(0)
    }
    pub fn signature(&self) -> Blob {
        self.0.blob(2)
    }
}
