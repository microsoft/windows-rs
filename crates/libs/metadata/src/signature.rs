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
    pub fn is_generic(&self) -> bool {
        matches!(self.kind, ElementType::GenericParam(_))
    }

    pub fn is_blittable(&self) -> bool {
        self.pointers > 0 || self.kind.is_blittable()
    }

    pub fn is_udt(&self) -> bool {
        self.pointers == 0 && self.kind.is_udt()
    }

    pub fn has_union(&self) -> bool {
        self.pointers == 0 && self.kind.has_union()
    }

    pub fn has_pack(&self) -> bool {
        self.pointers == 0 && self.kind.has_pack()
    }

    pub fn is_callback(&self) -> bool {
        self.pointers == 0 && self.kind.is_callback()
    }

    pub fn is_callback_array(&self) -> bool {
        self.pointers == 0 && self.kind.is_callback_array()
    }

    pub fn size(&self) -> usize {
        if self.pointers > 0 {
            1
        } else {
            self.kind.size()
        }
    }

    pub fn is_primitive(&self) -> bool {
        self.pointers > 0 || self.kind.is_primitive()
    }
}
