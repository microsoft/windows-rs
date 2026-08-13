use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

#[derive(Clone)]
struct Item {
    name: String,
    kind: u8,
    tokens: TokenStream,
}

#[derive(Default)]
struct Module {
    items: Vec<Item>,
    nested: BTreeMap<String, Self>,
}

impl Generator {
    /// Renders all currently supported items in the requested layout.
    pub fn render(&self, layout: Layout) -> Result<TokenStream, Error> {
        self.render_projection(layout, self.projection)
    }

    pub(super) fn render_projection(
        &self,
        layout: Layout,
        projection: Projection,
    ) -> Result<TokenStream, Error> {
        let modules = self.collect_modules(layout, projection)?;
        let mut root = Module::default();
        match layout {
            Layout::Modules => {
                for (namespace, items) in modules {
                    let mut module = &mut root;
                    if !namespace.is_empty() {
                        for name in namespace.split('.') {
                            module = module.nested.entry(name.to_string()).or_default();
                        }
                    }
                    module.items.extend(items);
                }
            }
            Layout::Flat => {
                let mut names = BTreeMap::<String, String>::new();
                for (namespace, items) in modules {
                    for item in items {
                        if let Some(first_namespace) = names.get(&item.name)
                            && first_namespace != &namespace
                        {
                            return Err(Error::FlatNameCollision {
                                name: item.name,
                                first_namespace: first_namespace.clone(),
                                second_namespace: namespace,
                            });
                        }
                        names.insert(item.name.clone(), namespace.clone());
                        root.items.push(item);
                    }
                }
            }
            Layout::Package => unreachable!(),
        }
        Ok(root.write())
    }

    fn collect_modules(
        &self,
        layout: Layout,
        projection: Projection,
    ) -> Result<BTreeMap<String, Vec<Item>>, Error> {
        let mut modules = BTreeMap::<String, Vec<Item>>::new();
        let values = self.lower_values();
        for item in self.values() {
            let definition = item.definition();
            let namespace = definition.namespace()?;
            let name = definition.name()?;
            modules
                .entry(namespace.to_string())
                .or_default()
                .push(Item {
                    name: name.to_string(),
                    kind: 0,
                    tokens: values.write_context(
                        namespace,
                        name,
                        layout,
                        projection,
                        self.preserve_field_names,
                        self.members(item.definition().entity()),
                    )?,
                });
        }
        for entry in self
            .winrt
            .iter()
            .filter(|entry| entry.kind == WinrtKind::Delegate)
        {
            let definition = self.shared.database.definition(entry.entity).unwrap();
            let namespace = definition.namespace()?;
            let metadata_name = definition.name()?;
            let name = metadata_name
                .split_once('`')
                .map_or(metadata_name, |(name, _)| name);
            let model = winrt_delegate::Delegate::lower(
                &self.shared.database,
                definition,
                &format!("{namespace}.{metadata_name}"),
            )?;
            modules
                .entry(namespace.to_string())
                .or_default()
                .push(Item {
                    name: name.to_string(),
                    kind: 0,
                    tokens: model.write(
                        values,
                        namespace,
                        layout,
                        projection,
                        self.winrt_explicit_items.contains(&entry.entity),
                    )?,
                });
        }
        for entry in self
            .winrt
            .iter()
            .filter(|entry| entry.kind == WinrtKind::Class)
        {
            let definition = self.shared.database.definition(entry.entity).unwrap();
            let namespace = definition.namespace()?;
            let name = definition.name()?;
            let model = winrt_class::Class::lower(
                &self.shared.database,
                definition,
                &self.shared.interface_relationships,
                &format!("{namespace}.{name}"),
            )?;
            modules
                .entry(namespace.to_string())
                .or_default()
                .push(Item {
                    name: name.to_string(),
                    kind: 0,
                    tokens: model.write(
                        values,
                        namespace,
                        layout,
                        projection,
                        self.members(entry.entity),
                        self.winrt_implementations.as_ref(),
                        &self.winrt_members,
                    )?,
                });
        }
        for entry in self
            .winrt
            .iter()
            .filter(|entry| entry.kind == WinrtKind::Interface)
        {
            let definition = self.shared.database.definition(entry.entity).unwrap();
            let namespace = definition.namespace()?;
            let metadata_name = definition.name()?;
            let name = trim_generic_arity(metadata_name);
            let model = winrt_interface::Interface::lower(
                &self.shared.database,
                definition,
                &self.shared.interface_relationships,
                &format!("{namespace}.{metadata_name}"),
            )?;
            modules
                .entry(namespace.to_string())
                .or_default()
                .push(Item {
                    name: name.to_string(),
                    kind: 0,
                    tokens: model.write(
                        values,
                        namespace,
                        layout,
                        projection,
                        self.members(entry.entity),
                        self.implements(entry.entity),
                        self.winrt_explicit_items.contains(&entry.entity),
                    )?,
                });
        }

        self.win32_items().render(
            layout,
            projection,
            &self.derives,
            |namespace, name, kind, tokens| {
                modules
                    .entry(namespace.to_string())
                    .or_default()
                    .push(Item {
                        name: name.to_string(),
                        kind,
                        tokens,
                    });
            },
        )?;
        Ok(modules)
    }

