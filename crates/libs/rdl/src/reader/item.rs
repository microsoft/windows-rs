use super::*;

#[derive(Debug)]
pub enum Item {
    Attribute(Attribute),
    Callback(Callback),
    Class(Class),
    Const(Const),
    Delegate(Delegate),
    Enum(Enum),
    Fn(Fn),
    Interface(Interface),
    Module(Module),
    Struct(Struct),
    Union(Union),
}

impl Item {
    fn replace_attrs(&mut self, new: Vec<syn::Attribute>) -> Vec<syn::Attribute> {
        match self {
            Self::Attribute(Attribute { attrs, .. })
            | Self::Callback(Callback { attrs, .. })
            | Self::Class(Class { attrs, .. })
            | Self::Const(Const { attrs, .. })
            | Self::Delegate(Delegate { attrs, .. })
            | Self::Enum(Enum { attrs, .. })
            | Self::Fn(Fn { attrs, .. })
            | Self::Interface(Interface { attrs, .. })
            | Self::Module(Module { attrs, .. })
            | Self::Struct(Struct { attrs, .. })
            | Self::Union(Union { attrs, .. }) => std::mem::replace(attrs, new),
        }
    }

    pub fn encode(
        &self,
        output: &mut metadata::writer::File,
        index: &Index,
        reference: &metadata::reader::TypeIndex,
        file: &File,
        namespace: &str,
        name: &str,
    ) -> Result<(), Error> {
        let encoder = &mut Encoder {
            output,
            index,
            reference,
            file,
            namespace,
            name,
            generics: vec![],
        };

        match self {
            Self::Attribute(ty) => ty.encode(encoder),
            Self::Callback(ty) => ty.encode(encoder),
            Self::Class(ty) => ty.encode(encoder),
            Self::Const(ty) => ty.encode(encoder),
            Self::Delegate(ty) => ty.encode(encoder),
            Self::Enum(ty) => ty.encode(encoder),
            Self::Fn(ty) => ty.encode(encoder),
            Self::Interface(ty) => ty.encode(encoder),
            Self::Struct(ty) => ty.encode(encoder),
            Self::Union(ty) => ty.encode(encoder),
            // Module items are expanded into their children during indexing and are
            // never placed in the index themselves, so this arm is unreachable.
            Self::Module(_) => unreachable!(
                "Module items cannot be encoded directly; they are expanded during indexing"
            ),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Attribute(item) => item.name.fmt(f),
            Self::Callback(item) => item.sig.ident.fmt(f),
            Self::Class(item) => item.name.fmt(f),
            Self::Const(item) => item.name.fmt(f),
            Self::Delegate(item) => item.sig.ident.fmt(f),
            Self::Enum(item) => item.name.fmt(f),
            Self::Fn(item) => item.sig.ident.fmt(f),
            Self::Interface(item) => item.name.fmt(f),
            Self::Module(item) => item.name.fmt(f),
            Self::Struct(item) => item
                .name
                .as_ref()
                .expect("top-level structs must be named")
                .fmt(f),
            Self::Union(item) => item.name.fmt(f),
        }
    }
}

impl syn::parse::Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let lookahead = input.lookahead1();

        let mut item = if lookahead.peek(syn::Token![struct]) {
            input.parse().map(Item::Struct)
        } else if lookahead.peek(syn::Token![enum]) {
            input.parse().map(Item::Enum)
        } else if lookahead.peek(syn::Token![mod]) {
            input.parse().map(Item::Module)
        } else if lookahead.peek(interface) {
            input.parse().map(Item::Interface)
        } else if lookahead.peek(attribute) {
            input.parse().map(Item::Attribute)
        } else if lookahead.peek(syn::Token![union]) {
            input.parse().map(Item::Union)
        } else if lookahead.peek(syn::Token![extern]) {
            if attrs
                .iter()
                .find(|a| a.path().is_ident("library"))
                .is_some()
            {
                input.parse().map(Item::Fn)
            } else {
                input.parse().map(Item::Callback)
            }
        } else if lookahead.peek(syn::Token![const]) {
            input.parse().map(Item::Const)
        } else if lookahead.peek(delegate) {
            input.parse().map(Item::Delegate)
        } else if lookahead.peek(class) {
            input.parse().map(Item::Class)
        } else {
            Err(lookahead.error())
        }?;

        item.replace_attrs(attrs);
        Ok(item)
    }
}
