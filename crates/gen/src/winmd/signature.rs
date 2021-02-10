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
    pub fn from_blob(blob: &mut Blob) -> Option<Self> {
        Self::from_blob_with_generics(blob, &[])
    }

    pub fn from_blob_with_generics(blob: &mut Blob, generics: &[ElementType]) -> Option<Self> {
        let modifiers = blob.read_modifiers();
        let mut by_ref = blob.read_expected(0x10);

        if blob.read_expected(0x01) {
            return None;
        }

        let is_array = blob.read_expected(0x1D);

        let mut pointers = 0;

        while blob.read_expected(0x0f) {
            pointers += 1;
        }

        let kind = ElementType::from_blob_with_generics(blob, generics);

        let mut is_const = modifiers
            .iter()
            .any(|def| def.full_name() == ("System.Runtime.CompilerServices", "IsConst"));

        Some(Self {
            kind,
            pointers,
            by_ref,
            is_const,
            is_array,
        })
    }
}
