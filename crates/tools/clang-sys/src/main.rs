use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use std::path::Path;
use syn::parse::{Parse, ParseStream};
use windows_clang::{Input, Target, ValueKind, capture, merge, projection};

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--native") {
        let status = std::process::Command::new("powershell.exe")
            .args([
                "-NoProfile",
                "-File",
                "crates\\libs\\clang-sys\\native\\build.ps1",
            ])
            .status()
            .unwrap();
        assert!(status.success(), "native build failed");
        return;
    }
    helpers::ensure_libclang();
    helpers::assert_libclang_version();
    let triple = if cfg!(target_arch = "aarch64") {
        "aarch64-pc-windows-msvc"
    } else {
        "x86_64-pc-windows-msvc"
    };
    let sdk = Path::new("target\\tool-clang\\native")
        .join(helpers::LIBCLANG_VERSION)
        .join(triple)
        .join("llvm");
    let include = std::path::absolute(sdk.join("include")).unwrap();
    let scratch = Path::new("target\\tool-clang\\bindings");
    std::fs::create_dir_all(scratch).unwrap();
    let rdl = scratch.join("bindings.rdl");
    let winmd = scratch.join("bindings.winmd");
    let raw = scratch.join("raw.rs");
    let args = [
        "-x".to_string(),
        "c++".into(),
        "-std=c++20".into(),
        "-I".into(),
        include.display().to_string(),
        "-I".into(),
        std::path::absolute("crates\\libs\\clang-sys\\native")
            .unwrap()
            .display()
            .to_string(),
    ];
    let source = "#include <clang-c/Index.h>\n#include \"extension.h\"\n";
    let input = Input {
        name: "bindings.h".into(),
        source: source.into(),
        arguments: args.into(),
        root_paths: Some(vec![
            include.join("clang-c"),
            std::path::absolute("crates\\libs\\clang-sys\\native\\extension.h").unwrap(),
        ]),
        macros: [
            "CINDEX_VERSION",
            "CINDEX_VERSION_MAJOR",
            "CINDEX_VERSION_MINOR",
        ]
        .map(String::from)
        .into(),
        ..Default::default()
    };
    let snapshots = capture(&[input], &[Target::X64, Target::Arm64, Target::X86]).unwrap();
    let native = &snapshots[0];
    let mut layouts = BTreeMap::new();
    for record in &native.records {
        if let Some(definition) = record
            .observations
            .iter()
            .find_map(|id| native.observations[id.0].definition.as_ref())
        {
            layouts.insert(
                record.name.clone(),
                (
                    definition.layout.size as usize,
                    definition.layout.alignment as usize,
                    &definition.fields,
                ),
            );
        }
    }
    let imports = snapshots
        .iter()
        .flat_map(|snapshot| {
            snapshot.values.iter().filter_map(|value| {
                let observation = &value.observations[0];
                matches!(observation.kind, ValueKind::Function { .. }).then(|| {
                    let extension = Path::new(&snapshot.files[observation.location.file.0])
                        .file_name()
                        .is_some_and(|name| name == "extension.h");
                    (
                        value.name.clone(),
                        projection::Import {
                            library: if extension {
                                "windows_clang.dll"
                            } else {
                                "libclang.dll"
                            }
                            .into(),
                            name: None,
                        },
                    )
                })
            })
        })
        .collect();
    let output =
        projection::project_with_imports(&merge(&snapshots).unwrap(), "Clang", &imports).unwrap();
    std::fs::write(
        &rdl,
        output
            .partitions
            .values()
            .map(|part| part.rdl.as_str())
            .collect::<String>(),
    )
    .unwrap();
    windows_rdl::reader()
        .input("metadata\\metadata.rdl")
        .input(&rdl)
        .output(&winmd)
        .write()
        .unwrap();
    windows_bindgen::builder()
        .input(&winmd)
        .filter("Clang")
        .sys()
        .flat()
        .output(&raw)
        .write();
    let parsed = syn::parse_file(&std::fs::read_to_string(raw).unwrap()).unwrap();
    let mut types = TokenStream::new();
    let mut fields = TokenStream::new();
    let mut loads = TokenStream::new();
    let mut wrappers = TokenStream::new();
    let mut assertions = TokenStream::new();
    let mut count = 0;
    for item in parsed.items {
        if let syn::Item::Struct(record) = &item {
            let name = &record.ident;
            if let Some((size, align, native_fields)) = layouts.get(&name.to_string()) {
                assertions.extend(quote! {
                    assert!(size_of::<#name>() == #size);
                    assert!(align_of::<#name>() == #align);
                });
                for field in &record.fields {
                    if let Some(field_name) = &field.ident
                        && let Some(native) = native_fields
                            .iter()
                            .find(|native| *field_name == native.name && native.bit_width.is_none())
                    {
                        assert_eq!(native.offset_bits % 8, 0);
                        let offset = native.offset_bits as usize / 8;
                        assertions.extend(quote! {
                            assert!(core::mem::offset_of!(#name, #field_name) == #offset);
                        });
                    }
                }
            }
        }
        let syn::Item::Macro(item) = &item else {
            types.extend(quote! { #item });
            continue;
        };
        assert_eq!(item.mac.path.segments.last().unwrap().ident, "link");
        let Binding {
            library,
            abi,
            signature,
        } = syn::parse2(item.mac.tokens.clone()).unwrap();
        let name = &signature.ident;
        let parameters = &signature.inputs;
        let output = &signature.output;
        let arguments: Vec<_> = parameters
            .iter()
            .map(|parameter| {
                let syn::FnArg::Typed(parameter) = parameter else {
                    panic!("unexpected receiver")
                };
                &parameter.pat
            })
            .collect();
        let module = if library.value() == "libclang.dll" {
            quote! { base }
        } else {
            assert_eq!(library.value(), "windows_clang.dll");
            quote! { extension }
        };
        let symbol = syn::LitByteStr::new(format!("{name}\0").as_bytes(), name.span());
        fields.extend(quote! { pub #name: unsafe extern #abi fn(#parameters) #output, });
        loads.extend(quote! {
            #name: unsafe { *#module.get(#symbol).map_err(|error| format!("{}: {error}", stringify!(#name)))? },
        });
        let body = if matches!(output, syn::ReturnType::Default) {
            quote! {
                unsafe { (crate::functions().#name)(#(#arguments),*); }
            }
        } else {
            quote! {
                unsafe { (crate::functions().#name)(#(#arguments),*) }
            }
        };
        wrappers.extend(quote! { pub unsafe fn #name(#parameters) #output { #body } });
        count += 1;
    }
    let generated = quote! {
        #types
        #[cfg(target_pointer_width = "64")]
        const _: () = { #assertions };
        pub(crate) struct Functions { #fields }
        impl Functions {
            pub(crate) unsafe fn load(base: &libloading::Library, extension: &libloading::Library)
                -> Result<Self, String> {
                Ok(Self { #loads })
            }
        }
        #wrappers
    };
    std::fs::write(
        "crates\\libs\\clang-sys\\src\\bindings.rs",
        format!("// Generated by tool-clang-sys. Do not edit.\n{generated}\n"),
    )
    .unwrap();
    assert!(
        std::process::Command::new("rustfmt")
            .args([
                "--edition",
                "2024",
                "crates\\libs\\clang-sys\\src\\bindings.rs"
            ])
            .status()
            .unwrap()
            .success()
    );
    println!("Generated {count} dynamically loaded functions");
}

struct Binding {
    library: syn::LitStr,
    abi: syn::LitStr,
    signature: syn::Signature,
}

impl Parse for Binding {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            library: input.parse()?,
            abi: input.parse()?,
            signature: input.parse()?,
        })
    }
}
