mod cfg;
mod cpp_enum;
mod cpp_fn;
mod cpp_struct;
mod r#enum;
mod format;
mod literals;
mod names;
mod r#struct;

use super::*;
use rayon::prelude::*;

#[derive(Clone)]
pub struct Writer {
    pub reader: &'static Reader,
    pub tree: &'static NameTree,
    pub output: String,
    pub namespace: &'static str,
    pub flat: bool,
    pub minimal: bool, // TODO: if minimal then don't include dependencies for method parameters.
    pub no_allow: bool,
    pub no_comment: bool,
    pub package: bool,
    pub rustfmt: String,
    pub sys: bool, // TODO: if sys and not package then include minimal "vtbl" definitions
}

impl Writer {
    fn with_namespace(&self, namespace: &'static str) -> Self {
        let mut clone = self.clone();
        clone.namespace = namespace;
        clone
    }

    pub fn write(&self, tree: &ItemTree) {
        if self.package {
            self.write_package(tree);
        } else {
            self.write_file(tree);
        }
    }

    fn write_file(&self, tree: &ItemTree) {
        let mut tokens = if self.flat {
            self.write_flat(tree)
        } else {
            self.write_modules(tree)
        };

        if self.sys {
            for dependency in &self.tree.items {
                match *dependency {
                    "HRESULT" => tokens.combine(quote! { pub type HRESULT = i32; }),
                    rest => panic!("windows-bindgen: {rest:?}"),
                }
            }
        }

        write_to_file(&self.output, self.format(&tokens.into_string()));
    }

    fn write_flat(&self, tree: &ItemTree) -> TokenStream {
        let mut tokens = TokenStream::new();

        for item in &tree.items {
            tokens.combine(self.write_item(item));
        }

        for tree in tree.nested.values() {
            tokens.combine(self.write_flat(tree));
        }

        tokens
    }

    fn write_modules(&self, tree: &ItemTree) -> TokenStream {
        let mut tokens = TokenStream::new();

        for item in &tree.items {
            tokens.combine(self.write_item(item));
        }

        for (name, tree) in &tree.nested {
            let name = to_ident(name);
            let nested = self.with_namespace(tree.namespace).write_modules(tree);
            tokens.combine(quote! { pub mod #name { #nested } });
        }

        tokens
    }

    fn write_package(&self, tree: &ItemTree) {
        for name in tree.nested.keys() {
            _ = std::fs::remove_dir_all(format!("{}/src/{name}", &self.output));
        }

        let trees = tree.flatten();

        trees.par_iter().for_each(|tree| {
            let directory = format!("{}/src/{}", &self.output, tree.namespace.replace('.', "/"));

            let mut tokens = TokenStream::new();

            for name in tree.nested.keys() {
                let name = to_ident(name);

                tokens.combine(quote! {
                    pub mod #name;
                });
            }

            let writer = self.with_namespace(tree.namespace);

            for item in &tree.items {
                tokens.combine(writer.write_item(item));
            }

            let output = format!("{directory}/mod.rs");
            write_to_file(&output, self.format(&tokens.into_string()));
        });

        let toml_path = format!("{}/Cargo.toml", &self.output);
        let mut toml = String::new();

        for line in read_file_lines(&toml_path) {
            toml.push_str(&line);
            toml.push('\n');

            if line == "# generated features" {
                break;
            }
        }

        for tree in trees.iter().skip(1) {
            let feature = tree.namespace.split_once('.').unwrap().1.replace('.', "_");

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

    fn write_item(&self, item: &'static Item) -> TokenStream {
        match item {
            Item::Struct(def) => self.write_struct(def),
            Item::Enum(def) => self.write_enum(def),
            Item::CppStruct(def) => self.write_cpp_struct(def),
            Item::CppEnum(def) => self.write_cpp_enum(def),
            Item::CppFn(def) => self.write_cpp_fn(def),
            _ => quote! {},
        }
    }
}
