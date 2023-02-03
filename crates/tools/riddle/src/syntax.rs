use metadata::writer;
use syn::{parse::*, spanned::*, *};

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
        let mut members = vec![];
        while !content.is_empty() {
            members.push(content.parse::<ModuleMember>()?);
        }
        Ok(Self { name, members })
    }
}

impl ToWriter for Module {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
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
        let mut methods = vec![];
        while !content.is_empty() {
            methods.push(content.parse::<TraitItemMethod>()?);
        }
        Ok(Self { name, methods })
    }
}

impl ToWriter for Interface {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        let mut methods = vec![];

        for method in &self.methods {
            methods.push(writer::Method { name: method.sig.ident.to_string(), return_type: writer::Type::Void, params: vec![] });
        }

        items.push(writer::Item::Interface(writer::Interface { namespace, name: self.name.to_string(), methods }));
        Ok(())
    }
}

impl ToWriter for ItemStruct {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        let mut fields = vec![];

        let Fields::Named(named) = &self.fields else {
            return Err(Error::new(self.fields.span(), "expected named fields"));
        };

        for field in &named.named {
            fields.push(writer::Field { name: field.ident.as_ref().unwrap().to_string(), ty: type_to_type(&field.ty)? });
        }

        items.push(writer::Item::Struct(writer::Struct { namespace, name: self.ident.to_string(), fields }));
        Ok(())
    }
}

impl ToWriter for ItemEnum {
    fn to_writer(&self, namespace: String, items: &mut Vec<writer::Item>) -> Result<()> {
        let mut constants = vec![];
        let mut value = 0;

        // TODO: need to read the `#[repr(u8)]` attribute infer the underlying type

        for variant in &self.variants {
            if let Some((_, discriminant)) = &variant.discriminant {
                let Expr::Lit(discriminant) = discriminant else {
                    return Err(Error::new(discriminant.span(), "expected literal discriminant"));
                };

                let Lit::Int(discriminant) = &discriminant.lit else {
                    return Err(Error::new(discriminant.lit.span(), "expected integer discriminant"));
                };

                value = discriminant.base10_parse()?;
            }

            constants.push(writer::Constant { name: variant.ident.to_string(), value: writer::Value::I32(value) });
            value += 1;
        }

        items.push(writer::Item::Enum(writer::Enum { namespace, name: self.ident.to_string(), constants }));
        Ok(())
    }
}

fn type_to_type(ty: &Type) -> Result<writer::Type> {
    let Type::Path(path) = ty else {
        return Err(Error::new(ty.span(), "expected type path"));
    };

    let mut name = String::new();

    for segment in &path.path.segments {
        if !name.is_empty() {
            name.push('.');
        }
        name.push_str(&segment.ident.to_string());
    }

    let ty = match name.as_str() {
        "bool" => writer::Type::Bool,
        "i8" => writer::Type::I8,
        "u8" => writer::Type::U8,
        "i16" => writer::Type::I16,
        "u16" => writer::Type::U16,
        "i32" => writer::Type::I32,
        "u32" => writer::Type::U32,
        "i64" => writer::Type::I64,
        "u64" => writer::Type::U64,
        "f32" => writer::Type::F32,
        "f64" => writer::Type::F64,
        "isize" => writer::Type::ISize,
        "usize" => writer::Type::USize,
        _ => {
            let Some((namespace, name)) = name.rsplit_once('.') else {
                return Err(Error::new(path.span(), "expected type"));
            };
            writer::Type::Named((namespace.to_string(), name.to_string()))
        }
    };

    Ok(ty)
}
