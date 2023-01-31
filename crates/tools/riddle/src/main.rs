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

fn main() {
    let filename = "crates/tools/riddle/src/test.ridl";
    let mut file = std::fs::File::open(filename).expect("failed to open file");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("failed to read file");

    let result = parse_str::<Module>(&input);
    match result {
        Ok(result) => println!("{result:#?}"),
        Err(error) => {
            let start = error.span().start();
            println!("error: {error}\n  --> {}:{:?}:{:?} ", filename, start.line, start.column);
        }
    }
}
