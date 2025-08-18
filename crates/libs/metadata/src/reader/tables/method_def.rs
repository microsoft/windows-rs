use super::*;

impl std::fmt::Debug for MethodDef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("MethodDef").field(&self.name()).finish()
    }
}

impl<'a> MethodDef<'a> {
    pub fn rva(&self) -> usize {
        self.usize(0)
    }

    pub fn impl_flags(&self) -> MethodImplAttributes {
        MethodImplAttributes(self.usize(1).try_into().unwrap())
    }

    pub fn flags(&self) -> MethodAttributes {
        MethodAttributes(self.usize(2).try_into().unwrap())
    }

    pub fn name(&self) -> &'a str {
        self.str(3)
    }

    pub fn signature(&self, generics: &[Type]) -> Signature {
        self.blob(4).read_method_signature(generics)
    }

    pub fn params(&self) -> RowIterator<'a, MethodParam<'a>> {
        self.list(5)
    }

    pub fn parent(&self) -> MemberRefParent<'a> {
        MemberRefParent::TypeDef(self.parent_row(5))
    }

    pub fn impl_map(&self) -> Option<ImplMap<'a>> {
        self.equal_range(1, MemberForwarded::MethodDef(*self).encode())
            .next()
    }

    pub fn calling_convention(&self) -> &str {
        self.impl_map().map_or("", |map| {
            let flags = map.flags();

            if flags.contains(PInvokeAttributes::CallConvPlatformapi) {
                "system"
            } else if flags.contains(PInvokeAttributes::CallConvCdecl) {
                "C"
            } else {
                ""
            }
        })
    }
}
