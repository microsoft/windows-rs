use rayon::prelude::*;
use std::io::prelude::*;

fn main() {
    let mut rustfmt = true;
    let mut expect_namespace = false;
    let mut namespace = String::new();
    for arg in std::env::args() {
        match arg.as_str() {
            "-p" => rustfmt = false,
            "-n" => expect_namespace = true,
            _ => {
                if expect_namespace {
                    namespace = arg;
                }
            }
        }
    }
    let mut output = std::path::PathBuf::from("crates/libs/windows/src/Windows");
    if namespace.is_empty() {
        let _ = std::fs::remove_dir_all(&output);
    }
    output.pop();
    let files = vec![metadata::reader::File::new("crates/libs/metadata/default/Windows.winmd").unwrap(), metadata::reader::File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap(), metadata::reader::File::new("crates/libs/metadata/default/Windows.Win32.Interop.winmd").unwrap()];
    let reader = &metadata::reader::Reader::new(&files);
    if !namespace.is_empty() {
        let tree = reader.tree(&namespace, &[]).expect("Namespace not found");
        gen_tree(reader, &output, &tree, rustfmt);
        return;
    }
    let root = reader.tree("Windows", &lib::EXCLUDE_NAMESPACES).expect("`Windows` namespace not found");
    let trees = root.flatten();
    trees.par_iter().for_each(|tree| gen_tree(reader, &output, tree, rustfmt));
    output.pop();
    output.push("Cargo.toml");
    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"
[package]
name = "windows"
version = "0.43.0"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
documentation = "https://microsoft.github.io/windows-docs-rs/"
readme = "../../../docs/readme.md"
rust-version = "1.64"

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = []

[dependencies]
windows_targets = { path = "../targets",  version = "0.43.0" }

[dependencies]
windows-implement = { path = "../implement",  version = "0.43.0", optional = true }
windows-interface = { path = "../interface",  version = "0.43.0", optional = true }

[features]
default = []
deprecated = []
implement = ["windows-implement"]
interface = ["windows-interface"]
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

    file.write_all(
        r#"
# These features are unstable and require Rust nightly:
debugger_visualizer = []

"#
        .as_bytes(),
    )
    .unwrap();

    std::fs::copy("license-mit", "crates/libs/windows/license-mit").unwrap();
    std::fs::copy("license-apache-2.0", "crates/libs/windows/license-apache-2.0").unwrap();
}

fn gen_tree(reader: &metadata::reader::Reader, output: &std::path::Path, tree: &metadata::reader::Tree, rustfmt: bool) {
    println!("{}", tree.namespace);
    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();

    let mut gen = bindgen::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.cfg = true;
    gen.doc = true;
    let mut tokens = bindgen::namespace(&gen, tree);
    tokens.push_str(r#"#[cfg(feature = "implement")] ::core::include!("impl.rs");"#);
    lib::format(tree.namespace, &mut tokens, rustfmt);
    std::fs::write(path.join("mod.rs"), tokens).unwrap();
    let mut tokens = bindgen::namespace_impl(&gen, tree);
    lib::format(tree.namespace, &mut tokens, rustfmt);
    std::fs::write(path.join("impl.rs"), tokens).unwrap();
}
