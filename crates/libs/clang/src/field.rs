use super::*;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub ty: metadata::Type,
    /// When this field is a C anonymous nested struct/union member (the C11 /
    /// MSVC anonymous aggregate idiom), the reconstructed nested record. It is
    /// emitted inline as `Anonymous: struct { ... }` (see [`Struct::write`]) so the
    /// RDL reader can rebuild it as a real nested type (`NestedClass`) rather than
    /// a hoisted `{Outer}_{n}` sibling. `None` for an ordinary field, whose type
    /// is carried by [`ty`](Self::ty).
    pub nested: Option<Box<Struct>>,
    /// When this field is a synthetic bit-field storage unit (`_bitfield` /
    /// `_bitfield1` ...), the logical members packed into it, each recorded as
    /// `(name, offset, width)` where `offset` is the bit position from the low
    /// end of the backing integer and `width` the member's bit count. Empty for
    /// an ordinary field. Emitted as `#[bitfield(name, offset, width)]` outer
    /// attributes (see [`Struct::write`]) so windows-bindgen can generate typed
    /// accessors; the winmd format itself has no notion of a bit-field.
    pub bitfields: Vec<(String, u32, u32)>,
}

/// A function or method parameter with an optional SAL annotation.
///
/// Unlike [`Field`], which represents a struct field and carries no direction
/// metadata, `Param` also records the [`ParamAnnotation`] extracted from any
/// SAL attribute on the parameter declaration.
#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub ty: metadata::Type,
    /// SAL-derived direction/optional flags.  All fields are `false` when no
    /// SAL annotation was detected for this parameter.
    pub annotation: ParamAnnotation,
}
