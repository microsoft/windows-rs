use super::*;

impl std::fmt::Debug for MethodParam<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("MethodParam").field(&self.name()).finish()
    }
}

impl MethodParam<'_> {
    pub fn flags(&self) -> ParamAttributes {
        ParamAttributes(self.usize(0).try_into().unwrap())
    }

    pub fn sequence(&self) -> u16 {
        self.usize(1).try_into().unwrap()
    }

    pub fn name(&self) -> &str {
        self.str(2)
    }
}
