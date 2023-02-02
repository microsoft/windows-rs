use metadata::writer;
use syn::{parse::*, *};

mod keywords {
    syn::custom_keyword!(interface);
}

pub trait ToWriter {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()>;
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
    fn to_writer(&self, namespace:String, items: &mut Vec<writer::Item>) -> Result<()> {
        for member in &self.members {
            match member {
                ModuleMember::Module(member) => member.to_writer(format!("{namespace}.{}", member.name), items)?,
                ModuleMember::Interface(member) => member.to_writer(namespace.clone(), items)?,
                ModuleMember::Struct(member) => member.to_writer(namespace.clone(), items)?,
                ModuleMember::Enum(member) => member.to_writer(namespace.clone(), items)?,
            }
        }
        Ok(())
    }
}

pub enum ModuleMember {
    Module(Module),
    Interface(Interface),
    Struct(ItemStruct),
    Enum(ItemEnum),
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
        } else if lookahead.peek(Token![enum]) {
            Ok(ModuleMember::Enum(input.parse()?))
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
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        let mut methods = Vec::new();

        for method in &self.methods {
            methods.push(writer::Method { name: method.sig.ident.to_string(), return_type: writer::Type::Void, params: vec![] });
        }

        items.push(writer::Item::Interface(writer::Interface {namespace, name: self.name.to_string(), methods}));
        Ok(())
    }
}

impl ToWriter for ItemStruct {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        items.push(writer::Item::Struct(writer::Struct{ namespace, name: self.ident.to_string(), fields: vec![] }));
        Ok(())
    }
}

impl ToWriter for ItemEnum {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        //let mut constants = Vec::new();

        items.push(writer::Item::Enum(writer::Enum{ namespace, name: self.ident.to_string(), constants: vec![writer::Constant{ name: "name".to_string(), value: writer::Value::I32(1)}] }));
        Ok(())
    }
}
