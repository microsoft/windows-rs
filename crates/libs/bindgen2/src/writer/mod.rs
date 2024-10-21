mod cfg;
mod cpp_handle;
mod format;
mod names;
mod value;

use super::*;
use rayon::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum InterfaceKind {
    None,
    Default,
    Static,
    Composable,
    Base,
}

#[derive(Clone)]
pub struct Writer {
    pub config: &'static Config,
    pub namespace: &'static str,
}

impl Writer {
    fn with_namespace(&self, namespace: &'static str) -> Self {
        let mut clone = self.clone();
        clone.namespace = namespace;
        clone
    }

    pub fn write(&self, tree: ItemTree) {
        if self.config.package {
            self.write_package(&tree);
        } else {
            self.write_file(tree);
        }
    }

    fn write_file(&self, tree: ItemTree) {
        let mut tokens = if self.config.flat {
            self.write_flat(tree)
        } else {
            self.write_modules(&tree)
        };

        // TODO: this is why it would be handy to have pseudo types for these in Item so we can write them more generically
        // TODO: should provide non-sys versions of these as well for no-deps builds?
        if self.config.no_deps {
            for dependency in &self.config.tree.items {
                tokens.combine(match *dependency {
                    "HRESULT" => quote! { pub type HRESULT = i32; },
                    "PWSTR" => quote! { pub type PWSTR = *mut u16; },
                    "PCSTR" => quote! { pub type PCSTR = *const u8; },
                    "PSTR" => quote! { pub type PSTR = *mut u8; },
                    "PCWSTR" => quote! { pub type PCWSTR = *const u16; },
                    "String" => quote! { pub type HSTRING = *mut core::ffi::c_void; },
                    "GUID" => quote! { 
                        #[repr(C)]
                        #[derive(Clone, Copy)]
                        pub struct GUID {
                            pub data1: u32,
                            pub data2: u16,
                            pub data3: u16,
                            pub data4: [u8; 8],
                        }
                        impl GUID {
                            pub const fn from_u128(uuid: u128) -> Self {
                                Self { data1: (uuid >> 96) as u32, data2: (uuid >> 80 & 0xffff) as u16, data3: (uuid >> 64 & 0xffff) as u16, data4: (uuid as u64).to_be_bytes() }
                            }
                        }
                    },
                    rest => panic!("windows-bindgen: {rest:?}"),
                });
            }
        }

        write_to_file(&self.config.output, self.format(&tokens.into_string()));
    }

    fn write_flat(&self, tree: ItemTree) -> TokenStream {
        let mut tokens = TokenStream::new();

        for item in tree.flatten_items() {
            tokens.combine(item.write(self));
        }

        tokens
    }

    fn write_modules(&self, tree: &ItemTree) -> TokenStream {
        let mut tokens = TokenStream::new();

        for item in &tree.items {
            tokens.combine(item.write(self));
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

            for item in &tree.items {
                tokens.combine(item.write(&writer));
            }

            let output = format!("{directory}/mod.rs");
            write_to_file(&output, self.format(&tokens.into_string()));
        });

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
