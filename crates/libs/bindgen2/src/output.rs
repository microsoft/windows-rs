use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

#[derive(Clone)]
struct Item {
    name: String,
    kind: ArtifactKind,
    variant: i32,
    tokens: TokenStream,
    features: BTreeSet<String>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) enum ArtifactKind {
    Source(u8),
    Manifest,
}

impl ArtifactKind {
    const fn source(self) -> Option<u8> {
        match self {
            Self::Source(kind) => Some(kind),
            Self::Manifest => None,
        }
    }
}

#[derive(Default)]
struct Module {
    items: Vec<Item>,
    nested: BTreeMap<String, Self>,
}

pub(super) struct PackagePlan {
    pub(super) removals: Vec<PathBuf>,
    pub(super) modules: Vec<PackageModule>,
    pub(super) features: Vec<String>,
}

pub(super) struct PackageModule {
    pub(super) path: PathBuf,
    pub(super) tokens: TokenStream,
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
        let total = std::time::Instant::now();
        let mut modules = BTreeMap::<String, Vec<Item>>::new();
        let phase = std::time::Instant::now();
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
                    kind: ArtifactKind::Source(0),
                    variant: 0,
                    tokens: values.write_context(
                        namespace,
                        name,
                        layout,
                        projection,
                        self.preserve_field_names,
                        self.members(item.definition().entity()),
                    )?,
                    features: BTreeSet::new(),
                });
        }
        report_timing("render WinRT values", phase.elapsed());
        let phase = std::time::Instant::now();
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
            let mut model = self.shared.winrt_catalogs.delegate(entry.entity).clone();
            model.expand_package_dependencies(&self.shared.winrt_artifacts);
            let features = tokens::feature_names(
                namespace,
                layout,
                model
                    .model_dependencies()
                    .iter()
                    .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
            );
            let tokens = model.write(
                values,
                namespace,
                layout,
                projection,
                self.winrt_explicit_items.contains(&entry.entity),
            )?;
            modules
                .entry(namespace.to_string())
                .or_default()
                .push(Item {
                    name: name.to_string(),
                    kind: ArtifactKind::Source(0),
                    variant: 0,
                    tokens,
                    features,
                });
        }
        report_timing("render WinRT delegates", phase.elapsed());
        let phase = std::time::Instant::now();
        for entry in self
            .winrt
            .iter()
            .filter(|entry| entry.kind == WinrtKind::Class)
        {
            let definition = self.shared.database.definition(entry.entity).unwrap();
            let namespace = definition.namespace()?;
            let name = definition.name()?;
            let mut model = self.shared.winrt_catalogs.class(entry.entity).clone();
            model.expand_package_dependencies(&self.shared.winrt_artifacts);
            modules
                .entry(namespace.to_string())
                .or_default()
                .push(Item {
                    name: name.to_string(),
                    kind: ArtifactKind::Source(0),
                    variant: 0,
                    tokens: model.write(
                        &winrt_class::WriteContext::new(
                            values,
                            namespace,
                            layout,
                            projection,
                            self.winrt_implementations.as_ref(),
                            &self.winrt_members,
                        ),
                        self.members(entry.entity),
                    )?,
                    features: BTreeSet::new(),
                });
        }
        report_timing("render WinRT classes", phase.elapsed());
        let phase = std::time::Instant::now();
        for entry in self
            .winrt
            .iter()
            .filter(|entry| entry.kind == WinrtKind::Interface)
        {
            let definition = self.shared.database.definition(entry.entity).unwrap();
            let namespace = definition.namespace()?;
            let metadata_name = definition.name()?;
            let name = trim_generic_arity(metadata_name);
            let mut model = self.shared.winrt_catalogs.interface(entry.entity).clone();
            model.expand_package_dependencies(&self.shared.winrt_artifacts);
            modules
                .entry(namespace.to_string())
                .or_default()
                .push(Item {
                    name: name.to_string(),
                    kind: ArtifactKind::Source(0),
                    variant: 0,
                    tokens: model.write(
                        &winrt_interface::WriteContext::new(values, namespace, layout, projection),
                        self.members(entry.entity),
                        self.implements(entry.entity),
                        self.winrt_explicit_items.contains(&entry.entity),
                    )?,
                    features: BTreeSet::new(),
                });
        }
        report_timing("render WinRT interfaces", phase.elapsed());

        let phase = std::time::Instant::now();
        self.win32_items().render(
            layout,
            projection,
            &self.derives,
            |namespace, name, kind, variant, tokens, features| {
                modules
                    .entry(namespace.to_string())
                    .or_default()
                    .push(Item {
                        name: name.to_string(),
                        kind,
                        variant,
                        tokens,
                        features,
                    });
            },
        )?;
        report_timing("render Win32", phase.elapsed());
        report_timing("render total", total.elapsed());
        Ok(modules)
    }

    pub(super) fn package_plan(&self, sys: bool) -> Result<PackagePlan, Error> {
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
                    .is_some_and(|items| items.iter().any(|item| item.kind.source().is_some()));
                let has_children =
                    direct_children(&namespaces, namespace).any(|child| !prunable.contains(child));
                if !has_items && !has_children {
                    prunable.insert(namespace.clone());
                }
            }
        }

        let removals = namespaces
            .iter()
            .filter(|namespace| !namespace.contains('.'))
            .map(|root| PathBuf::from("src").join(root))
            .collect();

        let mut namespace_dependencies = BTreeMap::<String, BTreeSet<String>>::new();
        let mut module_files = Vec::new();
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
                        if item.kind == ArtifactKind::Source(2)
                            && prelude_shadow(&item.name).is_some()
                        {
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
                items.sort_by(compare_items);
                let mut dependencies = BTreeSet::new();
                for item in items.drain(..) {
                    dependencies.extend(item.features);
                    if item.kind.source().is_some() {
                        tokens.extend(item.tokens);
                    }
                }
                namespace_dependencies.insert(namespace.clone(), dependencies);
            }

            let path = PathBuf::from("src")
                .join(namespace.replace('.', "\\"))
                .join("mod.rs");
            module_files.push(PackageModule { path, tokens });
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

        Ok(PackagePlan {
            removals,
            modules: module_files,
            features,
        })
    }
}

impl Module {
    fn write(mut self) -> TokenStream {
        self.items.sort_by(compare_items);
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

fn compare_items(left: &Item, right: &Item) -> Ordering {
    (
        left.kind != ArtifactKind::Source(3),
        &left.name,
        left.kind,
        left.variant,
    )
        .cmp(&(
            right.kind != ArtifactKind::Source(3),
            &right.name,
            right.kind,
            right.variant,
        ))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn architecture_variants_sort_by_metadata_key() {
        let item = |variant| Item {
            name: "DUPLICATE".to_string(),
            kind: ArtifactKind::Source(2),
            variant,
            tokens: TokenStream::new(),
            features: BTreeSet::new(),
        };
        let mut items = [item(5), item(2)];
        items.sort_by(compare_items);
        assert_eq!(items.map(|item| item.variant), [2, 5]);
    }
}
