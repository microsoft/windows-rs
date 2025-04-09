mod cfg;
mod cpp_handle;
mod format;
mod names;
mod value;

use super::*;
pub use cfg::*;
use rayon::prelude::*;

#[derive(Clone)]
pub struct Writer<'a> {
    pub config: &'a Config<'a>,
    namespace: &'static str,
}

impl<'a> std::ops::Deref for Writer<'a> {
    type Target = Config<'a>;

    fn deref(&self) -> &Self::Target {
        self.config
    }
}

impl<'a> Writer<'a> {
    pub fn new(config: &'a Config<'a>) -> Self {
        Self {
            config,
            namespace: "",
        }
    }

    fn with_namespace(&self, namespace: &'static str) -> Self {
        let mut clone = self.clone();
        clone.namespace = namespace;
        clone
    }

    #[track_caller]
    pub fn write(&self, tree: TypeTree) {
        if self.config.package {
            self.write_package(&tree);
        } else {
            self.write_file(tree);
        }
    }

    #[track_caller]
    fn write_file(&self, tree: TypeTree) {
        let tokens = if self.config.flat {
            self.write_flat(tree)
        } else {
            self.write_modules(&tree)
        };

        write_to_file(&self.config.output, self.format(&tokens.into_string()));
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
        for name in tree.nested.keys() {
            _ = std::fs::remove_dir_all(format!("{}/src/{name}", &self.config.output));
        }

        let trees = tree.flatten_trees();

        trees.par_iter().for_each(|tree| {
            let directory = format!(
                "{}/src/{}",
                &self.config.output,
                tree.namespace.replace('.', "/")
            );

            let mut tokens = TokenStream::new();

            for (name, tree) in &tree.nested {
                let name = to_ident(name);
                let feature = tree.feature();

                tokens.combine(quote! {
                    #[cfg(feature = #feature)]
                    pub mod #name;
                });
            }

            let writer = self.with_namespace(tree.namespace);

            for ty in &tree.types {
                tokens.combine(ty.write(&writer));
            }

            let output = format!("{directory}/mod.rs");
            write_to_file(&output, self.format(&tokens.into_string()));
        });

        if self.config.no_toml {
            return;
        }

        let toml_path = format!("{}/Cargo.toml", &self.config.output);
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
