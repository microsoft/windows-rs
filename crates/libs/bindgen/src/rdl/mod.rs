use super::*;
mod fmt;
mod from_reader;
mod to_winmd;
use crate::Result;
pub use from_reader::from_reader;
use syn::spanned::Spanned;

// TODO: may want to finally get rid of `syn` as it also doesn't support preserving code comments

impl File {
    pub fn parse_str(input: &str) -> Result<Self> {
        Ok(syn::parse_str::<Self>(input)?)
    }

    // Note: this isn't called automatically by `parse_str` to avoid canonicalizing when we're merely formatting IDL.
    pub fn canonicalize(&mut self) -> Result<()> {
        // TODO maybe we rewrite the `File` here to resolve any `super` references and use declarations so that
        // subsequently the rdl-to-winmd conversion can just assume everything's fully qualified?
        // * super can't refer to something outside of the IDL file
        // * use declarations are only used for unqualified names that aren't defined in the IDL file
        // * use declarations don't support globs and must name all externally defined types
        // This way we can quickly kick out common invalid IDL files before we lost file/span context info

        Ok(())
    }

    pub fn fmt(&self) -> String {
        fmt::Writer::new(self).into_string()
    }

    pub fn into_winmd(mut self) -> Result<Vec<u8>> {
        self.canonicalize()?;
        to_winmd::rdl_to_winmd(&self)
    }
}

// The value of the IDL-specific memory representation is that it allows for constructs that are not modeled in the abstract Module
// tree such as the use declarations and if we get rid of it we'd always "format" IDL by stripping out any of that into a single
// canonical form which would not be very friendly to developers.
#[derive(Debug)]
pub struct File {
    pub winrt: bool,
    pub references: Vec<syn::ItemUse>,
    pub modules: Vec<Module>,
}

// TODO: need to change these to unpack the syn types and store strings we can reference for efficiency along with spans since the syn
// is made for value semantics.

#[derive(Clone, Debug)]
pub struct Module {
    pub namespace: String,
    pub members: Vec<ModuleMember>,
}

#[derive(Clone, Debug)]
pub enum ModuleMember {
    Module(Module),
    Interface(Interface),
    Struct(Struct),
    Enum(Enum),
    Class(Class),
    Function(Function),
    Constant(Constant),
}

impl ModuleMember {
    pub fn name(&self) -> &str {
        match self {
            Self::Module(module) => crate::extension(&module.namespace),
            Self::Interface(member) => &member.name,
            Self::Struct(member) => &member.name,
            Self::Enum(member) => &member.name,
            Self::Class(member) => &member.name,
            Self::Function(member) => &member.name,
            Self::Constant(member) => &member.name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Enum {
    pub winrt: bool,
    pub name: String,
    pub item: syn::ItemEnum,
}

#[derive(Clone, Debug)]
pub struct Constant {
    pub name: String,
    pub item: syn::ItemConst,
}

#[derive(Clone, Debug)]
pub struct Struct {
    pub winrt: bool,
    pub name: String,
    pub attributes: Vec<syn::Attribute>,
    pub span: proc_macro2::Span,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub attributes: Vec<syn::Attribute>,
    pub span: proc_macro2::Span,
    pub ty: syn::Type,
}

#[derive(Clone, Debug)]
pub struct Class {
    pub name: String,
    pub attributes: Vec<syn::Attribute>,
    pub base: Option<syn::TypePath>,
    pub extends: Vec<syn::TypePath>,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub item: syn::TraitItemFn,
}

#[derive(Clone, Debug)]
pub struct Interface {
    pub winrt: bool,
    pub name: String,
    pub generics: Vec<String>,
    pub attributes: Vec<syn::Attribute>,
    pub extends: Vec<syn::TypePath>,
    pub methods: Vec<syn::TraitItemFn>,
}

syn::custom_keyword!(interface);
syn::custom_keyword!(class);

fn winrt(input: syn::parse::ParseStream) -> syn::Result<bool> {
    let attributes = input.call(syn::Attribute::parse_inner)?;
    if attributes.len() == 1 {
        if let syn::Meta::Path(path) = &attributes[0].meta {
            if path.is_ident("winrt") {
                return Ok(true);
            }

            if path.is_ident("win32") {
                return Ok(false);
            }
        }
    }

    Err(syn::Error::new(input.span(), "A single `#![win32]` or `#![winrt]` attribute required"))
}

impl syn::parse::Parse for File {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut references = vec![];
        let mut modules = vec![];
        let winrt = winrt(input)?;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Token![mod]) {
                modules.push(Module::parse("", winrt, input)?);
            } else if lookahead.peek(syn::Token![use]) {
                references.push(input.parse()?);
            } else {
                return Err(lookahead.error());
            }
        }
        Ok(Self { winrt, references, modules })
    }
}

impl Module {
    fn name(&self) -> &str {
        self.namespace.rsplit_once('.').map_or(&self.namespace, |(_, name)| name)
    }

