use super::*;

#[derive(Debug)]
pub enum Item {
    // WinRT/Win32 types
    // Attribute(ItemAttribute)
    Class(Class),
    // Delegate(ItemDelegate),
    Enum(Enum),
    Interface(Interface),
    Struct(Struct),
    Union(Union),
    // Win32 functions and constants
    Fn(Fn),
    Const(Const),

    // Nested namespace
    Module(Module),
    // For convenience but not expressed in metadata
    // Use(ItemUse),
    // Type(ItemType),
}

impl Item {
    fn replace_attrs(&mut self, new: Vec<syn::Attribute>) -> Vec<syn::Attribute> {
        match self {
            Self::Enum(Enum { attrs, .. })
            | Self::Fn(Fn { attrs, .. })
            | Self::Const(Const { attrs, .. })
            | Self::Class(Class { attrs, .. })
            | Self::Interface(Interface { attrs, .. })
            | Self::Module(Module { attrs, .. })
            | Self::Struct(Struct { attrs, .. })
            | Self::Union(Union { attrs, .. }) => std::mem::replace(attrs, new),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Enum(item) => item.name.fmt(f),
            Self::Fn(item) => item.sig.ident.fmt(f),
            Self::Const(item) => item.name.fmt(f),
            Self::Class(item) => item.name.fmt(f),
            Self::Interface(item) => item.name.fmt(f),
            Self::Struct(item) => match &item.name {
                Some(name) => name.fmt(f),
                None => write!(f, "<unnamed struct>"),
            },
            Self::Module(item) => item.name.fmt(f),
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
        } else if lookahead.peek(syn::Token![union]) {
            input.parse().map(Item::Union)
        } else if lookahead.peek(syn::Token![fn]) {
            input.parse().map(Item::Fn)
        } else if lookahead.peek(syn::Token![const]) {
            input.parse().map(Item::Const)
        } else if lookahead.peek(class) {
            input.parse().map(Item::Class)
        } else {
            Err(lookahead.error())
        }?;

        item.replace_attrs(attrs);
        Ok(item)
    }
}
