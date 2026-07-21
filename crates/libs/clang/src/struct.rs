use super::*;

/// Returns `true` when `record` is an anonymous struct/union defined inline as the
/// direct (non-array, non-pointer) type of a named field of its enclosing record —
/// the "named instance" idiom `struct { … } field;`. Such a record is emitted inline
/// as a nested field (`field: struct { … }`) by [`Struct::parse`], producing a real
/// `NestedClass`, so it must **not** also be hoisted to a `{Outer}_{n}` sibling.
///
/// The C11 field-less anonymous aggregate (`struct { … };`) is handled separately
/// (it reports `is_anonymous_record()`), and array/pointer-wrapped inline records
/// (`struct { … } arr[N];` / `*p;`) keep the hoisting path because the RDL grammar
/// cannot express a nested record inside an array/pointer type.
pub fn is_named_instance_record(record: &Cursor) -> bool {
    let kind = record.kind();
    if kind != CXCursor_StructDecl && kind != CXCursor_UnionDecl {
        return false;
    }
    // A field-less C11 anonymous aggregate is inlined via the `is_anonymous_record`
    // path; a named nested type keeps its own identity and stays hoisted.
    if !record.is_definition() || record.is_anonymous_record() || !is_anonymous_name(&record.name())
    {
        return false;
    }
    let parent = record.semantic_parent();
    if parent.kind() != CXCursor_StructDecl && parent.kind() != CXCursor_UnionDecl {
        return false;
    }
    // The record is a named instance iff some field of the parent has it *directly*
    // as its type. A field wrapping it in an array/pointer yields a different type
    // declaration (or none), so those correctly fail this test and stay hoisted.
    let loc = record.location_id();
    parent
        .children()
        .into_iter()
        .any(|c| c.kind() == CXCursor_FieldDecl && c.ty().ty().location_id() == loc)
}

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub is_union: bool,
    /// Non-zero packing size in bytes (e.g. 1 for `#pragma pack(push, 1)`),
    /// or `None` when the type uses its natural alignment.
    pub packing: Option<u16>,
    /// Forced over-alignment in bytes from `__declspec(align(N))` / `alignas(N)`
    /// when the declared alignment is *raised above* the natural field
    /// alignment, or `None` otherwise. Mutually exclusive with `packing`.
    pub alignment: Option<u16>,
}

