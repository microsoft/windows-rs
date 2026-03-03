use super::*;

#[derive(Debug)]
pub struct Struct {
    pub attrs: Vec<syn::Attribute>,
    pub token: syn::Token![struct],
    pub name: Option<syn::Ident>,
    pub fields: Vec<StructField>,
    pub winrt: bool,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum StructField {
    Regular(syn::Field),
    Nested { name: syn::Ident, def: Struct },
}

impl syn::parse::Parse for Struct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;

        let content;
        syn::braced!(content in input);

        let fields = content
            .parse_terminated(StructField::parse, syn::Token![,])?
            .into_iter()
            .collect();

        Ok(Self {
            attrs,
            token,
            name,
            fields,
            winrt: false,
        })
    }
}

impl syn::parse::Parse for StructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<syn::Token![:]>()?;

        if input.peek(syn::Token![struct]) {
            Ok(StructField::Nested {
                name,
                def: input.parse()?,
            })
        } else {
            Ok(StructField::Regular(syn::Field {
                attrs: vec![],
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
        let mut depth = 0;
        encode_struct_inner(encoder, self, None, encoder.name, &mut depth)
    }
}

fn encode_struct_inner(
    encoder: &mut Encoder,
    item: &Struct,
    outer: Option<metadata::writer::TypeDef>,
    name: &str,
    depth: &mut usize,
) -> Result<(), Error> {
    let nested = nested(item, name, depth);

    let value_type = encoder.output.TypeRef("System", "ValueType");
    let mut flags = metadata::TypeAttributes::SequentialLayout | metadata::TypeAttributes::Sealed;

    flags |= if outer.is_some() {
        metadata::TypeAttributes::NestedPublic
    } else {
        metadata::TypeAttributes::Public
    };

    if item.winrt {
        flags |= metadata::TypeAttributes::WindowsRuntime;
    }

    let ty = encoder.output.TypeDef(
        encoder.namespace,
        name,
        metadata::writer::TypeDefOrRef::TypeRef(value_type),
        flags,
    );

    fields(encoder, item, &nested)?;

    if let Some(outer) = outer {
        encoder.output.NestedClass(ty, outer);
    }

    for (nested_type_name, def) in nested.values() {
        *depth = 0;
        encode_struct_inner(encoder, def, Some(ty), nested_type_name, depth)?;
    }

    Ok(())
}

fn fields(
    encoder: &mut Encoder,
    item: &Struct,
    nested: &BTreeMap<String, (String, &Struct)>,
) -> Result<(), Error> {
    for field in &item.fields {
        match field {
            StructField::Regular(f) => {
                let name = f.ident.as_ref().unwrap().to_string();
                let ty = encode_type(encoder, &f.ty)?;
                encoder
                    .output
                    .Field(&name, &ty, metadata::FieldAttributes::Public);
            }
            StructField::Nested { name, .. } => {
                let field_name = name.to_string();
                let (nested_type_name, _) = &nested[&field_name];
                let ty = metadata::Type::named(encoder.namespace, nested_type_name.as_str());
                encoder
                    .output
                    .Field(&field_name, &ty, metadata::FieldAttributes::Public);
            }
        }
    }

    Ok(())
}

fn nested<'a>(
    item: &'a Struct,
    name: &str,
    depth: &mut usize,
) -> BTreeMap<String, (String, &'a Struct)> {
    item.fields
        .iter()
        .filter_map(|field| {
            if let StructField::Nested {
                name: ident_name,
                def,
                ..
            } = field
            {
                let field_name = ident_name.to_string();
                let nested_name = format!("{}_{}", name, *depth);
                *depth += 1;
                Some((field_name, (nested_name, def)))
            } else {
                None
            }
        })
        .collect()
}
