use super::*;

pub fn gen_build() -> TokenStream {
    gen_build_redirect(true)
}

pub fn gen_build_legacy() -> TokenStream {
    gen_build_redirect(false)
}

pub fn gen_build_redirect(redirect: bool) -> TokenStream {
    let tokens = RawString(gen_source_tree(redirect).into_string());
    let target_dir = RawString(target_dir());
    let workspace_dir = RawString(workspace_dir());

    quote! {
        {
            // The following must be injected into the token stream because the `OUT_DIR` and `PROFILE`
            // environment variables are only set when the build script run and not when it is being compiled.

            use ::std::io::Write;
            let mut path = ::std::path::PathBuf::from(
                ::std::env::var("OUT_DIR").expect("No `OUT_DIR` env variable set"),
            );

            path.push("windows.rs");
            ::std::fs::write(&path, #tokens).expect("Could not write generated code to windows.rs");

            let mut cmd = ::std::process::Command::new("rustfmt");
            cmd.arg(&path);
            let _ = cmd.output();

            fn copy(source: &::std::path::Path, destination: &mut ::std::path::PathBuf) {
                if let ::core::result::Result::Ok(entries) = ::std::fs::read_dir(source) {
                    for entry in entries.filter_map(|entry| entry.ok()) {
                        let path = entry.path();
                        if let ::core::option::Option::Some(last_path_component) = path.file_name() {
                            let _ = ::std::fs::create_dir_all(&destination);
                            destination.push(last_path_component);
                            if path.is_file() {
                                let _ = ::std::fs::copy(path, &destination);
                            } else if path.is_dir() {
                                let _ = ::std::fs::create_dir(&destination);
                                copy(&path, destination);
                            }
                            destination.pop();
                        }
                    }
                }
            }
            fn copy_to_profile(source: &::std::path::Path, destination: &::std::path::Path, profile: &str) {
                if let ::core::result::Result::Ok(files) = ::std::fs::read_dir(destination) {
                    for file in files.filter_map(|file| file.ok())  {
                        let mut path = file.path();
                        if path.is_dir() {
                            if let ::core::option::Option::Some(filename) = path.file_name() {
                                if filename == profile {
                                    copy(source, &mut path);
                                } else {
                                    copy_to_profile(source, &path, profile);
                                }
                            }
                        }
                    }
                }
            }

            let mut source: ::std::path::PathBuf = #workspace_dir.into();
            source.push(".windows");

            if source.exists() {
                println!("cargo:rerun-if-changed={}", source.to_str().expect("`workspace_dir` not a UTF-8 string"));

                // The `target_arch` cfg is not set for build scripts so we need to sniff it out from the environment variable.
                source.push(match ::std::env::var("CARGO_CFG_TARGET_ARCH").expect("No `CARGO_CFG_TARGET_ARCH` env variable set").as_str() {
                    "x86_64" => "x64",
                    "x86" => "x86",
                    "arm" => "arm",
                    "aarch64" => "arm64",
                    unimplemented => unimplemented!("`{}` architecture set by `CARGO_CFG_TARGET_ARCH`", unimplemented),
                });

                if source.exists() {
                    println!("cargo:rustc-link-search=native={}", source.to_str().expect("`CARGO_MANIFEST_DIR` not a valid path"));
                }

                let mut destination: ::std::path::PathBuf = #target_dir.into();

                let profile = ::std::env::var("PROFILE").expect("No `PROFILE` env variable set");
                copy_to_profile(&source, &destination, &profile);
            }
        }
    }
}

struct RawString(String);

impl ToTokens for RawString {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.push_str("r#\"");
        tokens.push_str(&self.0);
        tokens.push_str("\"#");
    }
}

fn gen_source_tree(redirect: bool) -> TokenStream {
    let reader = TypeReader::get();

    namespace_iter(&reader.types, redirect).fold(TokenStream::new(), |mut accum, n| {
        accum.combine(&n);
        accum
    })
}

fn namespace_iter(tree: &TypeTree, redirect: bool) -> impl Iterator<Item = TokenStream> + '_ {
    let gen = Gen::build(tree.namespace, redirect);

    tree.types.iter().map(move |t| gen_type_entry(t.1, &gen)).chain(gen_namespaces(&tree.namespaces, redirect))
}

fn gen_namespaces<'a>(namespaces: &'a BTreeMap<&'static str, TypeTree>, redirect: bool) -> impl Iterator<Item = TokenStream> + 'a {
    namespaces.iter().map(move |(name, tree)| {
        if tree.include && !redirect || !tree.namespace.starts_with("Windows.") && tree.namespace != "Windows" {
            // TODO: https://github.com/microsoft/windows-rs/issues/212
            // TODO: https://github.com/microsoft/win32metadata/issues/380

            let allow = if name == &tree.namespace {
                quote! { #[allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)] }
            } else {
                quote! {}
            };

            let name = to_ident(name);
            let tokens = namespace_iter(tree, redirect);

            quote! {
                #allow
                pub mod #name {
                    #(#tokens)*
                }
            }
        } else {
            TokenStream::new()
        }
    })
}
