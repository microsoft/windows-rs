use rayon::prelude::*;
use std::io::prelude::*;

fn main() {
    let mut output = std::path::PathBuf::from("crates/libs/windows/src/Windows");
    let _ = std::fs::remove_dir_all(&output);
    output.pop();

    let files = vec![
        metadata::reader2::File::new("crates/libs/metadata/default/Windows.winmd").unwrap(), 
        metadata::reader2::File::new("crates/libs/metadata/default/Windows.Win32.winmd").unwrap(),
        metadata::reader2::File::new("crates/libs/metadata/default/Windows.Win32.Interop.winmd").unwrap(),
    ];
    let reader = &metadata::reader2::Reader::new(&files);
    let root = &reader.tree().nested["Windows"];

    let trees = root.flatten();
    trees.par_iter().for_each(|tree| gen_tree(reader, &output, tree));

    output.pop();
    output.push("Cargo.toml");

    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"
[package]
name = "windows"
version = "0.37.0"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
documentation = "https://microsoft.github.io/windows-docs-rs/"
readme = "../../../.github/readme.md"
rust-version = "1.59"

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = []

[target.i686-pc-windows-msvc.dependencies]
windows_i686_msvc = { path = "../../targets/i686_msvc", version = "0.37.0" }

[target.i686-uwp-windows-msvc.dependencies]
windows_i686_msvc = { path = "../../targets/i686_msvc", version = "0.37.0" }

[target.x86_64-pc-windows-msvc.dependencies]
windows_x86_64_msvc = { path = "../../targets/x86_64_msvc", version = "0.37.0" }

[target.x86_64-uwp-windows-msvc.dependencies]
windows_x86_64_msvc = { path = "../../targets/x86_64_msvc", version = "0.37.0" }

[target.aarch64-pc-windows-msvc.dependencies]
windows_aarch64_msvc = { path = "../../targets/aarch64_msvc", version = "0.37.0" }

[target.aarch64-uwp-windows-msvc.dependencies]
windows_aarch64_msvc = { path = "../../targets/aarch64_msvc", version = "0.37.0" }

[target.i686-pc-windows-gnu.dependencies]
windows_i686_gnu = { path = "../../targets/i686_gnu", version = "0.37.0" }

[target.i686-uwp-windows-gnu.dependencies]
windows_i686_gnu = { path = "../../targets/i686_gnu", version = "0.37.0" }

[target.x86_64-pc-windows-gnu.dependencies]
windows_x86_64_gnu = { path = "../../targets/x86_64_gnu", version = "0.37.0" }

[target.x86_64-uwp-windows-gnu.dependencies]
windows_x86_64_gnu = { path = "../../targets/x86_64_gnu", version = "0.37.0" }

[dependencies]
windows-implement = { path = "../implement",  version = "0.37.0", optional = true }
windows-interface = { path = "../interface",  version = "0.37.0", optional = true }

[features]
default = []
deprecated = []
alloc = []
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

            file.write_all(format!("{} = [\"{}\"]\n", feature, dependency).as_bytes()).unwrap();
        } else {
            file.write_all(format!("{} = []\n", feature).as_bytes()).unwrap();
        }
    }

    std::fs::copy(".github/license-mit", "crates/libs/windows/license-mit").unwrap();
    std::fs::copy(".github/license-apache", "crates/libs/windows/license-apache").unwrap();
}

fn gen_tree(reader: &metadata::reader2::Reader, output: &std::path::Path, tree: &metadata::reader2::Tree) {
    println!("{}", tree.namespace);
    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();

    let mut gen = bindgen::bindgen2::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.cfg = true;
    gen.doc = true;
    gen.min_xaml = true;
    let mut tokens = bindgen::bindgen2::namespace(&gen, tree);
    tokens.push_str(r#"#[cfg(feature = "implement")] ::core::include!("impl.rs");"#);
    fmt_tokens(tree.namespace, &mut tokens);
    std::fs::write(path.join("mod.rs"), tokens).unwrap();

    let mut tokens = bindgen::bindgen2::namespace_impl(&gen, tree);
    fmt_tokens(tree.namespace, &mut tokens);
    std::fs::write(path.join("impl.rs"), tokens).unwrap();

}

fn fmt_tokens(namespace: &str, tokens: &mut String) {
    let mut child = std::process::Command::new("rustfmt").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::null()).spawn().expect("Failed to spawn `rustfmt`");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(tokens.as_bytes()).unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();

    if output.status.success() {
        *tokens = String::from_utf8(output.stdout).expect("Failed to parse UTF-8");
    } else {
        println!("** {} - rustfmt failed", namespace);
    }
}
