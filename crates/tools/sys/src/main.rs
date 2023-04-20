use metadata::reader::*;
use rayon::prelude::*;
use std::io::prelude::*;

fn main() {
    let time = std::time::Instant::now();
    let mut expect_namespace = false;
    let mut namespace = String::new();
    for arg in std::env::args() {
        match arg.as_str() {
            "-n" => expect_namespace = true,
            _ => {
                if expect_namespace {
                    namespace = arg;
                }
            }
        }
    }
    let mut output = std::path::PathBuf::from("crates/libs/sys/src/Windows");
    if namespace.is_empty() {
        _ = std::fs::remove_dir_all(&output);
    }
    output.pop();
    let files = File::with_default(&[]).unwrap();
    let reader = &Reader::new(&files);
    if !namespace.is_empty() {
        let tree = reader.tree(&namespace, &Default::default());
        gen_tree(reader, &output, &tree);
        return;
    }

    let root = reader.tree("Windows", &filter(reader));
    let trees = root.flatten();
    trees.par_iter().for_each(|tree| gen_tree(reader, &output, tree));
    output.pop();
    output.push("Cargo.toml");
    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"
[package]
name = "windows-sys"
version = "0.48.0"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
readme = "../../../docs/readme.md"
rust-version = "1.48"
categories = ["os::windows-apis"]

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = []
all-features = true

[dependencies.windows-targets]
version = "0.48.0"
path = "../targets"

[features]
default = []
"#
        .as_bytes(),
    )
    .unwrap();

    // Skip the root Windows tree while writing features
    for tree in trees.iter().skip(1) {
        let feature = tree.namespace[root.namespace.len() + 1..].replace('.', "_");

        if let Some(pos) = feature.rfind('_') {
            let dependency = &feature[..pos];

            file.write_all(format!("{feature} = [\"{dependency}\"]\n").as_bytes()).unwrap();
        } else {
            file.write_all(format!("{feature} = []\n").as_bytes()).unwrap();
        }
    }

    println!("  Finished in {:.2}s", time.elapsed().as_secs_f32());
}

fn gen_tree(reader: &Reader, output: &std::path::Path, tree: &Tree) {
    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();

    let mut gen = bindgen::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.sys = true;
    gen.doc = true;
    let tokens = bindgen::namespace(&gen, tree);
    std::fs::write(path.join("mod.rs"), tokens).unwrap();
}

fn filter<'a>(reader: &'a Reader) -> Filter<'a> {
    let mut exclude = vec![];

    for namespace in reader.namespaces() {
        // Find any COM interfaces in this namespace.
        let interfaces = reader.namespace_types(namespace, &Default::default()).any(|def| reader.type_def_kind(def) == TypeKind::Interface);

        // Find any interfaces-independent functions in this namespace.
        let functions = reader.namespace_functions(namespace).any(|function| {
            let cfg = reader.signature_cfg(&reader.method_def_signature(function, &[]));
            if cfg.types.values().flatten().any(|def| reader.type_def_kind(*def) == TypeKind::Interface) {
                return false;
            }
            if cfg.core_types.iter().any(|ty| matches!(ty, Type::IUnknown | Type::IInspectable)) {
                return false;
            }
            true
        });

        if interfaces && !functions {
            exclude.push(namespace);
        }
    }

    // Exclude additional namespace not covered by the hieuristic above.
    const EXCLUDE: [&str; 13] = ["Windows.Win32.Graphics.Direct2D", "Windows.Win32.Graphics.DirectComposition", "Windows.Win32.Graphics.DirectDraw", "Windows.Win32.Graphics.DirectWrite", "Windows.Win32.Graphics.DXCore", "Windows.Win32.Graphics.Dxgi", "Windows.Win32.Graphics.Imaging", "Windows.Win32.Foundation.Metadata", "Windows.Win32.Media.Audio.DirectSound", "Windows.Win32.Media.DirectShow", "Windows.Win32.Media.MediaFoundation", "Windows.Win32.System.WinRT", "Windows.Win32.UI.Xaml"];

    for namespace in &EXCLUDE {
        if exclude.contains(namespace) {
            panic!("already excluded `{}`", namespace);
        } else {
            exclude.push(namespace);
        }
    }

    Filter::new(&["Windows.Win32", "Windows.Wdk"], &exclude)
}
