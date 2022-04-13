use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeSpec(pub Row);

// impl TypeSpec {
//     pub fn signature<'a>(&self, scope: &'a Reader) -> Blob<'a> {
//         scope.blob(self.0, 0)
//     }
// }
