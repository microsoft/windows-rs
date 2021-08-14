use super::*;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Default)]
pub struct Signature {
    pub kind: ElementType,
    pub pointers: usize,
    pub by_ref: bool,
    pub is_const: bool,
    pub is_array: bool,
}

impl Signature {
    pub fn definition(&self, include: TypeInclude) -> Vec<TypeEntry> {
        self.kind.definition(include)
    }

    pub fn dependencies(&self, include: TypeInclude) -> Vec<TypeEntry> {
        self.kind.dependencies(include)
    }

    pub fn is_blittable(&self) -> bool {
        self.pointers > 0 || self.kind.is_blittable()
    }

    pub fn is_udt(&self) -> bool {
        self.pointers == 0 && self.kind.is_udt()
    }

    pub fn has_explicit(&self) -> bool {
        self.pointers == 0 && self.kind.has_explicit()
    }

    pub fn is_packed(&self) -> bool {
        if self.pointers > 0 {
            return false;
        }

        match &self.kind {
            ElementType::TypeDef(def) => def.is_packed(),
            ElementType::Array((signature, _)) => signature.is_packed(),
            _ => false,
        }
    }

    pub fn gen_win32(&self, gen: &Gen) -> TokenStream {
        let mut tokens = TokenStream::new();

        // TODO: this isn't correct since the signature alone isn't enough to tell whether its const - the param might be const as well
        for _ in 0..self.pointers {
            if self.is_const {
                tokens.combine(&quote! { *const });
            } else {
                tokens.combine(&quote! { *mut });
            }
        }

        let kind = gen_name(&self.kind, gen);

        if self.kind.is_nullable() {
            tokens.combine(&quote! {
                ::std::option::Option<#kind>
            });
        } else {
            tokens.combine(&kind)
        }

        tokens
    }

    pub fn gen_winrt(&self, gen: &Gen) -> TokenStream {
        let mut tokens = TokenStream::new();

        // TODO: this isn't correct since the signature alone isn't enough to tell whether its const - the param might be const as well
        for _ in 0..self.pointers {
            if self.is_const {
                tokens.combine(&quote! { *const });
            } else {
                tokens.combine(&quote! { *mut });
            }
        }

        let kind = gen_name(&self.kind, gen);

        if self.kind.is_nullable() {
            tokens.combine(&quote! {
                ::std::option::Option<#kind>
            });
        } else {
            tokens.combine(&kind)
        }

        tokens
    }

    pub fn gen_win32_abi(&self, gen: &Gen) -> TokenStream {
        let mut tokens = TokenStream::new();

        // TODO: this isn't correct since the signature alone isn't enough to tell whether its const - the param might be const as well
        for _ in 0..self.pointers {
            if self.is_const {
                tokens.combine(&quote! { *const });
            } else {
                tokens.combine(&quote! { *mut });
            }
        }

        tokens.combine(&self.kind.gen_abi_type(gen));
        tokens
    }

    pub fn gen_winrt_abi(&self, gen: &Gen) -> TokenStream {
        let mut tokens = TokenStream::new();

        // TODO: this isn't correct since the signature alone isn't enough to tell whether its const - the param might be const as well
        for _ in 0..self.pointers {
            if self.is_const {
                tokens.combine(&quote! { *const });
            } else {
                tokens.combine(&quote! { *mut });
            }
        }

        tokens.combine(&self.kind.gen_abi_type(gen));
        tokens
    }

    pub fn gen_win32_default(&self) -> TokenStream {
        if self.pointers > 0 {
            quote! { ::std::ptr::null_mut() }
        } else {
            gen_default(&self.kind)
        }
    }

    pub fn gen_winrt_default(&self) -> TokenStream {
        if self.pointers > 0 {
            quote! { ::std::ptr::null_mut() }
        } else {
            gen_default(&self.kind)
        }
    }
}
