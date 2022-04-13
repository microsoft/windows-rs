use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ImplMap(pub Row);

// impl ImplMap {
//     pub fn flags(&self, scope: &Reader) -> PInvokeAttributes {
//         PInvokeAttributes(scope.usize(self.0, 0))
//     }
//     pub fn scope(&self, scope: &Reader) -> ModuleRef {
//         ModuleRef(Row::new(scope.usize(self.0, 3) - 1, TABLE_MODULEREF, self.0.file as _))
//     }
// }
