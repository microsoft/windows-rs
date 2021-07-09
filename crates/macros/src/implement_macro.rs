use gen::{tables::TypeDef, TypeKind, TypeReader};
use std::collections::*;
use syn::parse::*;
use syn::*;

custom_keyword!(extend);

#[derive(Debug, Default)]
pub struct ImplementMacro {
    pub extend: Option<(&'static str, &'static str)>,
    pub overrides: BTreeSet<&'static str>,
    pub implement: BTreeSet<(&'static str, &'static str, Vec<String>)>,
}

impl ImplementMacro {
    pub fn interfaces(&self, reader: &'static TypeReader) -> Vec<(TypeDef, bool)> {
        // TODO: any one of `self.implement` could be a class in which case its interfaces should be enumerated

        let mut result = Vec::new();

        for (namespace, name, generics) in &self.implement {
            result.push((reader.resolve_type_def(namespace, name), false));
        }

        if let Some((namespace, name)) = self.extend {
            let extend = reader.resolve_type_def(namespace, name);

            for interface in extend.overridable_interfaces() {
                result.push((interface, true));
            }
        }

        result
    }

    fn parse_implement(&mut self, reader: &'static TypeReader, cursor: ParseStream) -> Result<()> {
        if let Ok(tree) = cursor.parse::<UseTree2>() {
            self.walk_implement(reader, &tree, &mut String::new())?;

            if !cursor.is_empty() {
                cursor.parse::<Token![,]>()?;
            }
        }

        Ok(())
    }

    fn walk_implement(
        &mut self,
        reader: &'static TypeReader,
        tree: &UseTree2,
        namespace: &mut String,
    ) -> Result<()> {
        match tree {
            UseTree2::Path(input) => {
                if !namespace.is_empty() {
                    namespace.push('.');
                }

                namespace.push_str(&input.ident.to_string());
                self.walk_implement(reader, &*input.tree, namespace)?;
            }
            UseTree2::Name(input) => {
                let name = input.ident.to_string();

                if let Some((namespace, name)) = reader.get_type_name(namespace, &name) {
                    match reader.resolve_type_def(namespace, name).kind() {
                        TypeKind::Class | TypeKind::Interface => {
                            self.implement.insert((namespace, name, Vec::new()));
                        }
                        _ => {
                            return Err(Error::new_spanned(
                                &input.ident,
                                format!("`{}.{}` not a class or interface", namespace, name),
                            ));
                        }
                    }
                } else {
                    return Err(Error::new_spanned(
                        &input.ident,
                        format!("`{}.{}` not found in metadata", namespace, name),
                    ));
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

    fn parse_override(&mut self, reader: &'static TypeReader, cursor: ParseStream) -> Result<()> {
        // Any number of methods may be overridden but only if a class is being overridden.
        if let Some((namespace, name)) = self.extend {
            if cursor.parse::<Token![override]>().is_ok() {
                let methods = reader
                    .resolve_type_def(namespace, name)
                    .overridable_methods();

                while let Ok(input) = cursor.parse::<Ident>() {
                    let name = input.to_string();

                    if let Some(name) = methods.get(name.as_str()) {
                        self.overrides.insert(name);
                    } else {
                        return Err(Error::new_spanned(
                            input,
                            format!("`{}` not an overridable method", name),
                        ));
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

    fn walk_extend(
        &mut self,
        reader: &'static TypeReader,
        tree: &UseTree2,
        namespace: &mut String,
    ) -> Result<()> {
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

                if let Some((namespace, name)) = reader.get_type_name(namespace, &name) {
                    if reader
                        .resolve_type_def(namespace, name)
                        .is_public_composable()
                    {
                        self.extend.replace((namespace, name));
                    } else {
                        return Err(Error::new_spanned(
                            &input.ident,
                            format!("`{}.{}` not extendable", namespace, name),
                        ));
                    }
                } else {
                    return Err(Error::new_spanned(
                        &input.ident,
                        format!("`{}.{}` not found in metadata", namespace, name),
                    ));
                }
            }
            UseTree2::Group(input) => {
                return Err(Error::new(input.brace_token.span, "Syntax not supported"));
            }
        }

        Ok(())
    }
}

impl Parse for ImplementMacro {
    fn parse(cursor: ParseStream) -> Result<Self> {
        let mut input = Self::default();
        let reader = TypeReader::get();

        while !cursor.is_empty() {
            input.parse_extend(reader, cursor)?;
            input.parse_override(reader, cursor)?;
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
    pub generics: syn::punctuated::Punctuated<UseTree2, Token![,]>,
}

pub struct UseGroup2 {
    pub brace_token: token::Brace,
    pub items: syn::punctuated::Punctuated<UseTree2, Token![,]>,
}

impl Parse for UseTree2 {
    fn parse(input: ParseStream) -> Result<UseTree2> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident)
            || lookahead.peek(Token![self])
            || lookahead.peek(Token![super])
            || lookahead.peek(Token![crate])
        {
            use syn::ext::IdentExt;
            let ident = input.call(Ident::parse_any)?;
            if input.peek(Token![::]) {
                Ok(UseTree2::Path(UsePath2 {
                    ident,
                    colon2_token: input.parse()?,
                    tree: Box::new(input.parse()?),
                }))
            } else {
                let generics = if input.peek(Token![<]) {
                    input.parse::<Token![<]>()?;
                    let items = input.parse_terminated(UseTree2::parse)?;
                    input.parse::<Token![>]>()?;
                    items
                } else {
                    syn::punctuated::Punctuated::new()
                };

                Ok(UseTree2::Name(UseName2 { ident, generics }))
            }
        } else if lookahead.peek(token::Brace) {
            let content;
            Ok(UseTree2::Group(UseGroup2 {
                brace_token: braced!(content in input),
                items: content.parse_terminated(UseTree2::parse)?,
            }))
        } else {
            Err(lookahead.error())
        }
    }
}
