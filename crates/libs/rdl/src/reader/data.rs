use super::*;

#[derive(Debug)]
pub struct Data {
    pub attrs: Vec<syn::Attribute>,
    pub token: syn::Token![static],
    pub name: syn::Ident,
    pub ty: syn::Type,
    pub winrt: bool,
}

impl syn::parse::Parse for Data {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let token = input.parse()?;
        let name = input.parse()?;
        input.parse::<syn::Token![:]>()?;
        let ty = input.parse()?;
        input.parse::<syn::Token![;]>()?;
        Ok(Self {
            attrs,
            token,
            name,
            ty,
            winrt: false,
        })
    }
}

struct Import {
    dll: syn::LitStr,
    name: Option<syn::LitStr>,
    ordinal: Option<syn::LitInt>,
    kind: syn::Ident,
}

impl syn::parse::Parse for Import {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let dll = input.parse()?;
        let mut name = None;
        let mut ordinal = None;
        let mut kind = None;
        while !input.is_empty() {
            input.parse::<syn::Token![,]>()?;
            if input.is_empty() {
                break;
            }
            let option: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;
            match option.to_string().as_str() {
                "name" if name.is_none() => name = Some(input.parse()?),
                "ordinal" if ordinal.is_none() => ordinal = Some(input.parse()?),
                "kind" if kind.is_none() => kind = Some(input.parse()?),
                _ => {
                    return Err(syn::Error::new(
                        option.span(),
                        "unknown or duplicate import option",
                    ));
                }
            }
        }
        if name.is_some() == ordinal.is_some() {
            return Err(input.error("data import requires exactly one export name or ordinal"));
        }
        let kind = kind.ok_or_else(|| input.error("data import kind is required"))?;
        Ok(Self {
            dll,
            name,
            ordinal,
            kind,
        })
    }
}

impl Encoder<'_> {
    pub fn encode_data(&mut self, item: &Data) -> Result<(), Error> {
        if item.winrt {
            return self.err(item.token, "data imports are not valid in WinRT namespaces");
        }
        let mut imports = item
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("import"));
        let Some(attr) = imports.next() else {
            return self.err(&item.name, "data declaration requires an import attribute");
        };
        if imports.next().is_some() {
            return self.err(attr, "duplicate data import attribute");
        }
        let import: Import = attr
            .parse_args()
            .map_err(|error| self.error(attr, &error.to_string()))?;
        let kind = match import.kind.to_string().as_str() {
            "data" => 1,
            "constant" => 2,
            _ => return self.err(&import.kind, "data import kind must be data or constant"),
        };
        let ordinal = if let Some(ordinal) = &import.ordinal {
            i32::from(
                ordinal
                    .base10_parse::<u16>()
                    .map_err(|_| self.error(ordinal, "data import ordinal must fit u16"))?,
            )
        } else {
            -1
        };
        if import
            .name
            .as_ref()
            .is_some_and(|name| name.value().is_empty())
        {
            return self.err(attr, "data import export name must not be empty");
        }
        let name = import
            .name
            .as_ref()
            .map_or_else(String::new, syn::LitStr::value);
        let ty = self.encode_type(&item.ty)?;
        let field = self.output.Field(
            &item.name.unraw_to_string(),
            &ty,
            metadata::FieldAttributes::Public | metadata::FieldAttributes::Static,
        );
        let target = metadata::writer::HasAttribute::Field(field);
        self.encode_named_attribute(
            target,
            &attribute_ref::AttributeRef {
                type_name: metadata::TypeName::named(
                    METADATA_NAMESPACE,
                    "NativeDataImportAttribute",
                ),
                args: vec![
                    (String::new(), metadata::Value::Utf8(import.dll.value())),
                    (String::new(), metadata::Value::Utf8(name)),
                    (String::new(), metadata::Value::I32(ordinal)),
                    (String::new(), metadata::Value::I32(kind)),
                ],
            },
        );
        if let Some(arches) = self.read_arch(&item.attrs)? {
            self.emit_arch_attribute(target, arches);
        }
        self.encode_attrs(target, &item.attrs, &["import"])
    }
}
