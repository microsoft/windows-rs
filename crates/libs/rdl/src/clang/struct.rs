use super::*;

#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub is_union: bool,
    /// Non-zero packing size in bytes (e.g. 1 for `#pragma pack(push, 1)`),
    /// or `None` when the type uses its natural alignment.
    pub packing: Option<u16>,
}

impl Struct {
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

        // Determine packing by comparing the struct's actual alignment (in bytes)
        // against the maximum natural alignment of its fields.  When `#pragma pack(N)`
        // is in effect, `clang_Type_getAlignOf` reports `N` (bytes) for the struct while
        // individual field types retain their natural alignments.  If the struct
        // alignment is strictly less than the maximum field alignment the struct is
        // packed, and the packing factor is the struct's alignment in bytes.
        let struct_align_bytes = cursor.ty().align_of();
        let mut max_field_align_bytes: i64 = 0;

        for child in cursor.children() {
            if child.kind() != CXCursor_FieldDecl {
                continue;
            }

            let field_align = child.ty().align_of();
            if field_align > max_field_align_bytes {
                max_field_align_bytes = field_align;
            }

            let name = child.name();
            let ty = child.ty().to_type(parser);
            fields.push(Field { name, ty });
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

        Ok(Self {
            name,
            fields,
            is_union,
            packing,
        })
    }

    pub fn write(&self, namespace: &str) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);

        let fields = self.fields.iter().map(|field| {
            let name = write_ident(&field.name);
            let ty = write_type(namespace, &field.ty);
            quote! { #name: #ty, }
        });

        let keyword = if self.is_union {
            quote! { union }
        } else {
            quote! { struct }
        };

        let packed_attr = if let Some(packing) = self.packing {
            let size = proc_macro2::Literal::u16_unsuffixed(packing);
            quote! { #[packed(#size)] }
        } else {
            quote! {}
        };

        Ok(quote! {
            #packed_attr
            #keyword #name {
                #(#fields)*
            }
        })
    }
}
