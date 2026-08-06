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
    Typedef(Typedef),
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
            | Self::Typedef(Typedef { attrs, .. })
            | Self::Union(Union { attrs, .. }) => std::mem::replace(attrs, new),
        }
    }

    pub fn attrs(&self) -> &[syn::Attribute] {
        match self {
            Self::Attribute(item) => &item.attrs,
            Self::Callback(item) => &item.attrs,
            Self::Class(item) => &item.attrs,
            Self::Const(item) => &item.attrs,
            Self::Delegate(item) => &item.attrs,
            Self::Enum(item) => &item.attrs,
            Self::Fn(item) => &item.attrs,
            Self::Interface(item) => &item.attrs,
            Self::Module(item) => &item.attrs,
            Self::Struct(item) => &item.attrs,
            Self::Typedef(item) => &item.attrs,
            Self::Union(item) => &item.attrs,
        }
    }

    pub fn name_span(&self) -> Span {
        match self {
            Self::Attribute(item) => item.name.span(),
            Self::Callback(item) => item.sig.ident.span(),
            Self::Class(item) => item.name.span(),
            Self::Const(item) => item.name.span(),
            Self::Delegate(item) => item.sig.ident.span(),
            Self::Enum(item) => item.name.span(),
            Self::Fn(item) => item.sig.ident.span(),
            Self::Interface(item) => item.name.span(),
            Self::Module(item) => item.name.span(),
            Self::Struct(item) => item.name.span(),
            Self::Typedef(item) => item.name.span(),
            Self::Union(item) => item.name.span(),
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
            Self::Struct(item) => item.name.fmt(f),
            Self::Typedef(item) => item.name.fmt(f),
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
        } else if lookahead.peek(syn::Token![type]) {
            input.parse().map(Item::Typedef)
        } else {
            Err(lookahead.error())
        }?;

        item.replace_attrs(attrs);
        Ok(item)
    }
}
