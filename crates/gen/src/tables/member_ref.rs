use super::*;

#[derive(Clone)]
pub struct MemberRef(pub Row);

impl MemberRef {
    pub fn parent(&self) -> MemberRefParent {
        self.0.decode(0)
    }

    pub fn name(&self) -> &'static str {
        self.0.str(1)
    }
}

impl std::fmt::Debug for MemberRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
