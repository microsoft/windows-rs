use super::*;

impl std::fmt::Debug for AssemblyRef {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("AssemblyRef").field(&self.0).finish()
    }
}
