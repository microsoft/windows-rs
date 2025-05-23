use super::*;

pub enum Item {
    // WinRT/Win32 types
    // Attribute(ItemAttribute)
    // Class(ItemClass),
    // Delegate(ItemDelegate),
    Enum(ItemEnum),
    Interface(ItemInterface),
    Struct(ItemStruct),
    // Win32 functions and constrants
    // Const(ItemConst),
    // Fn(ItemFn),

    // Nested namespace
    // Mod(ItemMod),

    // For convenience but not expressed in metadata
    // Use(ItemUse),
    // Type(ItemType),
}

impl Item {
    fn replace_attrs(&mut self, new: Vec<syn::Attribute>) -> Vec<syn::Attribute> {
        match self {
            Self::Enum(ItemEnum { attrs, .. })
            | Self::Interface(ItemInterface { attrs, .. })
            | Self::Struct(ItemStruct { attrs, .. }) => std::mem::replace(attrs, new),
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
        } else if lookahead.peek(interface) {
            input.parse().map(Item::Interface)
        } else {
            Err(lookahead.error())
        }?;

        item.replace_attrs(attrs);
        Ok(item)
    }
}
