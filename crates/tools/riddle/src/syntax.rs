use metadata::writer;
use syn::{parse::*, *};

mod keywords {
    syn::custom_keyword!(interface);
}

pub trait ToWriter {
    fn to_writer(&self, namespace: &str, items: &mut Vec<writer::Item>) -> Result<()>;
}

pub struct Module {
    pub name: Ident,
    pub members: Vec<ModuleMember>,
}

impl Parse for Module {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![mod]>()?;
        let name = input.parse::<Ident>()?;
        let content;
        braced!(content in input);
        let mut members = Vec::new();
        while !content.is_empty() {
            members.push(content.parse::<ModuleMember>()?);
        }
        Ok(Self { name, members })
    }
}

impl ToWriter for Module {
    fn to_writer(&self, namespace: &str, items: &mut Vec<writer::Item>) -> Result<()> {
        for member in &self.members {
            match member {
                ModuleMember::Module(member) => member.to_writer(&format!("{namespace}.{}", member.name), items)?,
                ModuleMember::Interface(member) => member.to_writer(namespace, items)?,
                ModuleMember::Struct(member) => member.to_writer(namespace, items)?,
            }
        }
        Ok(())
    }
}

pub enum ModuleMember {
    Module(Module),
    Interface(Interface),
    Struct(ItemStruct),
}

impl Parse for ModuleMember {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![mod]) {
            Ok(ModuleMember::Module(input.parse()?))
        } else if lookahead.peek(keywords::interface) {
            Ok(ModuleMember::Interface(input.parse()?))
        } else if lookahead.peek(Token![struct]) {
            Ok(ModuleMember::Struct(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct Interface {
    pub name: Ident,
    pub methods: Vec<TraitItemMethod>,
}

impl Parse for Interface {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<keywords::interface>()?;
        let name = input.parse::<Ident>()?;
        let content;
        braced!(content in input);
        let mut methods = Vec::new();
        while !content.is_empty() {
            methods.push(content.parse::<TraitItemMethod>()?);
        }
        Ok(Self { name, methods })
    }
}

impl ToWriter for Interface {
    fn to_writer(&self, namespace: &str, items: &mut Vec<writer::Item>) -> Result<()> {
        let mut methods = Vec::new();

        for method in &self.methods {
            methods.push(writer::Method { name: method.sig.ident.to_string(), return_type: writer::Type::Void, params: vec![] });
        }

        items.push(writer::Interface::item(namespace, &self.name.to_string(), methods));
        Ok(())
    }
}

impl ToWriter for ItemStruct {
    fn to_writer(&self, namespace: &str, items: &mut Vec<writer::Item>) -> Result<()> {
        items.push(writer::Struct::item(namespace, &self.ident.to_string(), vec![]));
        Ok(())
    }
}
