use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ClassLayout(pub Row);

// impl ClassLayout {
//     pub fn packing_size(&self, scope: &Reader) -> usize {
//         scope.usize(self.0, 0)
//     }
// }
