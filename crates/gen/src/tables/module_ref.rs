use super::*;

#[derive(Clone)]
pub struct ModuleRef(pub Row);

impl ModuleRef {
    pub fn name(&self) -> &'static str {
        self.0.str(0)
    }
}

impl std::fmt::Debug for ModuleRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
