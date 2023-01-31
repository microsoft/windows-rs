use metadata::writer;
use std::io::Read;
use syn::{parse::*, *};

mod keywords {
    syn::custom_keyword!(interface);
}

#[derive(Debug)]
struct Module {
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

#[derive(Debug)]
enum ModuleMember {
    Module(Module),
    Interface(Interface),
}

impl Parse for ModuleMember {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![mod]) {
            Ok(ModuleMember::Module(input.parse::<Module>()?))
        } else if lookahead.peek(keywords::interface) {
            Ok(ModuleMember::Interface(input.parse::<Interface>()?))
        } else {
            Err(lookahead.error())
        }
    }
}

#[derive(Debug)]
struct Interface {
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

fn syn_to_writer(module: Module) -> Result<Vec<writer::Item>> {
    //println!("{:#?}", &module);
    let mut items = Vec::new();
    module_to_writer(&module.name.to_string(), &module, &mut items)?;
    Ok(items)
}

fn module_to_writer(namespace: &str, module: &Module, items: &mut Vec<writer::Item>) -> Result<()> {
    for member in &module.members {
        match member {
            ModuleMember::Module(module) => module_to_writer(&format!("{namespace}.{}", module.name.to_string()), module, items)?,
            ModuleMember::Interface(interface) => interface_to_writer(&namespace, interface, items)?,
        }
    }
    Ok(())
}

fn interface_to_writer(namespace: &str, interface: &Interface, items: &mut Vec<writer::Item>) -> Result<()> {
    let mut methods = Vec::new();

    for method in &interface.methods {
        methods.push(writer::Method::new(&method.sig.ident.to_string(), writer::Type::Void, vec![]));
    }

    items.push(writer::Interface::item(&namespace, &interface.name.to_string(), methods));
    Ok(())
}

fn main() {
    let filename = "crates/tools/riddle/src/test.rs";
    let output = "crates/tools/riddle/src/test.winmd";

    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("failed to read file");

    let result = parse_str::<Module>(&input).and_then(syn_to_writer);
    match result {
        Ok(items) => {
            let buffer = writer::write("test", true, &items, &[]);
            std::fs::write(output, buffer).expect("failed to write file");
        }
        Err(error) => {
            let start = error.span().start();
            println!("error: {error}\n  --> {}:{:?}:{:?} ", filename, start.line, start.column);
        }
    }
}
