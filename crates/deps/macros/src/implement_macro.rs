use super::*;
use std::collections::*;
use syn::parse::*;
use syn::Ident;
use syn::*;

custom_keyword!(extend);

#[derive(Default)]
pub struct ImplementMacro {
    pub implement: BTreeSet<TypeDef>,
    pub extend: Option<TypeDef>,
    pub overrides: BTreeSet<&'static str>,
}

impl ImplementMacro {
    pub fn interfaces(&self) -> Vec<(TypeDef, bool)> {
        // TODO: any one of `self.implement` could be a class in which case its interfaces should be enumerated

        let mut result: Vec<(TypeDef, bool)> = self.implement.iter().map(|def| (def.clone(), false)).collect();

        if let Some(extend) = &self.extend {
            for interface in extend.overridable_interfaces() {
                result.push((interface, true));
            }
        }

        result
    }

    fn parse_implement(&mut self, reader: &'static TypeReader, cursor: ParseStream) -> Result<()> {
        let tree = cursor.parse::<UseTree2>()?;
        self.walk_implement(reader, &tree, &mut String::new())?;

        if !cursor.is_empty() {
            cursor.parse::<Token![,]>()?;
        }

        Ok(())
    }

    fn walk_implement(&mut self, reader: &'static TypeReader, tree: &UseTree2, namespace: &mut String) -> Result<()> {
        match tree {
            UseTree2::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push('.');
                }

                namespace.push_str(&input.ident.to_string());
                self.walk_implement(reader, &*input.tree, namespace)?;
            }
            UseTree2::Name(input) => {
                if let ElementType::TypeDef(def) = tree.to_element_type(reader, namespace)? {
                    self.implement.insert(def);
                } else {
                    return Err(Error::new_spanned(&input.ident, format!("`{}.{}` is not a class or interface", namespace, input.ident)));
                }
            }
            UseTree2::Group(input) => {
                for tree in &input.items {
                    self.walk_implement(reader, tree, namespace)?;
                }
            }
        }

        Ok(())
    }

    fn parse_override(&mut self, cursor: ParseStream) -> Result<()> {
        // Any number of methods may be overridden but only if a class is being overridden.
        if let Some(extend) = &self.extend {
            while cursor.parse::<Token![override]>().is_ok() {
                let methods = extend.overridable_methods();

                while let Ok(input) = cursor.parse::<Ident>() {
                    let name = input.to_string();

                    if let Some(name) = methods.get(name.as_str()) {
                        self.overrides.insert(name);
                    } else {
                        return Err(Error::new_spanned(input, format!("`{}` not an overridable method", name)));
                    }
                }

                if !cursor.is_empty() {
                    cursor.parse::<Token![,]>()?;
                }
            }
        }

        Ok(())
    }

    fn parse_extend(&mut self, reader: &'static TypeReader, cursor: ParseStream) -> Result<()> {
        // Only one class may be extended
        if self.extend.is_none() && cursor.parse::<extend>().is_ok() {
            self.walk_extend(reader, &cursor.parse()?, &mut String::new())?;

            if !cursor.is_empty() {
                cursor.parse::<Token![,]>()?;
            }
        }

        Ok(())
    }

    fn walk_extend(&mut self, reader: &'static TypeReader, tree: &UseTree2, namespace: &mut String) -> Result<()> {
        match tree {
            UseTree2::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push('.');
                }

                namespace.push_str(&input.ident.to_string());
                self.walk_extend(reader, &*input.tree, namespace)?;
            }
            UseTree2::Name(input) => {
                let name = input.ident.to_string();

                if let Some(def) = get_public_composable(reader, namespace, &name) {
                    self.extend.replace(def);
                } else {
                    return Err(Error::new_spanned(&input.ident, format!("`{}.{}` not extendable", namespace, name)));
                }
            }
            UseTree2::Group(input) => {
                return Err(Error::new(input.brace_token.span, "Syntax not supported"));
            }
        }

        Ok(())
    }
}

