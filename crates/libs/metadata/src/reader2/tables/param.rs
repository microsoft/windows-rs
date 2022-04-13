use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Param(pub Row);

// impl Param {
//     pub fn flags(&self, scope: &Reader) -> ParamAttributes {
//         ParamAttributes(scope.usize(self.0, 0))
//     }
//     pub fn sequence(&self, scope: &Reader) -> usize {
//         scope.usize(self.0, 1)
//     }
//     pub fn name<'a>(&self, scope: &'a Reader) -> &'a str {
//         scope.str(self.0, 2)
//     }
// }