impl Struct {
    /// Builds an opaque (empty) struct for a record that is only ever forward-declared
    /// and never defined in the translation unit (e.g. `struct NDR_ALLOC_ALL_NODES_CONTEXT;`).
    /// Such incomplete types are referenced only through pointers; emitting an empty struct
    /// lets those references resolve, mirroring the opaque structs in the canonical metadata.
    pub fn opaque(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fields: vec![],
            is_union: false,
            packing: None,
            alignment: None,
        }
    }

    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>, is_union: bool) -> Result<Self, Error> {
        let tag_name = cursor.name();
        // Use the public typedef alias if one exists (e.g. `_TEST` → `TEST`).
        // For anonymous types the spelling is empty; fall back to location_id.
        let name = if is_anonymous_name(&tag_name) {
            parser
                .tag_rename
                .get(&cursor.location_id())
                .cloned()
                .unwrap_or(tag_name)
        } else {
            parser
                .tag_rename
                .get(&tag_name)
                .cloned()
                .unwrap_or(tag_name)
        };
        let mut fields = vec![];

        // The struct is packed when its alignment is strictly less than the maximum
        // natural alignment of its fields; the packing factor is then the struct's own
        // alignment in bytes (as reported under `#pragma pack(N)`).
        let struct_align_bytes = cursor.ty().align_of();
        let mut max_field_align_bytes: i64 = 0;

        // Bit-field coalescing state.  The winmd format has no notion of a
        // bit-field, so consecutive bit-fields are merged into backing integer
        // fields (named `_bitfield` / `_bitfield1` / `_bitfield2` …) exactly as
        // the field sequence packs them into storage units.  `unit_size` is the
        // byte size of the storage unit currently being filled (0 when none is
        // open) and `remaining_bits` how many bits are still free in it.
        let mut bitfield_indices: Vec<usize> = vec![];
        let mut unit_size: i64 = 0;
        let mut remaining_bits: i64 = 0;

        // Count of anonymous record members seen so far, used to name the
        // synthetic fields `Anonymous`, `Anonymous2`, … in declaration order.
        let mut anonymous_count: usize = 0;

        // Count of C++ base subobjects seen so far (the Win32 `*EX`/`*2`/`*3`
        // extension structs derive from their base via `: public Base`).
        let mut base_count: usize = 0;

        for child in cursor.children() {
            // A C++ base class (`struct tagMONITORINFOEXA : public tagMONITORINFO`)
            // produces a `CXCursor_CXXBaseSpecifier`, not a `FieldDecl`, so its
            // members would otherwise be dropped — the struct would be too small.
            // The base subobject sits at the front of the layout, so emit it as a
            // leading field of the base type (named `Base`, `Base2`, …), matching
            // both the C anonymous-member spelling (`MONITORINFO;`) and the
            // single-field-per-base layout the canonical metadata records.
            if child.kind() == CXCursor_CXXBaseSpecifier {
                unit_size = 0;
                remaining_bits = 0;
                base_count += 1;
                let name = if base_count == 1 {
                    "Base".to_string()
                } else {
                    format!("Base{base_count}")
                };
                let field_align = child.ty().align_of();
                if field_align > max_field_align_bytes {
                    max_field_align_bytes = field_align;
                }
                let ty = child.ty().to_type(parser);
                fields.push(Field {
                    name,
                    ty,
                    nested: None,
                    bitfields: vec![],
                });
                continue;
            }

            // An anonymous struct/union member (the C11 / MSVC anonymous aggregate
            // idiom) produces no `FieldDecl`; libclang promotes its members and
            // exposes only the nested record declaration. Reconstruct it inline as
            // a nested record (`Anonymous: struct { … }`) so the RDL reader rebuilds
            // it as a true nested type (`NestedClass`) instead of a hoisted
            // `{Outer}_{n}` sibling. The parent's layout stays faithful.
            if matches!(child.kind(), CXCursor_StructDecl | CXCursor_UnionDecl)
                && child.is_anonymous_record()
            {
                unit_size = 0;
                remaining_bits = 0;
                anonymous_count += 1;
                let name = if anonymous_count == 1 {
                    "Anonymous".to_string()
                } else {
                    format!("Anonymous{anonymous_count}")
                };
                // An anonymous aggregate member contributes its own alignment to the
                // parent's natural alignment just like a named field; without this it
                // would be undercounted and a struct whose largest alignment comes from
                // an anonymous member (e.g. `STRRET`'s 8-aligned union) would be wrongly
                // flagged as forced-over-aligned.
                let field_align = child.ty().align_of();
                if field_align > max_field_align_bytes {
                    max_field_align_bytes = field_align;
                }
                let child_is_union = child.kind() == CXCursor_UnionDecl;
                let nested = Self::parse(child, parser, child_is_union)?;
                fields.push(Field {
                    name,
                    ty: metadata::Type::Void,
                    nested: Some(Box::new(nested)),
                    bitfields: vec![],
                });
                continue;
            }

            if child.kind() != CXCursor_FieldDecl {
                continue;
            }

            let field_align = child.ty().align_of();
            if field_align > max_field_align_bytes {
                max_field_align_bytes = field_align;
            }

            // A named instance of an inline anonymous record (`struct { … } field;`)
            // is emitted inline as a nested field so the RDL reader rebuilds it as a
            // real nested type (`NestedClass`) instead of a hoisted `{Outer}_{n}`
            // sibling. The record declaration is the field's own type declaration.
            let decl = child.ty().ty();
            if is_named_instance_record(&decl) {
                // Close any open bit-field storage unit.
                unit_size = 0;
                remaining_bits = 0;
                let child_is_union = decl.kind() == CXCursor_UnionDecl;
                let nested = Self::parse(decl, parser, child_is_union)?;
                fields.push(Field {
                    name: child.name(),
                    ty: metadata::Type::Void,
                    nested: Some(Box::new(nested)),
                    bitfields: vec![],
                });
                continue;
            }

            if child.is_bit_field() {
                let width = child.bit_field_width() as i64;
                if width <= 0 {
                    // A zero-width bit-field (`int : 0`) carries no member; it
                    // only forces the following field onto a fresh storage unit.
                    unit_size = 0;
                    remaining_bits = 0;
                    continue;
                }

                let size = child.ty().size_of();
                let member = child.name();
                if size != unit_size || width > remaining_bits {
                    // Open a new storage unit, backed by the bit-field's own
                    // (possibly signed) declared type. The member sits at the low
                    // end of the fresh unit (offset 0).
                    let ty = child.ty().to_type(parser);
                    bitfield_indices.push(fields.len());
                    // An anonymous padding bit-field (`int : 4;`) consumes bits but
                    // names no member, so it gets no accessor.
                    let members = if member.is_empty() {
                        vec![]
                    } else {
                        vec![(member, 0, width as u32)]
                    };
                    fields.push(Field {
                        name: String::new(),
                        ty,
                        nested: None,
                        bitfields: members,
                    });
                    unit_size = size;
                    remaining_bits = size * 8 - width;
                } else {
                    // Continue filling the open unit: the member's offset is the
                    // number of bits already consumed in it. Anonymous padding
                    // bit-fields advance the offset but emit no accessor.
                    let offset = (unit_size * 8 - remaining_bits) as u32;
                    if !member.is_empty()
                        && let Some(&index) = bitfield_indices.last()
                    {
                        fields[index].bitfields.push((member, offset, width as u32));
                    }
                    remaining_bits -= width;
                }
                continue;
            }

            // A regular member closes any open bit-field storage unit.
            unit_size = 0;
            remaining_bits = 0;

            let name = child.name();
            let ty = child.ty().to_type(parser);
            fields.push(Field {
                name,
                ty,
                nested: None,
                bitfields: vec![],
            });
        }

        // Name the backing fields now that the total count is known: a lone
        // backing field is `_bitfield`; multiple are `_bitfield1`, `_bitfield2`,
        // … numbered in declaration order across the whole type.
        if bitfield_indices.len() == 1 {
            fields[bitfield_indices[0]].name = "_bitfield".to_string();
        } else {
            for (n, &index) in bitfield_indices.iter().enumerate() {
                fields[index].name = format!("_bitfield{}", n + 1);
            }
        }

        // Only emit packing when the struct's alignment is positive (valid) and
        // strictly less than the largest field's natural alignment.  This avoids
        // emitting `#[packed(N)]` for structs whose packing matches their natural
        // alignment (which is effectively a no-op and can be safely omitted).
        let packing = if struct_align_bytes > 0 && max_field_align_bytes > struct_align_bytes {
            Some(struct_align_bytes as u16)
        } else {
            None
        };

        // The inverse case: `__declspec(align(N))` / `alignas(N)` raises the
        // declared alignment *above* the natural field alignment (the `CONTEXT` /
        // #3761 class). The winmd `ClassLayout` can only lower alignment via its
        // packing size, so this forced over-alignment is recorded separately and
        // surfaced as `#[align(N)]`. The two are mutually exclusive: a type either
        // packs below or is forced above its natural alignment, never both.
        let alignment = if struct_align_bytes > 0 && struct_align_bytes > max_field_align_bytes {
            Some(struct_align_bytes as u16)
        } else {
            None
        };

        Ok(Self {
            name,
            fields,
            is_union,
            packing,
            alignment,
        })
    }

    pub fn write(&self, namespace: &str) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);
        let attrs = self.write_attrs();
        let keyword = self.write_keyword();
        let fields = self.write_fields(namespace);

        Ok(quote! {
            #attrs
            #keyword #name {
                #(#fields)*
            }
        })
    }

    /// Emits this record inline, without a name, as the type of an anonymous
    /// nested field: `#attrs struct { … }` / `#attrs union { … }`. The RDL
    /// reader turns this into a real nested type (`NestedClass`).
    fn write_inline(&self, namespace: &str) -> TokenStream {
        let attrs = self.write_attrs();
        let keyword = self.write_keyword();
        let fields = self.write_fields(namespace);

        quote! {
            #attrs #keyword {
                #(#fields)*
            }
        }
    }

    fn write_keyword(&self) -> TokenStream {
        if self.is_union {
            quote! { union }
        } else {
            quote! { struct }
        }
    }

    /// The record's own `#[packed(N)]` / `#[align(N)]` attributes.
    fn write_attrs(&self) -> TokenStream {
        let packed_attr = if let Some(packing) = self.packing {
            let size = Literal::u16_unsuffixed(packing);
            quote! { #[packed(#size)] }
        } else {
            quote! {}
        };

        let align_attr = if let Some(alignment) = self.alignment {
            let size = Literal::u16_unsuffixed(alignment);
            quote! { #[align(#size)] }
        } else {
            quote! {}
        };

        quote! { #packed_attr #align_attr }
    }

    fn write_fields(&self, namespace: &str) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|field| {
                let name = write_ident(&field.name);
                let bitfields = field.bitfields.iter().map(|(member, offset, width)| {
                    let member = Literal::string(member);
                    let offset = Literal::u32_unsuffixed(*offset);
                    let width = Literal::u32_unsuffixed(*width);
                    quote! { #[bitfield(#member, #offset, #width)] }
                });
                if let Some(nested) = &field.nested {
                    let inner = nested.write_inline(namespace);
                    quote! { #(#bitfields)* #name: #inner, }
                } else {
                    let ty = write_type(namespace, &field.ty);
                    quote! { #(#bitfields)* #name: #ty, }
                }
            })
            .collect()
    }
}
