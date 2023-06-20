mod to_idl;
mod to_winmd;
mod writer;
use crate::Result;
use syn::spanned::Spanned;
pub use to_idl::from_reader;

impl File {
    pub fn parse_str(input: &str) -> Result<Self> {
        Ok(syn::parse_str::<Self>(input)?)
    }

    // Note: this isn't called automatically by `parse_str` to avoid canonicalizing when we're merely formatting IDL.
    pub fn canonicalize(&mut self) -> Result<()> {
        // TODO maybe we rewrite the `File` here to resolve any `super` references and use declarations so that
        // subsequently the idl-to-winmd conversion can just assume everything's fully qualified?
        // * super can't refer to something outside of the IDL file
        // * use declarations are only used for unqualified names that aren't defined in the IDL file
        // * use declarations don't support globs and must name all externally defined types
        // This way we can quickly kick out common invalid IDL files before we lost file/span context info

        Ok(())
    }

    pub fn fmt(&self) -> String {
        writer::Writer::new(self).into_string()
    }

    pub fn into_winmd(mut self) -> Result<Vec<u8>> {
        self.canonicalize()?;
        to_winmd::idl_to_winmd(&self)
    }
}

// TODO: always set the winrt bit on the assembly but only set the winrt bit on the TypeDef if its a WinRT type.
// Also, use a file-level attribute in the IDL file to indicate whether it contains WinRT or Win32 types
//  e.g. #![win32|winrt] - with default being winrt - that way Win32 and WinRT types could conceivably share a
// namespace but live in separate IDL files to simplify the IDL syntax.

// The value of the IDL-specific memory representation is that it allows for constructs that are not modeled in the abstract Module
// tree such as the use declarations and if we get rid of it we'd always "format" IDL by stripping out any of that into a single
// canonical form which would not be very friendly to developers.
pub struct File {
    pub references: Vec<syn::ItemUse>,
    pub modules: Vec<Module>,
}

// TODO: need to change these to unpack the syn types and store strings we can reference for efficiency along with spans since the syn
// is made for value semantics.

#[derive(Clone)]
pub struct Module {
    pub attributes: Vec<syn::Attribute>, // winrt/win32
    pub name: String,
    pub members: Vec<ModuleMember>,
}

#[derive(Clone)]
pub enum ModuleMember {
    Module(Module),
    Interface(Interface),
    Struct(Struct),
    Enum(Enum),
    Class(Class),
    // Function and Delegate
}

impl ModuleMember {
    pub fn name(&self) -> &str {
        match self {
            Self::Module(module) => &module.name,
            Self::Interface(member) => &member.name,
            Self::Struct(member) => &member.name,
            Self::Enum(member) => &member.name,
            Self::Class(member) => &member.name,
        }
    }
}

#[derive(Clone)]
pub struct Enum {
    pub name: String,
    pub item: syn::ItemEnum,
}

#[derive(Clone)]
pub struct Struct {
    pub name: String,
    pub attributes: Vec<syn::Attribute>,
    pub span: proc_macro2::Span,
    pub fields: Vec<Field>,
}

#[derive(Clone)]
pub struct Field {
    pub name: String,
    pub attributes: Vec<syn::Attribute>,
    pub span: proc_macro2::Span,
    pub ty: syn::Type,
}

#[derive(Clone)]
pub struct Class {
    pub name: String,
    pub attributes: Vec<syn::Attribute>,
    pub extends: Vec<syn::Path>,
}

#[derive(Clone)]
pub struct Interface {
    pub name: String,
    pub attributes: Vec<syn::Attribute>,
    pub methods: Vec<syn::TraitItemFn>,
}

syn::custom_keyword!(interface);
syn::custom_keyword!(class);

impl syn::parse::Parse for File {
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
        Ok(Self {
            references,
            modules,
        })
    }
}

impl syn::parse::Parse for Module {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![mod]>()?;
        let name = input.parse::<syn::Ident>()?.to_string();
        let content;
        syn::braced!(content in input);
        let mut members = vec![];
        while !content.is_empty() {
            members.push(content.parse::<ModuleMember>()?);
        }
        Ok(Self {
            attributes: vec![],
            name,
            members,
        })
    }
}

impl syn::parse::Parse for ModuleMember {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attributes: Vec<syn::Attribute> = input.call(syn::Attribute::parse_outer)?;
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![mod]) {
            if let Some(attribute) = attributes.first() {
                return Err(syn::Error::new(
                    attribute.span(),
                    "module attributes not supported",
                ));
            }
            Ok(ModuleMember::Module(input.parse()?))
        } else if lookahead.peek(interface) {
            Ok(ModuleMember::Interface(Interface::parse(
                attributes, input,
            )?))
        } else if lookahead.peek(syn::Token![struct]) {
            Ok(ModuleMember::Struct(Struct::parse(attributes, input)?))
        } else if lookahead.peek(syn::Token![enum]) {
            Ok(ModuleMember::Enum(Enum::parse(attributes, input)?))
        } else if lookahead.peek(class) {
            Ok(ModuleMember::Class(Class::parse(attributes, input)?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Class {
    fn parse(attributes: Vec<syn::Attribute>, input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<class>()?;
        let name = input.parse::<syn::Ident>()?.to_string();
        let mut extends = Vec::new();

        if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;
            while input.peek(syn::Ident) {
                extends.push(input.parse::<syn::Path>()?);
                _ = input.parse::<syn::Token![,]>();
            }
        }

        input.parse::<syn::Token![;]>()?;
        Ok(Self {
            attributes,
            name,
            extends,
        })
    }
}

impl Interface {
    fn parse(attributes: Vec<syn::Attribute>, input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<interface>()?;
        let name = input.parse::<syn::Ident>()?.to_string();
        let content;
        syn::braced!(content in input);
        let mut methods = vec![];
        while !content.is_empty() {
            methods.push(content.parse::<syn::TraitItemFn>()?);
        }
        Ok(Self {
            attributes,
            name,
            methods,
        })
    }
}

impl Struct {
    fn parse(attributes: Vec<syn::Attribute>, input: syn::parse::ParseStream) -> syn::Result<Self> {
        // TODO: need to validate that the struct is valid according to the constraints of the winmd type system.
        // Same for the other types. That way we can spit out errors quickly for things like unnamed fields.
        let span = input.span();
        let item: syn::ItemStruct = input.parse()?;
        let name = item.ident.to_string();
        let mut fields = vec![];

        let syn::Fields::Named(named) = item.fields else {
            return Err(syn::Error::new(
                item.span(),
                "unnamed fields not supported",
            ));
        };

        for field in named.named {
            fields.push(Field {
                span: field.span(),
                attributes: field.attrs,
                // Simply unwrapping since we already know that it is a named field.
                name: field.ident.unwrap().to_string(),
                ty: field.ty,
            });
        }

        Ok(Self {
            name,
            attributes,
            span,
            fields,
        })
    }
}

impl Enum {
    fn parse(attributes: Vec<syn::Attribute>, input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut item: syn::ItemEnum = input.parse()?;
        item.attrs = attributes;
        let name = item.ident.to_string();
        Ok(Self { name, item })
    }
}
