use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct AssemblyRef(pub Row);

impl std::fmt::Debug for AssemblyRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssemblyRef").field("row", &self.0).finish()
    }
}
