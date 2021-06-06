use super::*;

#[derive(Clone)]
pub struct Module(pub Row);

impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module").field("row", &self.0).finish()
    }
}
