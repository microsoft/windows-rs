use super::*;

#[derive(Debug, Clone)]
pub struct Struct {
    pub attrs: Vec<syn::Attribute>,
    pub span: Span,
    pub name: syn::Ident,
    pub body: RecordBody,
    pub winrt: bool,
}

impl syn::parse::Parse for Struct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token: syn::Token![struct] = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);

        let body = parse_record_body(&content)?;

        Ok(Self {
            attrs,
            span: token.span,
            name,
            body,
            winrt: false,
        })
    }
}

#[derive(Clone, Copy)]
pub struct Enclosing<'a> {
    outer: metadata::writer::TypeDef,
    arch: Option<i32>,
    name: &'a str,
    anonymous: Option<i32>,
}

impl Encoder<'_> {
    pub fn encode_struct(&mut self, item: &Struct) -> Result<(), Error> {
        let name = item.name.to_string();
        self.encode_record(&name, item.winrt, false, &item.body, &item.attrs, None)?;
        Ok(())
    }

    pub fn encode_record(
        &mut self,
        name: &str,
        winrt: bool,
        is_union: bool,
        body: &RecordBody,
        attrs: &[syn::Attribute],
        enclosing: Option<Enclosing<'_>>,
    ) -> Result<metadata::writer::TypeDef, Error> {
        let fields = &body.fields;
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

        let full_name = if let Some(enclosing) = enclosing {
            format!("{}/{name}", enclosing.name)
        } else {
            name.to_string()
        };

        if let Some(enclosing) = enclosing {
            self.output.NestedClass(type_def, enclosing.outer);
        }

        // Nested records inherit the parent's architecture omitted from RDL.
        let effective_arch = self.read_arch(attrs)?.or(enclosing.and_then(|e| e.arch));
        self.apply_record_attrs(type_def, attrs, effective_arch)?;
        let mut anonymous = None;
        for attr in attrs
            .iter()
            .filter(|attr| attr.path().is_ident("anonymous"))
        {
            if anonymous.is_some() || enclosing.is_none() || winrt {
                return self.err(
                    attr,
                    "anonymous provenance requires one attribute on a nested Win32 record",
                );
            }
            let index = attr
                .parse_args::<syn::LitInt>()
                .and_then(|value| value.base10_parse::<i32>())
                .map_err(|_| {
                    self.error(
                        attr,
                        "anonymous provenance requires a nonnegative field position",
                    )
                })?;
            if index < 0 {
                return self.err(
                    attr,
                    "anonymous provenance requires a nonnegative field position",
                );
            }
            if enclosing
                .and_then(|owner| owner.anonymous)
                .is_some_and(|implicit| implicit != index)
            {
                return self.err(
                    attr,
                    "anonymous position differs from the inline field position",
                );
            }
            anonymous = Some(index);
        }
        if !winrt && let Some(index) = enclosing.and_then(|owner| owner.anonymous) {
            anonymous = Some(index);
        }
        if let Some(index) = anonymous {
            self.encode_named_attribute(
                metadata::writer::HasAttribute::TypeDef(type_def),
                &attribute_ref::AttributeRef {
                    type_name: metadata::TypeName::named(
                        METADATA_NAMESPACE,
                        "NativeAnonymousAttribute",
                    ),
                    args: vec![(String::new(), metadata::Value::I32(index))],
                },
            );
        }

        // Defer nested TypeDefs until this record's FieldList is complete.
        let mut deferred: Vec<(String, &NestedRecord, Option<i32>)> = body
            .records
            .iter()
            .map(|record| {
                (
                    record.name.as_ref().unwrap().unraw_to_string(),
                    record,
                    None,
                )
            })
            .collect();
        let mut nested_names = HashSet::new();
        for (name, record, _) in &deferred {
            if !nested_names.insert(name.clone()) {
                return self.err(record.name.as_ref().unwrap(), "duplicate nested type name");
            }
        }
        let mut value_names: HashSet<_> = fields
            .iter()
            .map(|field| field.name.unraw_to_string())
            .collect();
        for constant in &body.constants {
            if !value_names.insert(constant.name.unraw_to_string()) {
                return self.err(&constant.name, "duplicate record member name");
            }
            // Record constants share their record's architecture.
            if let Some(attr) = constant
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("arch"))
            {
                return self.err(attr, "record constants inherit the record's architecture");
            }
        }
        let extent_names: Vec<_> = fields
            .iter()
            .map(|field| field.name.unraw_to_string())
            .collect();
        let extent_names: Vec<_> = extent_names.iter().map(String::as_str).collect();
        let extent_types = if fields.iter().any(|field| {
            field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("readable") || attr.path().is_ident("writable"))
        }) {
            fields
                .iter()
                .map(|field| match &field.ty {
                    FieldType::Type(ty) => self.encode_type(ty),
                    FieldType::Nested(_) => Ok(metadata::Type::Void),
                })
                .collect::<Result<Vec<_>, _>>()?
        } else {
            vec![]
        };
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
                    let child_name = format!("{name}_{index}");
                    if !nested_names.insert(child_name.clone()) {
                        return self.err(
                            &field.name,
                            "inline nested type name collides with a declaration",
                        );
                    }
                    let mt = metadata::Type::value_named(
                        self.namespace,
                        &format!("{full_name}/{child_name}"),
                    );
                    let index = index.try_into().map_err(|_| {
                        self.error(
                            &field.name,
                            "anonymous field position exceeds metadata limits",
                        )
                    })?;
                    deferred.push((child_name, rec, Some(index)));
                    mt
                }
            };
            let field_id = self
                .output
                .Field(&field_name, &mt, metadata::FieldAttributes::Public);
            if is_union {
                self.output.FieldLayout(field_id, 0);
            }
            self.encode_buffer_extents(
                metadata::writer::HasAttribute::Field(field_id),
                &field.attrs,
                &extent_names,
                true,
                attribute_ref::ExtentTypes {
                    counts: &extent_types,
                    subject: &mt,
                    result: None,
                },
            )?;
            self.encode_attrs(
                metadata::writer::HasAttribute::Field(field_id),
                &field.attrs,
                &["readable", "writable", "nullable"],
            )?;

            // Anonymous padding advances the bit offset but emits no attribute.
            let mut bit_offset = 0u32;
            for member in &field.bitfields {
                let next_offset = bit_offset
                    .checked_add(member.width)
                    .ok_or_else(|| self.error(&field.name, "bitfield offset exceeds u32"))?;
                if let Some(name) = &member.name {
                    let member_type = member
                        .ty
                        .as_ref()
                        .map(|ty| {
                            self.encode_type(ty)?.type_name().ok_or_else(|| {
                                self.error(ty, "expected a primitive or nongeneric bitfield type")
                            })
                        })
                        .transpose()?;
                    self.emit_bitfield_attribute(
                        metadata::writer::HasAttribute::Field(field_id),
                        name,
                        bit_offset,
                        member.width,
                        member_type,
                    );
                }
                bit_offset = next_offset;
            }
        }

        for constant in &body.constants {
            if winrt {
                return self.err(&constant.name, "record constants require Win32 metadata");
            }
            if constant.expr.is_none() {
                return self.err(&constant.name, "record constants require a value");
            }
            let ty = self.encode_type(&constant.ty)?;
            self.encode_const_value(&ty, constant, &constant.name.unraw_to_string())?;
        }
        let enclosing = Enclosing {
            outer: type_def,
            arch: effective_arch,
            name: &full_name,
            anonymous: None,
        };
        for (child_name, rec, anonymous) in deferred {
            self.encode_record(
                &child_name,
                winrt,
                rec.is_union,
                &rec.body,
                &rec.attrs,
                Some(Enclosing {
                    anonymous,
                    ..enclosing
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
            &["packed", "align", "anonymous"],
        )
    }
}
