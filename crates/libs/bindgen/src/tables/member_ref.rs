use super::*;

impl std::fmt::Debug for MemberRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MemberRef").field(&self.name()).finish()
    }
}

impl MemberRef {
    pub fn parent(&self) -> MemberRefParent {
        self.decode(0)
    }

    pub fn name(&self) -> &'static str {
        self.str(1)
    }
}
