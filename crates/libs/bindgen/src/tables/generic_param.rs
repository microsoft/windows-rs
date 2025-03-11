use super::*;

impl std::fmt::Debug for GenericParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("GenericParam").field(&self.name()).finish()
    }
}

impl GenericParam {
    pub fn sequence(&self) -> u16 {
        self.usize(0) as u16
    }

    pub fn flags(&self) -> u16 {
        self.usize(1) as u16
    }

    pub fn name(&self) -> &'static str {
        self.str(3)
    }
}
