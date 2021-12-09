use rayon::prelude::*;
use std::io::prelude::*;

fn main() {
    let start = std::time::Instant::now();
    let mut output = std::path::PathBuf::from(reader::workspace_dir());
    output.push("crates/deps/sys/src/Windows");
    let _ = std::fs::remove_dir_all(&output);
    output.pop();

    let reader = reader::TypeReader::get_mut();
    let root = reader.types.get_namespace("Windows").unwrap();

    let mut trees = Vec::new();
    collect_trees(&output, root.namespace, root, &mut trees);
    trees.par_iter().for_each(|tree| gen_tree(&output, root.namespace, tree));

    output.pop();
    output.push("Cargo.toml");

    write_toml(&output, root);
    println!("Elapsed: {} ms", start.elapsed().as_millis());
}

fn write_toml(output: &std::path::Path, tree: &reader::TypeTree) {
    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"
[package]
name = "windows-sys"
version = "0.28.0"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
readme = "../../../.github/readme.md"

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
all-features = true

[target.i686-pc-windows-msvc.dependencies]
windows_i686_msvc = { path = "../../targets/i686_msvc", version = "0.28.0" }

[target.x86_64-pc-windows-msvc.dependencies]
windows_x86_64_msvc = { path = "../../targets/x86_64_msvc", version = "0.28.0" }

[target.aarch64-pc-windows-msvc.dependencies]
windows_aarch64_msvc = { path = "../../targets/aarch64_msvc", version = "0.28.0" }

[target.i686-pc-windows-gnu.dependencies]
windows_i686_gnu = { path = "../../targets/i686_gnu", version = "0.28.0" }

[target.x86_64-pc-windows-gnu.dependencies]
windows_x86_64_gnu = { path = "../../targets/x86_64_gnu", version = "0.28.0" }

[features]
default = []
"#
        .as_bytes(),
    )
    .unwrap();

    write_features(&mut file, tree.namespace, tree);
}

fn write_features(file: &mut std::fs::File, root: &'static str, tree: &reader::TypeTree) {
    for tree in tree.namespaces.values() {
        if tree.namespace == "Windows.Win32.Interop" {
            continue;
        }

        write_feature(file, root, tree);
        write_features(file, root, tree);
    }
}

fn write_feature(file: &mut std::fs::File, root: &'static str, tree: &reader::TypeTree) {
    let feature = tree.namespace[root.len() + 1..].replace('.', "_");

    if let Some(pos) = feature.rfind('_') {
        let dependency = &feature[..pos];

        file.write_all(format!("{} = [\"{}\"]\n", feature, dependency).as_bytes()).unwrap();
    } else {
        file.write_all(format!("{} = []\n", feature).as_bytes()).unwrap();
    }
}

fn collect_trees<'a>(output: &std::path::Path, root: &'static str, tree: &'a reader::TypeTree, trees: &mut Vec<&'a reader::TypeTree>) {
    trees.push(tree);
    tree.namespaces.values().for_each(|tree| collect_trees(output, root, tree, trees));
    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();
}

fn gen_tree(output: &std::path::Path, _root: &'static str, tree: &reader::TypeTree) {
    if tree.namespace == "Windows.Win32.Interop" {
        return;
    }

    let mut path = std::path::PathBuf::from(output);

    path.push(tree.namespace.replace('.', "/"));
    path.push("mod.rs");

    let gen = bindgen::Gen { namespace: tree.namespace, sys: true, cfg: true, ..Default::default() };
    let mut tokens = bindgen::gen_namespace(&gen);

    let mut child = std::process::Command::new("rustfmt").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::null()).spawn().expect("Failed to spawn `rustfmt`");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(tokens.as_bytes()).unwrap();
    drop(stdin);
    let output = child.wait_with_output().unwrap();

    if output.status.success() {
        println!("{}", tree.namespace);
        tokens = String::from_utf8(output.stdout).expect("Failed to parse UTF-8");
    } else {
        println!("** {} - rustfmt failed", tree.namespace);
    }

    std::fs::write(&path, tokens).unwrap();
}
