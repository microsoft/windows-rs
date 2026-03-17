use super::*;

#[derive(Debug, Clone)]
pub struct Struct {
    pub attrs: Vec<syn::Attribute>,
    pub span: proc_macro2::Span,
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

impl Struct {
    pub fn encode(&self, encoder: &mut Encoder) -> Result<(), Error> {
        let mut breadcrumbs = vec![];
        encode_body(
            encoder,
            &self.name.to_string(),
            None,
            self.winrt,
            false,
            &self.attrs,
            &self.fields,
            &mut breadcrumbs,
        )
    }
}

/// Encode a struct or union type body into the metadata output.
///
/// This is shared between top-level `Struct`/`Union` encoding and the recursive
/// encoding of inline nested type definitions.
pub fn encode_body(
    encoder: &mut Encoder,
    item_name: &str,
    outer: Option<metadata::writer::TypeDef>,
    winrt: bool,
    is_union: bool,
    attrs: &[syn::Attribute],
    fields: &[Field],
    breadcrumbs: &mut Vec<String>,
) -> Result<(), Error> {
    breadcrumbs.push(item_name.to_string());

    let type_def = define_type(encoder, item_name, outer, winrt, is_union);

    // Register this type as nested before processing children so that the
    // NestedClass table stays sorted by nested-type row index.
    if let Some(outer) = outer {
        encoder.output.NestedClass(type_def, outer);
    }

    encode_fields(encoder, fields, type_def, is_union, breadcrumbs)?;

    if outer.is_none() {
        encode_attrs(
            encoder,
            metadata::writer::HasAttribute::TypeDef(type_def),
            attrs,
            &[],
        )?;
    }

    breadcrumbs.pop();

    Ok(())
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

/// Emit field entries and recursively encode any inline nested type definitions.
fn encode_fields(
    encoder: &mut Encoder,
    fields: &[Field],
    parent: metadata::writer::TypeDef,
    is_union: bool,
    breadcrumbs: &mut Vec<String>,
) -> Result<(), Error> {
    // First pass: emit all field metadata entries, assigning path-based names to
    // inline struct/union types so they can be referenced before being defined.
    let mut inline_counter = 0;
    for field in fields {
        let field_name = field.name.to_string();
        match &field.ty {
            FieldTy::Type(ty) => {
                let mt = encode_type(encoder, ty)?;
                let field_id =
                    encoder
                        .output
                        .Field(&field_name, &mt, metadata::FieldAttributes::Public);
                if is_union {
                    encoder.output.FieldLayout(field_id, 0);
                }
                encode_attrs(
                    encoder,
                    metadata::writer::HasAttribute::Field(field_id),
                    &field.attrs,
                    &[],
                )?;
            }
            FieldTy::Struct(_) | FieldTy::Union(_) => {
                let mt = inline_type(encoder, breadcrumbs, inline_counter);
                inline_counter += 1;
                let field_id =
                    encoder
                        .output
                        .Field(&field_name, &mt, metadata::FieldAttributes::Public);
                if is_union {
                    encoder.output.FieldLayout(field_id, 0);
                }
                encode_attrs(
                    encoder,
                    metadata::writer::HasAttribute::Field(field_id),
                    &field.attrs,
                    &[],
                )?;
            }
            FieldTy::StructArray(_, len) | FieldTy::UnionArray(_, len) => {
                let element = inline_type(encoder, breadcrumbs, inline_counter);
                inline_counter += 1;
                let mt = metadata::Type::ArrayFixed(Box::new(element), *len);
                let field_id =
                    encoder
                        .output
                        .Field(&field_name, &mt, metadata::FieldAttributes::Public);
                if is_union {
                    encoder.output.FieldLayout(field_id, 0);
                }
                encode_attrs(
                    encoder,
                    metadata::writer::HasAttribute::Field(field_id),
                    &field.attrs,
                    &[],
                )?;
            }
        }
    }

    // Second pass: recursively encode the bodies of any inline nested types.
    let mut inline_counter = 0;
    for field in fields {
        match &field.ty {
            FieldTy::Struct(inline_fields) => {
                let name = inline_name(breadcrumbs, inline_counter);
                inline_counter += 1;
                encode_body(
                    encoder,
                    &name,
                    Some(parent),
                    false,
                    false,
                    &[],
                    inline_fields,
                    breadcrumbs,
                )?;
            }
            FieldTy::Union(inline_fields) => {
                let name = inline_name(breadcrumbs, inline_counter);
                inline_counter += 1;
                encode_body(
                    encoder,
                    &name,
                    Some(parent),
                    false,
                    true,
                    &[],
                    inline_fields,
                    breadcrumbs,
                )?;
            }
            FieldTy::StructArray(inline_fields, _) => {
                let name = inline_name(breadcrumbs, inline_counter);
                inline_counter += 1;
                encode_body(
                    encoder,
                    &name,
                    Some(parent),
                    false,
                    false,
                    &[],
                    inline_fields,
                    breadcrumbs,
                )?;
            }
            FieldTy::UnionArray(inline_fields, _) => {
                let name = inline_name(breadcrumbs, inline_counter);
                inline_counter += 1;
                encode_body(
                    encoder,
                    &name,
                    Some(parent),
                    false,
                    true,
                    &[],
                    inline_fields,
                    breadcrumbs,
                )?;
            }
            FieldTy::Type(_) => {}
        }
    }

    Ok(())
}

/// Compute the `metadata::Type` reference for an inline nested type at `counter`.
fn inline_type(encoder: &Encoder, breadcrumbs: &[String], counter: usize) -> metadata::Type {
    let parent = breadcrumbs.last().unwrap();
    let path = format!("{}/{}_{}", breadcrumbs.join("/"), parent, counter);
    metadata::Type::named(encoder.namespace, &path)
}

/// Compute the short name (last segment) for an inline nested type at `counter`.
fn inline_name(breadcrumbs: &[String], counter: usize) -> String {
    let parent = breadcrumbs.last().unwrap();
    format!("{}_{}", parent, counter)
}

fn last_segment(path: &str) -> &str {
    path.rsplit('/').next().unwrap_or(path)
}
