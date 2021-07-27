use super::*;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct GenericParam(pub Row);

impl GenericParam {
    pub fn name(&self) -> &'static str {
        self.0.str(3)
    }
}
