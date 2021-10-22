use rayon::prelude::*;
use std::io::prelude::*;

fn main() {
    let start = std::time::Instant::now();
    let mut output = std::path::PathBuf::from(reader::workspace_dir());
    output.push("src\\Windows");
    let _ = std::fs::remove_dir_all(&output);
    output.pop();

    let reader = reader::TypeReader::get_mut();
    include_all(&mut reader.types);

    let root = reader.types.get_namespace("Windows").unwrap();

    let mut trees = Vec::new();
    collect_trees(&output, root.namespace, root, &mut trees);
    trees
        .par_iter()
        .for_each(|tree| gen_tree(&output, root.namespace, tree));

    output.pop();
    output.push("Cargo.toml");

    write_toml(&output, root);
    println!("Elapsed: {} ms", start.elapsed().as_millis());
    let start = std::time::Instant::now();

    // rustfmt doesn't work reliably in parallel so have to run cargo fmt at the end, very slowly...
    println!("\ncargo fmt...");
    let mut cmd = ::std::process::Command::new("cargo");
    output.pop();
    cmd.current_dir(output);
    cmd.arg("fmt");
    cmd.output().unwrap();

    println!("Elapsed: {} ms", start.elapsed().as_millis());
}

fn write_toml(output: &std::path::Path, tree: &reader::TypeTree) {
    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"[package]
name = "windows"
version = "0.21.1"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
documentation = "https://docs.rs/windows"
readme = ".github/readme.md"
exclude = [".github", ".windows", "docs", "tests"]

[workspace]
members = ["crates/*", "tests/legacy/*", "tests/metadata/*", "tests/winrt/*", "tests/win32/*"]
exclude = ["tests/component"]

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = ["x86_64-pc-windows-msvc"]

[dependencies]
windows_macros = { path = "crates/macros",  version = "0.21.1", optional = true }
windows_reader = { path = "crates/reader", version = "0.21.1", optional = true }
windows_gen = { path = "crates/gen",  version = "0.21.1", optional = true }

[target.i686-pc-windows-msvc.dependencies]
windows_x86 = { path = "crates/x86", version = "0.21.1" }

[target.x86_64-pc-windows-msvc.dependencies]
windows_x64 = { path = "crates/x64", version = "0.21.1" }

# TODO: add deprecated feature
[features]
default = []
build = ["windows_gen", "windows_macros", "windows_reader"]
"#
        .as_bytes(),
    )
    .unwrap();

    write_features(&mut file, tree.namespace, tree);
}

fn write_features(file: &mut std::fs::File, root: &'static str, tree: &reader::TypeTree) {
    for tree in tree.namespaces.values() {
        write_feature(file, root, tree);
        write_features(file, root, tree);
    }
}

fn write_feature(file: &mut std::fs::File, root: &'static str, tree: &reader::TypeTree) {
    let feature = tree.namespace[root.len() + 1..].replace('.', "_");

    if let Some(pos) = feature.rfind('_') {
        let dependency = &feature[..pos];

        file.write_all(format!("{} = [\"{}\"]\n", feature, dependency).as_bytes())
            .unwrap();
    } else {
        file.write_all(format!("{} = []\n", feature).as_bytes())
            .unwrap();
    }
}

fn include_all(tree: &mut reader::TypeTree) {
    tree.include = true;

    tree.types
        .values_mut()
        .for_each(|entry| entry.include = reader::TypeInclude::Full);

    tree.namespaces.values_mut().for_each(include_all);
}

fn collect_trees<'a>(
    output: &std::path::Path,
    root: &'static str,
    tree: &'a reader::TypeTree,
    trees: &mut Vec<&'a reader::TypeTree>,
) {
    trees.push(tree);

    tree.namespaces
        .values()
        .for_each(|tree| collect_trees(output, root, tree, trees));

    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "\\"));
    std::fs::create_dir_all(&path).unwrap();
}

fn gen_tree(output: &std::path::Path, root: &'static str, tree: &reader::TypeTree) {
    println!("{}", tree.namespace);
    let mut path = std::path::PathBuf::from(output);

    path.push(tree.namespace.replace('.', "\\"));
    path.push("mod.rs");

    let tokens = gen::gen_source_file(root, tree);
    std::fs::write(&path, tokens.into_string().as_bytes()).unwrap();
}