    pub(super) fn write_package(
        &self,
        output: &Path,
        rustfmt: Option<&str>,
        sys: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut modules = self.collect_modules(Layout::Package, self.projection)?;
        let mut namespaces = BTreeSet::new();
        for namespace in modules.keys() {
            let mut namespace = namespace.as_str();
            loop {
                namespaces.insert(namespace.to_string());
                let Some((parent, _)) = namespace.rsplit_once('.') else {
                    break;
                };
                namespace = parent;
            }
        }

        let mut prunable = BTreeSet::new();
        if sys {
            for namespace in namespaces.iter().rev() {
                let has_items = modules
                    .get(namespace)
                    .is_some_and(|items| !items.is_empty());
                let has_children =
                    direct_children(&namespaces, namespace).any(|child| !prunable.contains(child));
                if !has_items && !has_children {
                    prunable.insert(namespace.clone());
                }
            }
        }

        for root in namespaces
            .iter()
            .filter(|namespace| !namespace.contains('.'))
        {
            let _ = std::fs::remove_dir_all(output.join("src").join(root));
        }

        let mut namespace_dependencies = BTreeMap::<String, BTreeSet<String>>::new();
        for namespace in &namespaces {
            if prunable.contains(namespace) {
                continue;
            }
            let flatten_children = namespace == "Windows.Win32";
            let mut tokens = TokenStream::new();
            if flatten_children {
                tokens.extend(quote! { #![allow(ambiguous_glob_reexports)] });
            }

            let children = direct_children(&namespaces, namespace)
                .filter(|child| !prunable.contains(*child))
                .collect::<Vec<_>>();
            for child in &children {
                let name = tokens::ident(child.rsplit_once('.').map_or(child, |(_, name)| name));
                if flatten_children {
                    let feature = namespace_feature(child);
                    tokens.extend(quote! {
                        #[cfg(feature = #feature)]
                        pub mod #name;
                        #[cfg(feature = #feature)]
                        pub use #name::*;
                    });
                } else if child == &"Windows.Win32" {
                    tokens.extend(quote! { pub mod #name; });
                } else {
                    let feature = namespace_feature(child);
                    tokens.extend(quote! {
                        #[cfg(feature = #feature)]
                        pub mod #name;
                    });
                }
            }

            if flatten_children {
                let mut shadows = BTreeMap::<&str, BTreeSet<String>>::new();
                for child in &children {
                    for item in modules.get(*child).into_iter().flatten() {
                        if item.kind == 2 && prelude_shadow(&item.name).is_some() {
                            shadows
                                .entry(item.name.as_str())
                                .or_default()
                                .insert(namespace_feature(child));
                        }
                    }
                }
                for (name, features) in shadows {
                    let shadow = prelude_shadow(name).unwrap();
                    let features = features.iter().collect::<Vec<_>>();
                    let cfg = if features.len() == 1 {
                        quote! { #[cfg(#(feature = #features)*)] }
                    } else {
                        quote! { #[cfg(any( #(feature = #features),* ))] }
                    };
                    tokens.extend(quote! {
                        #cfg
                        pub use #shadow;
                    });
                }
            }

            if let Some(items) = modules.get_mut(namespace) {
                items.sort_by(|left, right| {
                    (left.kind != 3, &left.name, left.kind).cmp(&(
                        right.kind != 3,
                        &right.name,
                        right.kind,
                    ))
                });
                for item in items.drain(..) {
                    tokens.extend(item.tokens);
                }
            }

            let contents = format::format_with_config(&tokens.to_string(), rustfmt)?;
            namespace_dependencies.insert(namespace.clone(), feature_dependencies(&contents));
            let path = output
                .join("src")
                .join(namespace.replace('.', "\\"))
                .join("mod.rs");
            write_if_changed(&path, contents)?;
        }

        let mut features = Vec::new();
        for namespace in namespaces.iter().skip(1) {
            if prunable.contains(namespace)
                || namespace == "Windows.Win32"
                || namespace == "Windows"
            {
                continue;
            }
            let feature = namespace_feature(namespace);
            let (parent, _) = namespace.rsplit_once('.').unwrap();
            let dependencies = if parent == "Windows.Win32" {
                namespace_dependencies
                    .get(namespace)
                    .into_iter()
                    .flatten()
                    .filter(|dependency| dependency.as_str() != feature)
                    .map(|dependency| format!("\"{dependency}\""))
                    .collect::<Vec<_>>()
                    .join(", ")
            } else if parent != "Windows" {
                format!("\"{}\"", namespace_feature(parent))
            } else if namespace == "Windows.Foundation" {
                String::new()
            } else {
                "\"Foundation\"".to_string()
            };
            features.push(format!("{feature} = [{dependencies}]"));
        }
        features.sort();

        let toml_path = output.join("Cargo.toml");
        let existing = std::fs::read_to_string(&toml_path)?;
        let Some((prefix, _)) = existing.split_once("# generated features") else {
            return Err(format!(
                "missing `# generated features` marker in `{}`",
                toml_path.display()
            )
            .into());
        };
        let mut toml = format!("{prefix}# generated features\n");
        for feature in features {
            toml.push_str(&feature);
            toml.push('\n');
        }
        write_if_changed(&toml_path, toml)?;
        Ok(())
    }
}

impl Module {
    fn write(mut self) -> TokenStream {
        self.items.sort_by(|left, right| {
            (left.kind != 3, &left.name, left.kind).cmp(&(right.kind != 3, &right.name, right.kind))
        });
        let items = self.items.into_iter().map(|item| item.tokens);
        let nested = self.nested.into_iter().map(|(name, module)| {
            let name = tokens::ident(&name);
            let module = module.write();
            quote! { pub mod #name { #module } }
        });
        quote! {
            #(#items)*
            #(#nested)*
        }
    }
}

fn direct_children<'a>(
    namespaces: &'a BTreeSet<String>,
    namespace: &'a str,
) -> impl Iterator<Item = &'a str> {
    namespaces.iter().filter_map(move |candidate| {
        let suffix = candidate.strip_prefix(namespace)?;
        let suffix = suffix.strip_prefix('.')?;
        (!suffix.contains('.')).then_some(candidate.as_str())
    })
}

fn namespace_feature(namespace: &str) -> String {
    if let Some(stem) = namespace.strip_prefix("Windows.Win32.") {
        stem.replace('.', "_")
    } else if let Some((_, rest)) = namespace.split_once('.') {
        rest.replace('.', "_")
    } else {
        namespace.to_string()
    }
}

fn prelude_shadow(name: &str) -> Option<TokenStream> {
    Some(match name {
        "None" => quote! { core::option::Option::None },
        "Some" => quote! { core::option::Option::Some },
        "Ok" => quote! { core::result::Result::Ok },
        "Err" => quote! { core::result::Result::Err },
        _ => return None,
    })
}

fn write_if_changed(path: &Path, contents: String) -> Result<(), Box<dyn std::error::Error>> {
    if std::fs::read_to_string(path).is_ok_and(|existing| existing == contents) {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, contents)?;
    Ok(())
}

fn feature_dependencies(contents: &str) -> BTreeSet<String> {
    let mut result = BTreeSet::new();
    let mut rest = contents;
    while let Some((_, suffix)) = rest.split_once("feature = \"") {
        let Some((feature, suffix)) = suffix.split_once('"') else {
            break;
        };
        result.insert(feature.to_string());
        rest = suffix;
    }
    result
}
