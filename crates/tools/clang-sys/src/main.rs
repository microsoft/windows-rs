use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use std::path::Path;
use syn::parse::{Parse, ParseStream};

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
    let base = scratch.join("clang.rdl");
    let base_winmd = scratch.join("clang.winmd");
    let extra = scratch.join("extension.rdl");
    let callbacks = scratch.join("callbacks.rdl");
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
        "--target=x86_64-pc-windows-msvc".into(),
    ];
    let headers: Vec<_> = std::fs::read_dir(include.join("clang-c"))
        .unwrap()
        .map(|header| {
            header
                .unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect();
    let source = "#include <clang-c/Index.h>\n#include \"extension.h\"\n";
    let native = windows_clang::extract(
        [windows_clang::Input::new("bindings.h", source)
            .with_root_suffixes(headers.iter().cloned().chain(["extension.h".into()]))],
        &args.iter().map(String::as_str).collect::<Vec<_>>(),
    )
    .unwrap();
    let mut layouts = BTreeMap::new();
    let mut callback_fields = BTreeMap::new();
    let mut callback_source = source.to_string();
    for fact in native.facts() {
        if let windows_clang::FactData::Record {
            size,
            align,
            fields,
            ..
        } = &fact.data
            && *size >= 0
            && *align > 0
        {
            layouts.insert(fact.name.clone(), (*size as usize, *align as usize, fields));
            for field in fields {
                if matches!(field.ty, windows_clang::TypeRef::FunctionPointer { .. }) {
                    let name = format!("{}_{}", fact.name, field.name);
                    callback_source.push_str(&format!(
                        "typedef decltype((({}*)nullptr)->{}) {name};\n",
                        fact.name, field.name
                    ));
                    callback_fields.insert((fact.name.clone(), field.name.clone()), name);
                }
            }
        }
    }
    let mut generator = windows_clang::clang();
    generator
        .input(include.join("clang-c\\Index.h"))
        .namespace("Clang")
        .library("libclang.dll")
        .args(&args)
        .output(&base);
    for header in &headers {
        generator.filter(header);
    }
    generator.write().unwrap();
    windows_rdl::reader()
        .input("metadata\\metadata.rdl")
        .input(&base)
        .output(&base_winmd)
        .write()
        .unwrap();
    windows_clang::clang()
        .input_text(callback_source)
        .filter("windows-clang-0.h")
        .reference(&base_winmd)
        .namespace("Clang")
        .args(&args)
        .output(&callbacks)
        .write()
        .unwrap();
    windows_clang::clang()
        .input("crates\\libs\\clang-sys\\native\\extension.h")
        .filter("extension.h")
        .reference(&base_winmd)
        .namespace("Clang")
        .library("libClangSharp.dll")
        .args(&args)
        .output(&extra)
        .write()
        .unwrap();
    windows_rdl::reader()
        .input("metadata\\metadata.rdl")
        .input(&base)
        .input(&extra)
        .input(&callbacks)
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
    for mut item in parsed.items {
        if let syn::Item::Struct(record) = &mut item {
            let name = &record.ident;
            if let Some((size, align, native_fields)) = layouts.get(&name.to_string()) {
                assertions.extend(quote! {
                    assert!(size_of::<#name>() == #size);
                    assert!(align_of::<#name>() == #align);
                });
                for field in &mut record.fields {
                    if let Some(field_name) = &field.ident {
                        if let Some(callback) =
                            callback_fields.get(&(name.to_string(), field_name.to_string()))
                        {
                            let callback = syn::Ident::new(callback, field_name.span());
                            field.ty = syn::parse_quote! { #callback };
                        }
                        if let Some(native) = native_fields
                            .iter()
                            .find(|native| *field_name == native.name && native.bit_width.is_none())
                        {
                            assert!(native.offset >= 0 && native.offset % 8 == 0);
                            let offset = native.offset as usize / 8;
                            assertions.extend(quote! {
                                assert!(core::mem::offset_of!(#name, #field_name) == #offset);
                            });
                        }
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
            assert_eq!(library.value(), "libclangsharp.dll");
            quote! { extension }
        };
        let symbol = syn::LitByteStr::new(format!("{name}\0").as_bytes(), name.span());
        fields.extend(quote! { pub #name: unsafe extern #abi fn(#parameters) #output, });
        loads.extend(quote! {
            #name: unsafe { *#module.get(#symbol).map_err(|error| format!("{}: {error}", stringify!(#name)))? },
        });
        let body = if matches!(output, syn::ReturnType::Default) {
            quote! {
                crate::with_library(|library| unsafe { (library.functions.#name)(#(#arguments),*); });
            }
        } else {
            quote! {
                crate::with_library(|library| unsafe { (library.functions.#name)(#(#arguments),*) })
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
