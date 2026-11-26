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
                if *tn != Type::Pointer64Helper.type_name()
                    && *tn != Type::UnalignedHelper.type_name()
                    && config.plan.types.contains_key(tn)
                {
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
    pub(crate) fn write_package(&self, tree: &TypeTree, output: &Path) {
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
            let mut tokens = TokenStream::new();

            for (name, tree) in &tree.nested {
                if prunable.contains(tree.namespace) {
                    continue;
                }

                let name = to_ident(name);
                let feature = namespace_feature(tree.namespace);

                tokens.combine(quote! {
                    #[cfg(feature = #feature)]
                    pub mod #name;
                });
            }

            if self.reader.legacy() {
                tokens.combine(self.write_legacy_reexports(tree.namespace));
            }

            let config = self
                .with_namespace(tree.namespace)
                .with_prunable(prunable.clone());

            for ty in &tree.types {
                tokens.combine(ty.write(&config));
            }

            let path = directory.join("mod.rs");
            write_to_file(&path, self.format(tokens));
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

        // Sort feature lines by feature name for stable Cargo.toml output.
        let mut feature_lines: Vec<String> = Vec::new();

        for tree in trees.iter().skip(1) {
            if prunable.contains(tree.namespace) {
                continue;
            }

            let feature = namespace_feature(tree.namespace);

            // Legacy features depend on their parent; Win32 and WDK roots need `Win32_Foundation`.
            if self.reader.legacy() {
                if let Some((dependency, _)) = feature.rsplit_once('_') {
                    feature_lines.push(format!("{feature} = [\"{dependency}\"]"));
                } else if namespace_starts_with(tree.namespace, "Windows.Win32")
                    || namespace_starts_with(tree.namespace, "Windows.Wdk")
                {
                    feature_lines.push(format!("{feature} = [\"Win32_Foundation\"]"));
                } else if tree.namespace == "Windows.Foundation" {
                    feature_lines.push(format!("{feature} = []"));
                } else {
                    feature_lines.push(format!("{feature} = [\"Foundation\"]"));
                }
                continue;
            }

            // Nested namespaces depend on their parent; WinRT roots depend on Foundation.
            let (parent, _leaf) = tree.namespace.rsplit_once('.').unwrap();

            if parent != "Windows" {
                let dependency = namespace_feature(parent);

                feature_lines.push(format!("{feature} = [\"{dependency}\"]"));
            } else if tree.namespace == "Windows.Foundation"
                || tree.namespace == "Windows.Win32"
                || tree.namespace == "Windows.Wdk"
            {
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

impl Config<'_> {
    /// Legacy packages defined types now owned by sibling crates, such as `IReference` and
    /// `DateTime`; re-export them at their metadata paths.
    fn write_legacy_reexports(&self, namespace: &str) -> TokenStream {
        let mut names: Vec<_> = self
            .plan
            .external
            .iter()
            .filter(|(name, _)| name.namespace() == namespace)
            .map(|(name, reference)| format!("{reference}::{}", name.name()))
            .collect();

        names.sort();

        let names = names
            .iter()
            .map(|name| name.parse::<TokenStream>().unwrap());
        quote! { #(pub use #names;)* }
    }
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
