use super::*;

#[derive(Debug, Clone)]
pub struct Struct {
    pub attrs: Vec<syn::Attribute>,
    pub span: proc_macro2::Span,
    pub name: Option<syn::Ident>,
    pub fields: Vec<StructField>,
    pub winrt: bool,
    pub is_union: bool,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum StructField {
    Regular(syn::Field),
    Nested { name: syn::Ident, def: Struct },
}

impl syn::parse::Parse for Struct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token: syn::Token![struct] = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);

        let fields = content
            .parse_terminated(StructField::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            span: token.span,
            name,
            fields,
            winrt: false,
            is_union: false,
        })
    }
}

impl syn::parse::Parse for StructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let name = input.parse()?;
        input.parse::<syn::Token![:]>()?;

        if input.peek(syn::Token![struct]) {
            Ok(StructField::Nested {
                name,
                def: input.parse()?,
            })
        } else if input.peek(syn::Token![union]) {
            let union_token: syn::Token![union] = input.parse()?;
            let nested_name = input.parse()?;

            let content;
            syn::braced!(content in input);

            let fields = content
                .parse_terminated(StructField::parse, syn::Token![,])?
                .into_iter()
                .collect();

            Ok(StructField::Nested {
                name,
                def: Struct {
                    attrs: vec![],
                    span: union_token.span,
                    name: nested_name,
                    fields,
                    winrt: false,
                    is_union: true,
                },
            })
        } else {
            Ok(StructField::Regular(syn::Field {
                attrs,
                ident: Some(name),
                ty: input.parse()?,
                vis: syn::Visibility::Inherited,
                colon_token: Some(Default::default()),
                mutability: syn::FieldMutability::None,
            }))
        }
    }
}

impl Struct {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let mut breadcrumbs = vec![];
        let item_name = self.name.as_ref().unwrap().to_string();
        encode_struct_inner(encoder, self, &item_name, None, &mut breadcrumbs)
    }
}

struct NestedEntry<'a> {
    full_path: String,
    def: &'a Struct,
}

fn encode_struct_inner(
    encoder: &mut Encoder,
    item: &Struct,
    item_name: &str,
    outer: Option<metadata::writer::TypeDef>,
    breadcrumbs: &mut Vec<String>,
) -> Result<(), Error> {
    breadcrumbs.push(item_name.to_string());
    let nested = collect_nested(item, breadcrumbs);
    let type_def = define_type(encoder, item_name, outer, item.winrt, item.is_union);

    emit_fields(encoder, item, &nested, item.is_union)?;

    if outer.is_none() {
        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::TypeDef(type_def),
            &item.attrs,
            &[],
        )?;
    }

    if let Some(outer) = outer {
        encoder.output.NestedClass(type_def, outer);
    }

    encode_children(encoder, &nested, type_def, breadcrumbs)?;
    breadcrumbs.pop();

    Ok(())
}

fn collect_nested<'a>(
    item: &'a Struct,
    breadcrumbs: &[String],
) -> BTreeMap<String, NestedEntry<'a>> {
    let parent_name = breadcrumbs.last().unwrap();
    let struct_path = breadcrumbs.join("/");
    item.fields
        .iter()
        .filter_map(|field| match field {
            StructField::Nested { name, def, .. } => Some((name, def)),
            _ => None,
        })
        .enumerate()
        .map(|(i, (name, def))| {
            let full_path = format!("{}/{}_{}", struct_path, parent_name, i);
            (name.to_string(), NestedEntry { full_path, def })
        })
        .collect()
}

fn define_type(
    encoder: &mut Encoder,
    item_name: &str,
    outer: Option<metadata::writer::TypeDef>,
    winrt: bool,
    is_union: bool,
) -> metadata::writer::TypeDef {
    let value_type = encoder.output.TypeRef("System", "ValueType");

    let layout_flag = if is_union {
        metadata::TypeAttributes::ExplicitLayout
    } else {
        metadata::TypeAttributes::SequentialLayout
    };

    let mut flags = layout_flag | metadata::TypeAttributes::Sealed;

    if outer.is_some() {
        flags |= metadata::TypeAttributes::NestedPublic;
    } else {
        flags |= metadata::TypeAttributes::Public;
    };

    if winrt {
        flags |= metadata::TypeAttributes::WindowsRuntime;
    }

    encoder.output.TypeDef(
        if outer.is_some() {
            ""
        } else {
            encoder.namespace
        },
        last_segment(item_name),
        metadata::writer::TypeDefOrRef::TypeRef(value_type),
        flags,
    )
}

fn encode_children(
    encoder: &mut Encoder,
    nested: &BTreeMap<String, NestedEntry>,
    outer: metadata::writer::TypeDef,
    breadcrumbs: &mut Vec<String>,
) -> Result<(), Error> {
    for entry in nested.values() {
        let nested_name = last_segment(&entry.full_path);
        encode_struct_inner(encoder, entry.def, nested_name, Some(outer), breadcrumbs)?;
    }
    Ok(())
}

fn last_segment(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}

fn emit_fields(
    encoder: &mut Encoder,
    item: &Struct,
    nested: &BTreeMap<String, NestedEntry>,
    is_union: bool,
) -> Result<(), Error> {
    for field in &item.fields {
        match field {
            StructField::Regular(regular) => {
                let field_name = regular.ident.as_ref().unwrap().to_string();
                let field_type = encode_type(encoder, &regular.ty)?;
                let field_id = encoder.output.Field(
                    &field_name,
                    &field_type,
                    metadata::FieldAttributes::Public,
                );
                if is_union {
                    encoder.output.FieldLayout(field_id, 0);
                }
                encode_attrs(
                    encoder,
                    metadata::writer::HasAttribute::Field(field_id),
                    &regular.attrs,
                    &[],
                )?;
            }
            StructField::Nested { name, .. } => {
                let field_name = name.to_string();
                let field_type =
                    metadata::Type::named(encoder.namespace, &nested[&field_name].full_path);
                let field_id = encoder.output.Field(
                    &field_name,
                    &field_type,
                    metadata::FieldAttributes::Public,
                );
                if is_union {
                    encoder.output.FieldLayout(field_id, 0);
                }
            }
        }
    }
    Ok(())
}
