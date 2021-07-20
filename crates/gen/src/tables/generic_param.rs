use super::*;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct GenericParam(pub Row);

impl GenericParam {
    pub fn name(&self) -> &'static str {
        self.0.str(3)
    }
}

impl std::fmt::Debug for GenericParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
