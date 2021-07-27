use super::*;

#[derive(Clone)]
pub struct ModuleRef(pub Row);

impl ModuleRef {
    pub fn name(&self) -> &'static str {
        self.0.str(0)
    }
}
