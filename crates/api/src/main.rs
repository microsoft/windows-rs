use std::io::prelude::*;
use rayon::prelude::*;

fn main() {
    let start = std::time::Instant::now();
    let mut output = std::path::PathBuf::from(reader::workspace_dir());
    output.pop();
    output.push("windows-api-rs\\src");

    let _ = std::fs::remove_dir_all(&output);
    let reader = reader::TypeReader::get_mut();
    include_all(&mut reader.types);

    let root = reader.types.get_namespace("Windows").unwrap();

    let mut trees = Vec::new();
    collect_trees(&output, root.namespace, &root, &mut trees);
    trees.par_iter().for_each(|tree|gen_tree(&output, root.namespace, tree));

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
    let version = env!("CARGO_PKG_VERSION");

    file.write_all(
        format!(
            r#"[package]
name = "windows-api"
version = "{}"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Windows API"

[dependencies]
windows = {{ version = "{}", default-features = false, path = "/git/windows-rs" }}

[workspace]
members = ["examples/*"]

[features]
"#,
            version, version
        )
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

fn collect_trees<'a>(output: &std::path::Path, root: &'static str, tree: &'a reader::TypeTree, trees: &mut Vec<&'a reader::TypeTree>) {
    trees.push(tree);
    
    tree.namespaces
    .values()
    .for_each(|tree| collect_trees(output, root, tree, trees));

    let mut path = std::path::PathBuf::from(output);

    if root == tree.namespace {
        std::fs::create_dir_all(&path).unwrap();
    } else {
        path.push(tree.namespace[root.len() + 1..].replace('.', "\\"));
        std::fs::create_dir_all(&path).unwrap();
    }
}

fn gen_tree(output: &std::path::Path, root: &'static str, tree: &reader::TypeTree) {
    println!("{}", tree.namespace);
    let mut path = std::path::PathBuf::from(output);

    if root == tree.namespace {
        path.push("lib.rs");
    } else {
        path.push(tree.namespace[root.len() + 1..].replace('.', "\\"));
        path.push("mod.rs");
    }

    let tokens = gen::gen_source_file(root, tree);
    std::fs::write(&path, tokens.into_string().as_bytes()).unwrap();
}
