use super::*;
use syn::spanned::Spanned;

// TODO: always set the winrt bit on the assembly but only set the winrt bit on the TypeDef if its a WinRT type.
// Also, use a file-level attribute in the IDL file to indicate whether it contains WinRT or Win32 types
//  e.g. #![win32|winrt] - with default being winrt - that way Win32 and WinRT types could conceivably share a
// namespace but live in separate IDL files to simplify the IDL syntax.

syn::custom_keyword!(interface);
syn::custom_keyword!(class);

impl IdlFile {
    pub fn parse_str(source: &str) -> Result<Self> {
        Ok(syn::parse_str::<IdlFile>(source)?)
    }
}

impl syn::parse::Parse for IdlFile {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut references = vec![];
        let mut modules = vec![];
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Token![mod]) {
                modules.push(input.parse()?);
            } else if lookahead.peek(syn::Token![use]) {
                references.push(input.parse()?);
            } else {
                return Err(lookahead.error());
            }
        }
        Ok(Self { references, modules })
    }
}

impl syn::parse::Parse for IdlModule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![mod]>()?;
        let ident = input.parse::<syn::Ident>()?;
        let content;
        syn::braced!(content in input);
        let mut members = vec![];
        while !content.is_empty() {
            members.push(content.parse::<IdlModuleMember>()?);
        }
        Ok(Self { attributes: vec![], ident, members })
    }
}

impl syn::parse::Parse for IdlModuleMember {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attributes: Vec<syn::Attribute> = input.call(syn::Attribute::parse_outer)?;
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![mod]) {
            if let Some(attribute) = attributes.first() {
                return Err(syn::Error::new(attribute.span(), "module attribute are not supported"));
            }
            Ok(IdlModuleMember::Module(input.parse()?))
        } else if lookahead.peek(interface) {
            Ok(IdlModuleMember::Interface(IdlInterface::parse(attributes, input)?))
        } else if lookahead.peek(syn::Token![struct]) {
            Ok(IdlModuleMember::Struct(IdlStruct::parse(attributes, input)?))
        } else if lookahead.peek(syn::Token![enum]) {
            Ok(IdlModuleMember::Enum(IdlEnum::parse(attributes, input)?))
        } else if lookahead.peek(class) {
            Ok(IdlModuleMember::Class(IdlClass::parse(attributes, input)?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl IdlClass {
    fn parse(attributes: Vec<syn::Attribute>, input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<class>()?;
        let ident = input.parse::<syn::Ident>()?;
        let mut extends = Vec::new();

        if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;
            while input.peek(syn::Ident) {
                extends.push(input.parse::<syn::Path>()?);
                _ = input.parse::<syn::Token![,]>();
            }
        }

        input.parse::<syn::Token![;]>()?;
        Ok(Self { attributes, ident, extends })
    }
}

impl IdlInterface {
    fn parse(attributes: Vec<syn::Attribute>, input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<interface>()?;
        let ident = input.parse::<syn::Ident>()?;
        let content;
        syn::braced!(content in input);
        let mut methods = vec![];
        while !content.is_empty() {
            methods.push(content.parse::<syn::TraitItemMethod>()?);
        }
        Ok(Self { attributes, ident, methods })
    }
}

impl IdlStruct {
    fn parse(attributes: Vec<syn::Attribute>, input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut item: syn::ItemStruct = input.parse()?;
        item.attrs = attributes;
        Ok(Self { item })
    }
}

impl IdlEnum {
    fn parse(attributes: Vec<syn::Attribute>, input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut item: syn::ItemEnum = input.parse()?;
        item.attrs = attributes;
        Ok(Self { item })
    }
}
