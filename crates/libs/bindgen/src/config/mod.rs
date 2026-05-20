mod cfg;
mod cpp_handle;
mod format;
mod names;
mod value;

use super::*;
pub use cfg::*;

#[derive(Clone)]
pub struct Config<'a> {
    pub bindgen: &'a Bindgen,
    pub reader: &'a Reader,
    pub types: &'a TypeMap,
    pub references: &'a References,
    pub filter: &'a Filter,
    pub implement: Option<&'a Implements>,
    pub derive: &'a Derive,
    pub link: &'a str,
    pub warnings: &'a WarningBuilder,
    pub namespace: &'static str,
}

impl Config<'_> {
    pub fn with_namespace(&self, namespace: &'static str) -> Self {
        let mut clone = self.clone();
        clone.namespace = namespace;
        clone
    }

    /// Returns `true` if the `_Impl` scaffolding for the given type should be
    /// emitted, based on the `--implement` option.
    ///
    /// `default` is the behavior to fall back on when `--implement` is not
    /// set: `true` for types that are emitted unconditionally today (such
    /// as COM/Win32 interfaces) and `false` (or some other condition such as
    /// `!is_exclusive`) for WinRT interfaces.
    pub fn should_implement(&self, name: TypeName, default: bool) -> bool {
        match self.implement {
            None => default,
            Some(implements) if implements.is_empty() => true,
            Some(implements) => implements.matches(name),
        }
    }
}

impl<'a> Config<'a> {
    #[track_caller]
    pub fn write(&self, tree: TypeTree) {
        if self.bindgen.layout.is_package() {
            self.write_package(&tree);
        } else {
            self.write_file(tree);
        }
    }

    #[track_caller]
    fn write_file(&self, tree: TypeTree) {
        let tokens = if self.bindgen.layout.is_flat() {
            self.write_flat(tree)
        } else {
            self.write_modules(&tree)
        };

        write_to_file(&self.bindgen.output, self.format(&tokens.into_string()));
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
            let nested = self.with_namespace(tree.namespace).write_modules(tree);
            tokens.combine(quote! { pub mod #name { #nested } });
        }

        tokens
    }

    fn write_package(&self, tree: &TypeTree) {
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

        if self.bindgen.layout.no_toml() {
            return;
        }

        let toml_path = format!("{output}/Cargo.toml");
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
            } else if namespace_starts_with(tree.namespace, "Windows.Win32")
                || namespace_starts_with(tree.namespace, "Windows.Wdk")
            {
                toml.push_str(&format!("{feature} = [\"Win32_Foundation\"]\n"));
            } else if tree.namespace != "Windows.Foundation" {
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
