use super::*;

/// Cargo-feature `#[cfg(...)]` emission for `--package` output.
#[derive(Default)]
pub struct Cfg {
    features: BTreeSet<&'static str>,
}

impl Cfg {
    pub fn new(dependencies: &TypeMap, config: &Config) -> Self {
        if !config.bindgen.layout.is_package() {
            return Self::default();
        }

        let features: BTreeSet<&'static str> = dependencies
            .keys()
            .filter_map(|tn| {
                if config.types.contains_key(tn) {
                    Some(tn.namespace())
                } else {
                    None
                }
            })
            .collect();

        Self { features }
    }

    pub fn difference(&self, dependencies: &TypeMap, config: &Config) -> Self {
        if !config.bindgen.layout.is_package() {
            return Self::default();
        }

        let mut difference = Self::new(dependencies, config);

        for feature in &self.features {
            difference.features.remove(feature);
        }

        difference
    }

    pub fn write(&self, config: &Config, not: bool) -> TokenStream {
        let mut compact = BTreeSet::<&'static str>::new();

        for feature in self.features.iter().rev() {
            let mut keep = true;

            for compact in &compact {
                if namespace_starts_with(compact, feature) {
                    keep = false;
                    break;
                }
            }

            if keep {
                compact.insert(feature);
            }
        }

        let mut features = BTreeSet::new();

        for dependency in compact {
            if dependency.is_empty()
                || namespace_starts_with(config.namespace, dependency)
                || dependency == "Windows.Foundation"
                || config.prunable.contains(dependency)
            {
                continue;
            }

            features.insert(namespace_feature(dependency));
        }

        let mut tokens = quote! {};

        match features.len() {
            0 => {}
            1 => {
                if not {
                    tokens.combine(quote! { #[cfg(not(#(feature = #features)*))] });
                } else {
                    tokens.combine(quote! { #[cfg(#(feature = #features)*)] });
                }
            }
            _ => {
                if not {
                    tokens.combine(quote! { #[cfg(not(all( #(feature = #features),* )))] });
                } else {
                    tokens.combine(quote! { #[cfg(all( #(feature = #features),* ))] });
                }
            }
        }

        tokens
    }
}

impl Config<'_> {
    #[track_caller]
    pub(crate) fn write_package(&self, tree: &TypeTree) {
        let output = &self.bindgen.output;
        for name in tree.nested.keys() {
            _ = std::fs::remove_dir_all(output.join("src").join(name));
        }

        let trees = tree.flatten_trees();

        // Pure COM-interface namespaces are empty in `windows-sys` and can be pruned.
        let mut prunable = BTreeSet::new();
        if self.bindgen.style.is_sys() {
            self.collect_prunable(tree, &mut prunable);
        }
        // Share pruned namespaces so cfg gates never reference them.
        let prunable = std::sync::Arc::new(prunable);

        for_each(trees.iter(), |tree| {
            if prunable.contains(tree.namespace) {
                return;
            }

            let directory = output.join("src").join(tree.namespace.replace('.', "/"));

            // Flat Win32/WDK umbrellas glob-reexport private per-header child modules.
            let flatten_children = is_flat_container(tree.namespace);

            let mut tokens = TokenStream::new();

            if flatten_children {
                // Duplicate Win32 free constants stay reachable through their header modules.
                tokens.combine(quote! { #![allow(ambiguous_glob_reexports)] });
            }

            // Prelude-name constants are shadowed back to the prelude after glob reexports.
            let mut prelude_shadows: BTreeMap<&'static str, BTreeSet<String>> = BTreeMap::new();

            for (name, tree) in &tree.nested {
                if prunable.contains(tree.namespace) {
                    continue;
                }

                let name = to_ident(name);

                if flatten_children {
                    let feature = tree.feature();

                    for ty in &tree.types {
                        if matches!(ty, Type::CppConst(_))
                            && prelude_value_shadow(ty.type_name().name()).is_some()
                        {
                            prelude_shadows
                                .entry(ty.type_name().name())
                                .or_default()
                                .insert(feature.clone());
                        }
                    }

                    tokens.combine(quote! {
                        #[cfg(feature = #feature)]
                        pub mod #name;
                        #[cfg(feature = #feature)]
                        pub use #name::*;
                    });
                } else if is_flat_container(tree.namespace) {
                    // The umbrella is always present; only per-header children are feature-gated.
                    tokens.combine(quote! {
                        pub mod #name;
                    });
                } else {
                    let feature = tree.feature();

                    tokens.combine(quote! {
                        #[cfg(feature = #feature)]
                        pub mod #name;
                    });
                }
            }

            for (name, features) in &prelude_shadows {
                let shadow = prelude_value_shadow(name).unwrap();
                let features: Vec<&String> = features.iter().collect();

                let cfg = if features.len() == 1 {
                    quote! { #[cfg(#(feature = #features)*)] }
                } else {
                    quote! { #[cfg(any( #(feature = #features),* ))] }
                };

                tokens.combine(quote! {
                    #cfg
                    pub use #shadow;
                });
            }

            let config = self
                .with_namespace(tree.namespace)
                .with_prunable(prunable.clone());

            for ty in &tree.types {
                tokens.combine(ty.write(&config));
            }

            let path = directory.join("mod.rs");
            write_to_file(&path, self.format(&tokens.into_string()));
        });

        let toml_path = output.join("Cargo.toml");
        let mut toml = String::new();

        for line in read_file_lines(&toml_path) {
            toml.push_str(&line);
            toml.push('\n');

            if line == "# generated features" {
                break;
            }
        }

        let feature_namespaces: BTreeSet<&str> = trees
            .iter()
            .skip(1)
            .map(|tree| tree.namespace)
            .filter(|namespace| !prunable.contains(namespace))
            .filter(|namespace| !is_flat_container(namespace))
            .collect();

        // Sort feature lines by feature name for stable Cargo.toml output.
        let mut feature_lines: Vec<String> = Vec::new();

        for tree in trees.iter().skip(1) {
            if prunable.contains(tree.namespace) {
                continue;
            }

            // The flat umbrella has no feature; its per-header children do.
            if is_flat_container(tree.namespace) {
                continue;
            }

            let feature = tree.feature();

            // Dependencies follow namespace shape: Win32 peers, WinRT parent, or Foundation.
            let (parent, _leaf) = tree.namespace.rsplit_once('.').unwrap();

            if parent == "Windows.Win32" {
                // Win32 header features depend on the other header stems their APIs reference.
                let config = self.with_namespace(tree.namespace);
                let mut dependencies = BTreeSet::new();

                for ty in &tree.types {
                    let cfg = Cfg::new(&ty.dependencies(config.reader), &config);
                    dependencies.extend(cfg.features);
                }

                dependencies.remove(tree.namespace);

                // Sort dependencies by emitted feature name.
                let list = dependencies
                    .iter()
                    .filter(|namespace| feature_namespaces.contains(*namespace))
                    .map(|namespace| namespace_feature(namespace))
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .map(|feature| format!("\"{feature}\""))
                    .collect::<Vec<_>>()
                    .join(", ");

                feature_lines.push(format!("{feature} = [{list}]"));
            } else if parent != "Windows" {
                // Nested WinRT namespaces depend on their parent root feature.
                let dependency = namespace_feature(parent);

                feature_lines.push(format!("{feature} = [\"{dependency}\"]"));
            } else if tree.namespace == "Windows.Foundation" {
                feature_lines.push(format!("{feature} = []"));
            } else {
                feature_lines.push(format!("{feature} = [\"Foundation\"]"));
            }
        }

        feature_lines.sort();
        for line in feature_lines {
            toml.push_str(&line);
            toml.push('\n');
        }

        write_to_file(&toml_path, toml);
    }

    /// Records package namespaces whose modules would be empty.
    fn collect_prunable(&self, tree: &TypeTree, prunable: &mut BTreeSet<&'static str>) -> bool {
        let config = self.with_namespace(tree.namespace);

        let self_empty = tree
            .types
            .iter()
            .all(|ty| ty.write(&config).into_string().trim().is_empty());

        // Visit every child so all prunable descendants are recorded.
        let mut children_prunable = true;
        for child in tree.nested.values() {
            if !self.collect_prunable(child, prunable) {
                children_prunable = false;
            }
        }

        let prune = self_empty && children_prunable;
        if prune && !tree.namespace.is_empty() {
            prunable.insert(tree.namespace);
        }
        prune
    }
}

/// Always-present umbrella module for flat Win32 header stems.
fn is_flat_container(namespace: &str) -> bool {
    namespace == "Windows.Win32"
}

/// Prelude item shadowed by a flat Win32/WDK free constant, if any.
fn prelude_value_shadow(name: &str) -> Option<TokenStream> {
    Some(match name {
        "None" => quote! { core::option::Option::None },
        "Some" => quote! { core::option::Option::Some },
        "Ok" => quote! { core::result::Result::Ok },
        "Err" => quote! { core::result::Result::Err },
        _ => return None,
    })
}

fn for_each<I, F, T>(i: I, f: F)
where
    I: Iterator<Item = T>,
    F: Fn(T) + Sync,
    T: Send,
{
    std::thread::scope(|s| {
        for item in i {
            s.spawn(|| f(item));
        }
    });
}
