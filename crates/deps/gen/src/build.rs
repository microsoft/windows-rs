use super::*;

pub fn gen_build() -> TokenStream {
    let tokens = RawString(gen_source_tree().into_string());
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