    fn parse(namespace: &str, winrt: bool, input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<syn::Token![mod]>()?;
        let name = input.parse::<syn::Ident>()?.to_string();

        let namespace = if namespace.is_empty() { name.to_string() } else { format!("{namespace}.{name}") };

        let content;
        syn::braced!(content in input);
        let mut members = vec![];
        while !content.is_empty() {
            members.push(ModuleMember::parse(&namespace, winrt, &content)?);
        }
        Ok(Self { namespace, members })
    }
}

impl ModuleMember {
    fn parse(namespace: &str, winrt: bool, input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attributes: Vec<syn::Attribute> = input.call(syn::Attribute::parse_outer)?;
        let lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![mod]) {
            if let Some(attribute) = attributes.first() {
                return Err(syn::Error::new(attribute.span(), "`use` attributes not supported"));
            }
            Ok(ModuleMember::Module(Module::parse(namespace, winrt, input)?))
        } else if lookahead.peek(interface) {
            Ok(ModuleMember::Interface(Interface::parse(namespace, winrt, attributes, input)?))
        } else if lookahead.peek(syn::Token![struct]) {
            Ok(ModuleMember::Struct(Struct::parse(namespace, winrt, attributes, input)?))
        } else if lookahead.peek(syn::Token![enum]) {
            Ok(ModuleMember::Enum(Enum::parse(namespace, winrt, attributes, input)?))
        } else if lookahead.peek(class) {
            Ok(ModuleMember::Class(Class::parse(attributes, input)?))
        } else if lookahead.peek(syn::Token![fn]) {
            Ok(ModuleMember::Function(Function::parse(namespace, attributes, input)?))
        } else if lookahead.peek(syn::Token![const]) {
            Ok(ModuleMember::Constant(Constant::parse(namespace, attributes, input)?))
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
        let mut base = None;

        if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;
            while !input.peek(syn::Token![;]) {
                if input.peek(class) {
                    input.parse::<class>()?;
                    base = Some(input.parse()?);
                } else {
                    extends.push(input.parse()?);
                }
                _ = input.parse::<syn::Token![,]>();
            }
        }

        input.parse::<syn::Token![;]>()?;
        Ok(Self { attributes, name, base, extends })
    }
}

impl Interface {
    fn parse(_namespace: &str, winrt: bool, attributes: Vec<syn::Attribute>, input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<interface>()?;
        let name = input.parse::<syn::Ident>()?.to_string();

        let mut generics = Vec::new();

        if input.peek(syn::Token![<]) {
            input.parse::<syn::Token![<]>()?;
            while input.peek(syn::Ident) {
                generics.push(input.parse::<syn::Ident>()?.to_string());
                _ = input.parse::<syn::Token![,]>();
            }

            input.parse::<syn::Token![>]>()?;
        }

        let mut extends = Vec::new();

        if input.peek(syn::Token![:]) {
            input.parse::<syn::Token![:]>()?;
            while !input.peek(syn::token::Brace) {
                extends.push(input.parse()?);
                _ = input.parse::<syn::Token![,]>();
            }
        }

        let content;
        syn::braced!(content in input);
        let mut methods = vec![];
        while !content.is_empty() {
            methods.push(content.parse()?);
        }
        Ok(Self { winrt, attributes, generics, extends, name, methods })
    }
}

impl Struct {
    fn parse(_namespace: &str, winrt: bool, attributes: Vec<syn::Attribute>, input: syn::parse::ParseStream) -> syn::Result<Self> {
        // TODO: need to validate that the struct is valid according to the constraints of the winmd type system.
        // Same for the other types. That way we can spit out errors quickly for things like unnamed fields.
        let span = input.span();
        let item: syn::ItemStruct = input.parse()?;
        let name = item.ident.to_string();
        let mut fields = vec![];

        let syn::Fields::Named(named) = item.fields else {
            return Err(syn::Error::new(item.span(), "unnamed fields not supported"));
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

        Ok(Self { winrt, name, attributes, span, fields })
    }
}

impl Enum {
    fn parse(_namespace: &str, winrt: bool, attributes: Vec<syn::Attribute>, input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut item: syn::ItemEnum = input.parse()?;
        item.attrs = attributes;
        let name = item.ident.to_string();
        Ok(Self { winrt, name, item })
    }
}

impl Constant {
    fn parse(_namespace: &str, attributes: Vec<syn::Attribute>, input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut item: syn::ItemConst = input.parse()?;
        item.attrs = attributes;
        let name = item.ident.to_string();
        Ok(Self { name, item })
    }
}

impl Function {
    fn parse(_namespace: &str, attributes: Vec<syn::Attribute>, input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut item: syn::TraitItemFn = input.parse()?;
        item.attrs = attributes;
        let name = item.sig.ident.to_string();
        Ok(Self { name, item })
    }
}
