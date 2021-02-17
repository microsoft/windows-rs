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

    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        self.kind.dependencies()
    }

    pub fn gen_field(&self, gen: Gen) -> TokenStream {
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
}
