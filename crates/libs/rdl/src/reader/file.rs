use super::*;

#[derive(Debug)]
pub struct File {
    pub items: Vec<Item>,
    pub imports: Vec<Import>,
    pub source: String,
}

#[derive(Debug)]
pub struct Import {
    pub path: Vec<String>,
    pub local: Option<String>,
    pub glob: bool,
    pub span: Span,
}

impl syn::parse::Parse for File {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = vec![];
        let mut imports = vec![];
        while !input.is_empty() {
            if input.peek(syn::Token![use]) {
                let item = input.parse::<syn::ItemUse>()?;
                collect_imports(&item.tree, &mut vec![], &mut imports)?;
            } else {
                items.push(Item::Module(input.parse()?));
            }
        }

        Ok(Self {
            items,
            imports,
            source: String::new(),
        })
    }
}

fn collect_imports(
    tree: &syn::UseTree,
    prefix: &mut Vec<String>,
    imports: &mut Vec<Import>,
) -> syn::Result<()> {
    use syn::spanned::Spanned;

    match tree {
        syn::UseTree::Path(path) => {
            let name = path.ident.unraw_to_string();
            if prefix.is_empty() && matches!(name.as_str(), "crate" | "self" | "super") {
                return Err(syn::Error::new(
                    path.ident.span(),
                    "RDL imports must use an absolute metadata namespace",
                ));
            }
            prefix.push(name);
            collect_imports(&path.tree, prefix, imports)?;
            prefix.pop();
        }
        syn::UseTree::Name(name) => {
            let name = name.ident.unraw_to_string();
            let path = if name == "self" {
                if prefix.is_empty() {
                    return Err(syn::Error::new(
                        tree.span(),
                        "`self` import requires a namespace prefix",
                    ));
                }
                prefix.clone()
            } else {
                let mut path = prefix.clone();
                path.push(name);
                path
            };
            imports.push(Import {
                local: path.last().cloned(),
                path,
                glob: false,
                span: tree.span(),
            });
        }
        syn::UseTree::Rename(rename) => {
            let mut path = prefix.clone();
            let name = rename.ident.unraw_to_string();
            if name != "self" {
                path.push(name);
            } else if path.is_empty() {
                return Err(syn::Error::new(
                    rename.ident.span(),
                    "`self` import requires a namespace prefix",
                ));
            }
            imports.push(Import {
                path,
                local: Some(rename.rename.unraw_to_string()),
                glob: false,
                span: tree.span(),
            });
        }
        syn::UseTree::Glob(_) => {
            if prefix.is_empty() {
                return Err(syn::Error::new(
                    tree.span(),
                    "glob import requires a namespace",
                ));
            }
            imports.push(Import {
                path: prefix.clone(),
                local: None,
                glob: true,
                span: tree.span(),
            });
        }
        syn::UseTree::Group(group) => {
            for tree in &group.items {
                collect_imports(tree, prefix, imports)?;
            }
        }
    }
    Ok(())
}
