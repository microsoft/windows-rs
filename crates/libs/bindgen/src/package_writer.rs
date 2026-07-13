use super::*;

/// Cargo-feature `#[cfg(...)]` emission for `--package` output.
///
/// Only the package layout emits per-namespace cargo features; other layouts
/// never call any of this machinery. Call sites are expected to gate
/// construction on `config.bindgen.layout.is_package()`.
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
            _ = std::fs::remove_dir_all(format!("{output}/src/{name}"));
        }

        let trees = tree.flatten_trees();

        for_each(trees.iter(), |tree| {
            let directory = format!("{output}/src/{}", tree.namespace.replace('.', "/"));

            let mut tokens = TokenStream::new();

            for (name, tree) in &tree.nested {
                let name = to_ident(name);
                let feature = tree.feature();

                tokens.combine(quote! {
                    #[cfg(feature = #feature)]
                    pub mod #name;
                });
            }

            let config = self.with_namespace(tree.namespace);

            for ty in &tree.types {
                tokens.combine(ty.write(&config));
            }

            let path = format!("{directory}/mod.rs");
            write_to_file(&path, self.format(&tokens.into_string()));
        });

        let toml_path = format!("{output}/Cargo.toml");
        let mut toml = String::new();

        for line in read_file_lines(&toml_path) {
            toml.push_str(&line);
            toml.push('\n');

            if line == "# generated features" {
                break;
            }
        }

        let feature_namespaces: BTreeSet<&str> =
            trees.iter().skip(1).map(|tree| tree.namespace).collect();

        for tree in trees.iter().skip(1) {
            let feature = tree.feature();

            // Derive the dependency from the namespace's dot structure. A nested WinRT
            // namespace depends on its parent; a crate-root module is either a flat Win32/WDK
            // header stem (lowercase leaf, e.g. `pathcch`, `wdm`) or a WinRT root (PascalCase
            // leaf, e.g. `Storage`).
            let (parent, leaf) = tree.namespace.rsplit_once('.').unwrap();

            if parent != "Windows" {
                // A nested WinRT namespace (e.g. `Foundation.Collections`) depends on its
                // parent root feature.
                let dependency = namespace_feature(parent);

                toml.push_str(&format!("{feature} = [\"{dependency}\"]\n"));
            } else if leaf.starts_with(|c: char| c.is_ascii_lowercase()) {
                // A flat Win32/WDK header-stem module. There is no umbrella feature, so the
                // stem's cargo feature must pull in exactly the other stems whose types its
                // APIs reference (the same namespaces that gate its items per `Cfg`), so that
                // enabling one header's feature makes its whole surface usable. This mirrors
                // the classic per-feature dependency lists (cargo tolerates the resulting
                // cycles between mutually-referencing headers). References that resolve to
                // always-on core types (no emitted feature) are filtered out.
                let config = self.with_namespace(tree.namespace);
                let mut dependencies = BTreeSet::new();

                for ty in &tree.types {
                    let cfg = Cfg::new(&ty.dependencies(config.reader), &config);
                    dependencies.extend(cfg.features);
                }

                dependencies.remove(tree.namespace);

                let list = dependencies
                    .iter()
                    .filter(|namespace| feature_namespaces.contains(*namespace))
                    .map(|namespace| format!("\"{}\"", namespace_feature(namespace)))
                    .collect::<Vec<_>>()
                    .join(", ");

                toml.push_str(&format!("{feature} = [{list}]\n"));
            } else if tree.namespace == "Windows.Foundation" {
                // The WinRT `Foundation` base is always available with no dependency.
                toml.push_str(&format!("{feature} = []\n"));
            } else {
                // Other WinRT roots depend on the always-on `Foundation` base.
                toml.push_str(&format!("{feature} = [\"Foundation\"]\n"));
            }
        }

        write_to_file(&toml_path, toml);
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
