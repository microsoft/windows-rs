use super::*;

impl std::fmt::Debug for GenericParam<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("GenericParam").field(&self.name()).finish()
    }
}

impl<'a> GenericParam<'a> {
    pub fn sequence(&self) -> u16 {
        self.usize(0).try_into().unwrap()
    }

    pub fn flags(&self) -> GenericParamAttributes {
        GenericParamAttributes(self.usize(1).try_into().unwrap())
    }

    pub fn owner(&self) -> TypeOrMethodDef<'a> {
        self.decode(2)
    }

    pub fn name(&self) -> &str {
        self.str(3)
    }
}
