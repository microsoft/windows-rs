use super::*;

/// True for `struct { ... } field;` records that are emitted inline, not hoisted.
///
/// Array/pointer-wrapped inline records still hoist because RDL cannot nest them there.
pub fn is_named_instance_record(record: &Cursor) -> bool {
    let kind = record.kind();
    if kind != CXCursor_StructDecl && kind != CXCursor_UnionDecl {
        return false;
    }
    // Field-less anonymous aggregates and named nested types use other paths.
    if !record.is_definition() || record.is_anonymous_record() || !is_anonymous_name(&record.name())
    {
        return false;
    }
    let parent = record.semantic_parent();
    if parent.kind() != CXCursor_StructDecl && parent.kind() != CXCursor_UnionDecl {
        return false;
    }
    // Only a direct field type is emitted inline; arrays/pointers stay hoisted.
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
    pub opaque: bool,
    /// Non-zero packing size in bytes, or `None` for natural alignment.
    pub packing: Option<u16>,
    /// Forced over-alignment in bytes; mutually exclusive with `packing`.
    pub alignment: Option<u16>,
}

impl Struct {
    /// Build an opaque struct for a forward declaration referenced through pointers.
    pub fn opaque(name: &str) -> Self {
        Self {
            name: name.to_string(),
            fields: vec![],
            is_union: false,
            opaque: true,
            packing: None,
            alignment: None,
        }
    }

    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>, is_union: bool) -> Result<Self, Error> {
        let tag_name = cursor.name();
        // Use the public typedef alias; anonymous types are keyed by source location.
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

        // Packing lowers the struct alignment below its largest field alignment.
        let struct_align_bytes = cursor.ty().align_of();
        let mut max_field_align_bytes: i64 = 0;

        // Coalesce consecutive bit-fields into backing integer fields; winmd has no
        // bit-field concept.
        let mut bitfield_indices: Vec<usize> = vec![];
        let mut unit_size: i64 = 0;
        let mut remaining_bits: i64 = 0;

        // Names anonymous aggregate fields in declaration order.
        let mut anonymous_count: usize = 0;

        // Names C++ base subobjects in declaration order.
        let mut base_count: usize = 0;

        for child in cursor.children() {
            // C++ base subobjects sit at the front of the layout; emit leading fields.
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

            // Reconstruct field-less anonymous aggregates inline as nested records.
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
                // Anonymous aggregate members contribute to the parent's natural alignment.
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

            // Emit `struct { ... } field;` inline so the reader rebuilds a nested type.
            let decl = child.ty().ty();
            if is_named_instance_record(&decl) {
                unit_size = 0;
                remaining_bits = 0;
                let child_is_union = decl.kind() == CXCursor_UnionDecl;
                let nested = Self::parse(decl, parser, child_is_union)?;
                fields.push(Field {
                    name: demacro_member_name(child.name(), parser.macro_defs),
                    ty: metadata::Type::Void,
                    nested: Some(Box::new(nested)),
                    bitfields: vec![],
                });
                continue;
            }

            if child.is_bit_field() {
                let width = child.bit_field_width() as i64;
                if width <= 0 {
                    // A zero-width bit-field only forces a fresh storage unit.
                    unit_size = 0;
                    remaining_bits = 0;
                    continue;
                }

                let size = child.ty().size_of();
                let member = demacro_member_name(child.name(), parser.macro_defs);
                if size != unit_size || width > remaining_bits {
                    // New storage units use the bit-field's declared signedness.
                    let ty = child.ty().to_type(parser);
                    bitfield_indices.push(fields.len());
                    // Anonymous padding consumes bits but gets no accessor.
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
                    // Continue filling the open unit; padding advances the offset only.
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

            unit_size = 0;
            remaining_bits = 0;

            let name = demacro_member_name(child.name(), parser.macro_defs);
            let ty = child.ty().to_type(parser);
            fields.push(Field {
                name,
                ty,
                nested: None,
                bitfields: vec![],
            });
        }

        // Name backing fields after the total count is known.
        if bitfield_indices.len() == 1 {
            fields[bitfield_indices[0]].name = "_bitfield".to_string();
        } else {
            for (n, &index) in bitfield_indices.iter().enumerate() {
                fields[index].name = format!("_bitfield{}", n + 1);
            }
        }

        // Emit packing only when it lowers natural alignment.
        let packing = if struct_align_bytes > 0 && max_field_align_bytes > struct_align_bytes {
            Some(struct_align_bytes as u16)
        } else {
            None
        };

        // Record forced over-alignment separately because `ClassLayout` can only lower it.
        let alignment = if struct_align_bytes > 0 && struct_align_bytes > max_field_align_bytes {
            Some(struct_align_bytes as u16)
        } else {
            None
        };

        Ok(Self {
            name,
            fields,
            is_union,
            opaque: false,
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

    /// Emit this record inline as the type of an anonymous nested field.
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

    /// The record's layout attributes.
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
                // RDL bit-field syntax uses implicit offsets; gaps become padding.
                if !field.bitfields.is_empty() {
                    let ty = write_type(namespace, &field.ty);
                    let mut members = vec![];
                    let mut cursor = 0u32;
                    for (member, offset, width) in &field.bitfields {
                        if *offset > cursor {
                            let pad = Literal::u32_unsuffixed(offset - cursor);
                            members.push(quote! { _: #pad, });
                        }
                        let member = write_ident(member);
                        let width_lit = Literal::u32_unsuffixed(*width);
                        members.push(quote! { #member: #width_lit, });
                        cursor = offset + width;
                    }
                    return quote! { #name: #ty { #(#members)* }, };
                }
                if let Some(nested) = &field.nested {
                    let inner = nested.write_inline(namespace);
                    quote! { #name: #inner, }
                } else {
                    let ty = write_type(namespace, &field.ty);
                    quote! { #name: #ty, }
                }
            })
            .collect()
    }
}
