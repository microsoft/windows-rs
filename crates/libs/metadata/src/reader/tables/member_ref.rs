use super::*;

impl std::fmt::Debug for MemberRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("MemberRef").field(&self.0).finish()
    }
}

impl<'a> MemberRef<'a> {
    pub fn parent(&self) -> MemberRefParent<'a> {
        self.decode(0)
    }

    pub fn name(&self) -> &'a str {
        self.str(1)
    }

    pub fn signature(&self, generics: &[Type]) -> Signature {
        self.blob(2).read_method_signature(generics)
    }
}
