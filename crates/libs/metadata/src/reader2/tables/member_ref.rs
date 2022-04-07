use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MemberRef<'a>(pub Row<'a>);

impl<'a> MemberRef<'a> {
    pub fn parent(&self) -> MemberRefParent {
        self.0.decode(0)
    }
}
