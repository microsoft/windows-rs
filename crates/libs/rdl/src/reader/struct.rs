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

#[derive(Clone, Copy)]
pub struct Enclosing {
    outer: metadata::writer::TypeDef,
    arch: Option<i32>,
}

impl Encoder<'_> {
    pub fn encode_struct(&mut self, item: &Struct) -> Result<(), Error> {
        let name = item.name.to_string();
        let type_def =
            self.encode_record(&name, item.winrt, false, &item.fields, &item.attrs, None)?;
        self.origin(type_def, &item.name);
        Ok(())
    }

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

        // Nested records inherit the parent's architecture omitted from RDL.
        let effective_arch = self.read_arch(attrs)?.or(enclosing.and_then(|e| e.arch));
        self.apply_record_attrs(type_def, attrs, effective_arch)?;

        // Defer nested TypeDefs until this record's FieldList is complete.
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
                    // Empty-namespace leaf names are resolved through NestedClass.
                    let child_name = format!("{name}_{index}");
                    let mt = metadata::Type::value_named("", &child_name);
                    deferred.push((child_name, rec));
                    mt
                }
            };
            let field_id = self
                .output
                .Field(&field_name, &mt, metadata::FieldAttributes::Public);
            self.origin(field_id, &field.name);
            if is_union {
                let layout = self.output.FieldLayout(field_id, 0);
                self.origin(layout, &field.name);
            }
            self.encode_attrs(
                metadata::writer::HasAttribute::Field(field_id),
                &field.attrs,
                &[],
            )?;

            // Anonymous padding advances the bit offset but emits no attribute.
            let mut bit_offset = 0u32;
            for member in &field.bitfields {
                if let Some(name) = &member.name {
                    self.emit_bitfield_attribute(
                        metadata::writer::HasAttribute::Field(field_id),
                        &name.unraw_to_string(),
                        bit_offset,
                        member.width,
                    );
                }
                bit_offset += member.width;
            }
        }

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

    pub fn apply_record_attrs(
        &mut self,
        type_def: metadata::writer::TypeDef,
        attrs: &[syn::Attribute],
        arch_bits: Option<i32>,
    ) -> Result<(), Error> {
        if let Some(packing_size) = self.read_packed(attrs)? {
            let layout = self.output.ClassLayout(type_def, packing_size, 0);
            let attribute = attrs
                .iter()
                .find(|attribute| attribute.path().is_ident("packed"))
                .unwrap();
            self.origin(layout, attribute);
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
