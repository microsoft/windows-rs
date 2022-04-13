use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct GenericParam(pub Row);

// impl GenericParam {
//     pub fn name<'a>(&self, scope: &'a Reader) -> &'a str {
//         scope.str(self.0, 3)
//     }
// }
