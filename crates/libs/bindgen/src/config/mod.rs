mod cfg;
mod cpp_handle;
mod format;
mod names;
mod value;

use super::*;
pub use cfg::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Sys,
    Safe,
    SafeMinimal,
}

impl Mode {
    pub fn new(sys: bool, minimal: bool) -> Self {
        match (sys, minimal) {
            (true, false) => Self::Sys,
            (false, true) => Self::SafeMinimal,
            (false, false) => Self::Safe,
            (true, true) => unreachable!("checked by Bindgen before Config construction"),
        }
    }

    pub fn is_sys(self) -> bool {
        matches!(self, Self::Sys)
    }

    pub fn is_minimal(self) -> bool {
        matches!(self, Self::SafeMinimal)
    }

    pub fn is_sys_or_minimal(self) -> bool {
        !matches!(self, Self::Safe)
    }
}

#[derive(Clone)]
pub struct Config<'a> {
    pub reader: &'a Reader,
    pub types: &'a TypeMap,
    pub references: &'a References,
    pub filter: &'a Filter,
    pub output: &'a str,
    pub flat: bool,
    pub no_allow: bool,
    pub no_comment: bool,
    pub no_deps: bool,
    pub no_toml: bool,
    pub package: bool,
    pub rustfmt: &'a str,
    pub mode: Mode,
    pub typedef: bool,
    pub sys_fn_ptrs: bool,
    pub sys_fn_extern: bool,
    pub implement: bool,
    pub implements: &'a Implements,
    pub specific_deps: bool,
    pub derive: &'a Derive,
    pub link: &'a str,
    pub warnings: &'a WarningBuilder,
    pub namespace: Arc<str>,
}

impl Config<'_> {
    pub fn with_namespace(&self, namespace: &str) -> Self {
        let mut clone = self.clone();
        clone.namespace = Arc::from(namespace);
        clone
    }

    /// Returns the [`ImplementMode`] for the given type, combining:
    /// - the `--implement` / `--implements` options (whether `_Impl` should be
    ///   emitted at all), and
    /// - the `?Ns.Type` trait-only filter (whether the caller-side method
    ///   wrapper block should be suppressed).
    ///
    /// `default` is the fallback when neither `--implement` nor `--implements`
    /// constrains emission: `true` for types emitted unconditionally (COM/Win32
    /// interfaces) and `false` (or `!is_exclusive`) for WinRT interfaces.
    pub fn implement_mode(&self, name: &TypeName, default: bool) -> ImplementMode {
        let emit_impl = if self.implement {
            true
        } else if !self.implements.is_empty() {
            self.implements.matches(name)
        } else {
            default
        };

        if !emit_impl {
            ImplementMode::None
        } else if self.filter.is_trait_only(name) {
            ImplementMode::TraitOnly
        } else {
            ImplementMode::Full
        }
    }
}

impl<'a> Config<'a> {
    #[track_caller]
    pub fn write(&self, tree: TypeTree) {
        if self.package {
            self.write_package(&tree);
        } else {
            self.write_file(tree);
        }
    }

    #[track_caller]
    fn write_file(&self, tree: TypeTree) {
        let tokens = if self.flat {
            self.write_flat(tree)
        } else {
            self.write_modules(&tree)
        };

        write_to_file(self.output, self.format(&tokens.into_string()));
    }

    fn write_flat(&self, tree: TypeTree) -> TokenStream {
        let mut tokens = TokenStream::new();

        for ty in tree.flatten_types() {
            tokens.combine(ty.write(self));
        }

        tokens
    }

    fn write_modules(&self, tree: &TypeTree) -> TokenStream {
        let mut tokens = TokenStream::new();

        for ty in &tree.types {
            tokens.combine(ty.write(self));
        }

        for (name, tree) in &tree.nested {
            let name = to_ident(name);
            let nested = self.with_namespace(&tree.namespace).write_modules(tree);
            tokens.combine(quote! { pub mod #name { #nested } });
        }

        tokens
    }

    fn write_package(&self, tree: &TypeTree) {
        for name in tree.nested.keys() {
            _ = std::fs::remove_dir_all(format!("{}/src/{name}", self.output));
        }

        let trees = tree.flatten_trees();

        for_each(trees.iter(), |tree| {
            let directory = format!("{}/src/{}", self.output, tree.namespace.replace('.', "/"));

            let mut tokens = TokenStream::new();

            for (name, tree) in &tree.nested {
                let name = to_ident(name);
                let feature = tree.feature();

                tokens.combine(quote! {
                    #[cfg(feature = #feature)]
                    pub mod #name;
                });
            }

            let config = self.with_namespace(&tree.namespace);

            for ty in &tree.types {
                tokens.combine(ty.write(&config));
            }

            let output = format!("{directory}/mod.rs");
            write_to_file(&output, self.format(&tokens.into_string()));
        });

        if self.no_toml {
            return;
        }

        let toml_path = format!("{}/Cargo.toml", self.output);
        let mut toml = String::new();

        for line in read_file_lines(&toml_path) {
            toml.push_str(&line);
            toml.push('\n');

            if line == "# generated features" {
                break;
            }
        }

        for tree in trees.iter().skip(1) {
            let feature = tree.feature();

            if let Some(pos) = feature.rfind('_') {
                let dependency = &feature[..pos];

                toml.push_str(&format!("{feature} = [\"{dependency}\"]\n"));
            } else if namespace_starts_with(&tree.namespace, "Windows.Win32")
                || namespace_starts_with(&tree.namespace, "Windows.Wdk")
            {
                toml.push_str(&format!("{feature} = [\"Win32_Foundation\"]\n"));
            } else if &*tree.namespace != "Windows.Foundation" {
                toml.push_str(&format!("{feature} = [\"Foundation\"]\n"));
            } else {
                toml.push_str(&format!("{feature} = []\n"));
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
