use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ModuleRef(pub Row);

// impl ModuleRef {
//     pub fn name<'a>(&self, scope: &'a Reader) -> &'a str {
//         scope.str(self.0, 0)
//     }
// }
