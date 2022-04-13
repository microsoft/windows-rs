use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct MemberRef(pub Row);

// impl MemberRef {
//     pub fn parent(&self, scope: &Reader) -> MemberRefParent {
//         scope.decode(self.0, 0)
//     }
//     pub fn signature<'a>(&self, scope: &'a Reader) -> Blob<'a> {
//         scope.blob(self.0, 2)
//     }
// }
