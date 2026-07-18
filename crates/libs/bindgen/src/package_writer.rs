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

        // In the flat `windows-sys` package a namespace of pure COM interfaces emits
        // nothing — sys renders no interfaces — leaving an empty module file and a dead
        // Cargo feature. Collect those prunable namespaces so their module declaration,
        // file, feature, and any dependency-list references are all suppressed.
        let mut prunable = BTreeSet::new();
        if self.bindgen.style.is_sys() {
            self.collect_prunable(tree, &mut prunable);
        }

        for_each(trees.iter(), |tree| {
            if prunable.contains(tree.namespace) {
                return;
            }

            let directory = format!("{output}/src/{}", tree.namespace.replace('.', "/"));

            // Children of a `Win32`/`Wdk` umbrella are private per-header submodules whose contents
            // are glob-re-exported, so the whole flat Win32/WDK surface is reachable directly under
            // `Win32`/`Wdk` (e.g. `windows::Win32::CreateFileW`) with no per-header module in the
            // public path — while each still compiles as its own feature-gated file.
            let flatten_children = is_flat_container(tree.namespace);

            let mut tokens = TokenStream::new();

            // Free constants under a flattened Win32/WDK umbrella whose names shadow a Rust prelude
            // item (e.g. `None`), mapped to the features that define them, so the glob re-export can
            // be shadowed back to the prelude below.
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
                        mod #name;
                        #[cfg(feature = #feature)]
                        pub use #name::*;
                    });
                } else if is_flat_container(tree.namespace) {
                    // The `Win32`/`Wdk` umbrella is an always-present container; its per-header
                    // children carry the Cargo features, so the umbrella itself is never gated.
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

        let feature_namespaces: BTreeSet<&str> = trees
            .iter()
            .skip(1)
            .map(|tree| tree.namespace)
            .filter(|namespace| !prunable.contains(namespace))
            .filter(|namespace| !is_flat_container(namespace))
            .collect();

        for tree in trees.iter().skip(1) {
            if prunable.contains(tree.namespace) {
                continue;
            }

            // The `Win32`/`Wdk` umbrella is a pure container with no types of its own; it has no
            // Cargo feature (its per-header children do), so it is skipped here entirely.
            if is_flat_container(tree.namespace) {
                continue;
            }

            let feature = tree.feature();

            // Derive the dependency from the namespace's dot structure. A per-header Win32/WDK
            // namespace (`Windows.Win32.<stem>` / `Windows.Wdk.<stem>`) pulls in the other header
            // stems its APIs reference; a nested WinRT namespace depends on its parent root; a WinRT
            // root depends on the always-on `Foundation` base.
            let (parent, _leaf) = tree.namespace.rsplit_once('.').unwrap();

            if parent == "Windows.Win32" || parent == "Windows.Wdk" {
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

                // Sort by the emitted feature name (not the source namespace) so the list stays
                // stable and readable regardless of how the `Windows.Win32` / `Windows.Wdk`
                // umbrellas order the underlying namespaces, and so a Win32/WDK stem-name overlap
                // collapses to a single feature entry.
                let list = dependencies
                    .iter()
                    .filter(|namespace| feature_namespaces.contains(*namespace))
                    .map(|namespace| namespace_feature(namespace))
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .map(|feature| format!("\"{feature}\""))
                    .collect::<Vec<_>>()
                    .join(", ");

                toml.push_str(&format!("{feature} = [{list}]\n"));
            } else if parent != "Windows" {
                // A nested WinRT namespace (e.g. `Foundation.Collections`) depends on its
                // parent root feature.
                let dependency = namespace_feature(parent);

                toml.push_str(&format!("{feature} = [\"{dependency}\"]\n"));
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

    /// Records every namespace whose module would be empty in this package layout —
    /// its own types all emit nothing (a `windows-sys` namespace of pure COM
    /// interfaces) and every nested child is likewise prunable — into `prunable`.
    /// Returns whether `tree` itself is prunable so the recursion can fold children.
    fn collect_prunable(&self, tree: &TypeTree, prunable: &mut BTreeSet<&'static str>) -> bool {
        let config = self.with_namespace(tree.namespace);

        let self_empty = tree
            .types
            .iter()
            .all(|ty| ty.write(&config).into_string().trim().is_empty());

        // Recurse into every child (no short-circuit) so all prunable descendants are
        // recorded even when a sibling keeps the subtree.
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

/// The always-present umbrella modules that group the flat Win32/WDK header stems. They own no
/// types (their per-header children do) and so carry no Cargo feature and no feature gate.
fn is_flat_container(namespace: &str) -> bool {
    namespace == "Windows.Win32" || namespace == "Windows.Wdk"
}

/// Path to the standard-library item a flat Win32/WDK free constant would shadow if glob-re-exported
/// under its umbrella. Win32 metadata carries unscoped-enum values as free constants (e.g.
/// `Windows.Win32.ro` emits `pub const None: RoErrorReportingFlags`); flattening every header into a
/// single `Win32` glob would bring such a name into scope and shadow the Rust prelude for anyone
/// doing `use windows::Win32::*`. Emitting an explicit `pub use` of the prelude item shadows the glob
/// (explicit imports win over glob imports), restoring prelude behaviour and hiding the raw constant.
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
