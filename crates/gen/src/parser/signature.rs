use super::*;

// TODO: this replaces gen::Type
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Signature {
    pub kind: ElementType,
    pub pointers: usize,
    pub by_ref: bool,
    pub is_const: bool,
    pub is_array: bool,
}

impl Signature {
    pub fn from_blob(blob: &mut Blob, generics: &[ElementType]) -> Option<Self> {
        let is_const = blob
            .read_modifiers()
            .iter()
            .any(|def| def.full_name() == ("System.Runtime.CompilerServices", "IsConst"));

        let by_ref = blob.read_expected(0x10);

        if blob.read_expected(0x01) {
            return None;
        }

        let is_array = blob.read_expected(0x1D);

        let mut pointers = 0;

        while blob.read_expected(0x0f) {
            pointers += 1;
        }

        let kind = ElementType::from_blob(blob, generics);

        Some(Self {
            kind,
            pointers,
            by_ref,
            is_const,
            is_array,
        })
    }

    pub fn definition(&self) -> Option<tables::TypeDef> {
        self.kind.definition()
    }

    pub fn is_blittable(&self) -> bool {
        // TODO: In theory, this should deal with "tree" structs that point to themselves - need a test for this.
        self.pointers > 0 || self.kind.is_blittable()
    }

    pub fn gen(&self, gen: Gen) -> TokenStream {
        let mut tokens = TokenStream::new();

        for _ in 0..self.pointers {
            if self.is_const {
                tokens.combine(&quote! { *const });
            } else {
                tokens.combine(&quote! { *mut });
            }
        }

        let kind = self.kind.gen_name(gen);

        if self.kind.is_nullable() {
            tokens.combine(&quote! {
                ::std::option::Option<#kind>
            });
        } else {
            tokens.combine(&kind)
        }

        tokens
    }

    pub fn gen_abi(&self, gen: Gen) -> TokenStream {
        let mut tokens = TokenStream::new();

        for _ in 0..self.pointers {
            if self.is_const {
                tokens.combine(&quote! { *const });
            } else {
                tokens.combine(&quote! { *mut });
            }
        }

        tokens.combine(&self.kind.gen_abi(gen));
        tokens
    }

    pub fn gen_default(&self) -> TokenStream {
        if self.pointers > 0 {
            quote! { ::std::ptr::null_mut() }
        } else {
            self.kind.gen_default()
        }
    }
}
