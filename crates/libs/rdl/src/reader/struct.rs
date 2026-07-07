use super::*;

#[derive(Debug, Clone)]
pub struct Struct {
    pub attrs: Vec<syn::Attribute>,
    pub span: Span,
    pub name: syn::Ident,
    pub fields: Vec<Field>,
    pub winrt: bool,
}

impl syn::parse::Parse for Struct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token: syn::Token![struct] = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);

        let fields = content
            .parse_terminated(Field::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            span: token.span,
            name,
            fields,
            winrt: false,
        })
    }
}

/// The enclosing context of an inline nested record: the parent `TypeDef` that
/// owns the `NestedClass` relationship, plus the parent's effective architecture
/// (a nested type cannot diverge in architecture from its parent, so it inherits
/// this when its own `#[arch]` is absent). `None` for a top-level type.
#[derive(Clone, Copy)]
pub struct Enclosing {
    outer: metadata::writer::TypeDef,
    arch: Option<i32>,
}

impl Encoder<'_> {
    pub fn encode_struct(&mut self, item: &Struct) -> Result<(), Error> {
        let name = item.name.to_string();
        self.encode_record(&name, item.winrt, false, &item.fields, &item.attrs, None)?;
        Ok(())
    }

    /// Recursively encodes a struct/union and any inline anonymous nested
    /// records it contains.
    ///
    /// `name` is the type's own metadata name; it also seeds the names of any
    /// inline nested children (`{name}_{index}`, matching the flat naming scheme
    /// used by `windows-bindgen` and the writer). `enclosing` carries the parent
    /// `TypeDef` for a nested record (`None` for a top-level type).
    ///
    /// The record's own attributes (`#[packed]`/`#[align]`/`#[arch]`) are applied
    /// immediately after its `TypeDef` is created, before any nested child — so
    /// the `ClassLayout` table stays sorted by parent (types are created in
    /// pre-order, parent before child). All of the record's own fields are then
    /// pushed contiguously *before* any nested child `TypeDef`, so the metadata
    /// `FieldList` ranges stay valid and every `NestedClass(inner, outer)` keeps
    /// `inner > outer`.
    ///
    /// `inherited_arch` carries the enclosing type's effective architecture. A
    /// nested type cannot diverge in architecture from its parent, so the RDL
    /// omits the redundant `#[arch]` on inline nested records; the winmd still
    /// needs it (bindgen hoists nested types to arch-gated flat helpers), so a
    /// nested record with no explicit `#[arch]` inherits the parent's here.
    pub fn encode_record(
        &mut self,
        name: &str,
        winrt: bool,
        is_union: bool,
        fields: &[Field],
        attrs: &[syn::Attribute],
        enclosing: Option<Enclosing>,
    ) -> Result<metadata::writer::TypeDef, Error> {
        let value_type = self.output.TypeRef("System", "ValueType");

        let layout_flag = if is_union {
            metadata::TypeAttributes::ExplicitLayout
        } else {
            metadata::TypeAttributes::SequentialLayout
        };

        let mut flags = layout_flag
            | metadata::TypeAttributes::Sealed
            | if winrt {
                metadata::TypeAttributes::WindowsRuntime
            } else {
                metadata::TypeAttributes::default()
            };

        // A nested type lives in the empty namespace and is `NestedPublic`; a
        // top-level type lives in the encoder's namespace and is `Public`.
        let namespace = if enclosing.is_some() {
            flags |= metadata::TypeAttributes::NestedPublic;
            ""
        } else {
            flags |= metadata::TypeAttributes::Public;
            self.namespace
        };

        let type_def = self.output.TypeDef(
            namespace,
            name,
            metadata::writer::TypeDefOrRef::TypeRef(value_type),
            flags,
        );

        if let Some(enclosing) = enclosing {
            self.output.NestedClass(type_def, enclosing.outer);
        }

        // A nested type inherits its enclosing type's architecture; the RDL omits
        // the redundant `#[arch]` on nested records, so fall back to the parent's.
        let effective_arch = self.read_arch(attrs)?.or(enclosing.and_then(|e| e.arch));
        self.apply_record_attrs(type_def, attrs, effective_arch)?;

        // Push all of this record's fields first, deferring nested children so
        // their `TypeDef`s (and fields) come after the complete field list.
        let mut deferred: Vec<(String, &NestedRecord)> = vec![];
        for (index, field) in fields.iter().enumerate() {
            let field_name = field.name.unraw_to_string();
            let mt = match &field.ty {
                FieldType::Type(ty) => {
                    let mt = self.encode_type(ty)?;
                    if winrt {
                        self.validate_type_is_winrt(ty, &mt)?;
                    }
                    mt
                }
                FieldType::Nested(rec) => {
                    // The nested child is referenced by its bare leaf name in the
                    // empty namespace; the writer resolves this back to inline
                    // syntax and `windows-bindgen` resolves it via the enclosing
                    // type's nested-class list.
                    let child_name = format!("{name}_{index}");
                    let mt = metadata::Type::value_named("", &child_name);
                    deferred.push((child_name, rec));
                    mt
                }
            };
            let field_id = self
                .output
                .Field(&field_name, &mt, metadata::FieldAttributes::Public);
            if is_union {
                self.output.FieldLayout(field_id, 0);
            }
            self.encode_attrs(
                metadata::writer::HasAttribute::Field(field_id),
                &field.attrs,
                &[],
            )?;
        }

        // Encode the deferred nested children (depth-first) now that the parent's
        // field list is complete.
        for (child_name, rec) in deferred {
            self.encode_record(
                &child_name,
                winrt,
                rec.is_union,
                &rec.fields,
                &rec.attrs,
                Some(Enclosing {
                    outer: type_def,
                    arch: effective_arch,
                }),
            )?;
        }

        Ok(type_def)
    }

    /// Applies `#[packed(N)]` / `#[align(N)]` and any residual custom attributes
    /// to a record `TypeDef`, plus the effective architecture (`arch_bits`, which
    /// the caller resolves from the record's own `#[arch]` or the parent's).
    pub fn apply_record_attrs(
        &mut self,
        type_def: metadata::writer::TypeDef,
        attrs: &[syn::Attribute],
        arch_bits: Option<i32>,
    ) -> Result<(), Error> {
        if let Some(packing_size) = self.read_packed(attrs)? {
            self.output.ClassLayout(type_def, packing_size, 0);
        }

        if let Some(alignment) = self.read_align(attrs)? {
            self.emit_align_attribute(metadata::writer::HasAttribute::TypeDef(type_def), alignment);
        }

        if let Some(arch_bits) = arch_bits {
            self.emit_arch_attribute(metadata::writer::HasAttribute::TypeDef(type_def), arch_bits);
        }

        self.encode_attrs(
            metadata::writer::HasAttribute::TypeDef(type_def),
            attrs,
            &["packed", "align"],
        )
    }
}