fn get_public_composable(reader: &'static TypeReader, namespace: &str, name: &str) -> Option<TypeDef> {
    if let Some(ElementType::TypeDef(def)) = reader.get_type((namespace, name)) {
        if def.is_public_composable() {
            return Some(def.clone());
        }
    }

    None
}

fn get_implementable(reader: &'static TypeReader, namespace: &str, name: &str) -> Option<TypeDef> {
    if let Some(ElementType::TypeDef(def)) = reader.get_type((namespace, name)) {
        match def.kind() {
            TypeKind::Class | TypeKind::Interface => return Some(def.clone()),
            _ => {}
        }
    }

    None
}

impl Parse for ImplementMacro {
    fn parse(cursor: ParseStream) -> Result<Self> {
        let mut input = Self::default();
        let reader = TypeReader::get();
        input.parse_extend(reader, cursor)?;
        input.parse_override(cursor)?;

        while !cursor.is_empty() {
            input.parse_implement(reader, cursor)?;
        }

        Ok(input)
    }
}

pub enum UseTree2 {
    Path(UsePath2),
    Name(UseName2),
    Group(UseGroup2),
}

pub struct UsePath2 {
    pub ident: Ident,
    pub colon2_token: Token![::],
    pub tree: Box<UseTree2>,
}

pub struct UseName2 {
    pub ident: Ident,
    pub generics: Vec<UseTree2>,
}

pub struct UseGroup2 {
    pub brace_token: token::Brace,
    pub items: syn::punctuated::Punctuated<UseTree2, Token![,]>,
}

impl UseTree2 {
    fn to_element_type(&self, reader: &'static TypeReader, namespace: &mut String) -> Result<ElementType> {
        match self {
            UseTree2::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push('.');
                }

                namespace.push_str(&input.ident.to_string());
                input.tree.to_element_type(reader, namespace)
            }
            UseTree2::Name(input) => {
                let name = input.ident.to_string();

                if reader.types.get_namespace(namespace).is_some() {
                    if let Some(mut def) = get_implementable(reader, namespace, &name) {
                        for g in &input.generics {
                            def.generics.push(g.to_element_type(reader, &mut String::new())?);
                        }

                        Ok(ElementType::TypeDef(def))
                    } else {
                        Err(Error::new_spanned(&input.ident, format!("`{}.{}` not a class or interface", namespace, name)))
                    }
                } else if let Some(def) = ElementType::from_string_lossy(&name) {
                    Ok(def)
                } else {
                    Ok(ElementType::GenericParam(name))
                }
            }
            UseTree2::Group(input) => Err(Error::new(input.brace_token.span, "Syntax not supported")),
        }
    }
}

impl Parse for UseTree2 {
    fn parse(input: ParseStream) -> Result<UseTree2> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            use syn::ext::IdentExt;
            let ident = input.call(Ident::parse_any)?;
            if input.peek(Token![::]) {
                Ok(UseTree2::Path(UsePath2 { ident, colon2_token: input.parse()?, tree: Box::new(input.parse()?) }))
            } else {
                let generics = if input.peek(Token![<]) {
                    input.parse::<Token![<]>()?;
                    let mut generics = Vec::new();
                    loop {
                        generics.push(input.parse::<UseTree2>()?);

                        if input.parse::<Token![,]>().is_err() {
                            break;
                        }
                    }
                    input.parse::<Token![>]>()?;
                    generics
                } else {
                    Vec::new()
                };

                Ok(UseTree2::Name(UseName2 { ident, generics }))
            }
        } else if lookahead.peek(token::Brace) {
            let content;
            let brace_token = braced!(content in input);
            let items = content.parse_terminated(UseTree2::parse)?;

            Ok(UseTree2::Group(UseGroup2 { brace_token, items }))
        } else {
            Err(lookahead.error())
        }
    }
}
