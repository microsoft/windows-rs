use std::io::prelude::*;

fn main() {
    // let output = std::env::args()
    //     .nth(1)
    //     .expect("Expected one command line argument for output directory");
    let output = r#"c:\git\windows-api-rs\src"#;

    let mut output = std::path::PathBuf::from(&output);

    let _ = std::fs::remove_dir_all(&output);
    let reader = reader::TypeReader::get_mut();
    include_all(&mut reader.types);

    let root = reader.types.get_namespace("Windows").unwrap();

    gen_tree(&output, root.namespace, root);

    output.pop();
    output.push("Cargo.toml");

    write_toml(&output, root);

    println!("Formatting...");
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("fmt");
    cmd.arg("--manifest-path");
    cmd.arg(output);
    cmd.output().unwrap();
}

fn write_toml(output: &std::path::Path, tree: &reader::TypeTree) {
    let mut file = std::fs::File::create(&output).unwrap();
    let version = env!("CARGO_PKG_VERSION");

    // TODO: pin the windows crate dependency to the same "version" so everything is versioned in lockstep.
    // Currently its "0.21" to ease development.
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
windows = {{ version = "0.21", default-features = false }}

[features]
"#,
            version
        )
        .as_bytes(),
    )
    .unwrap();

    write_features(&mut file, tree);
}

fn write_features(file: &mut std::fs::File, tree: &reader::TypeTree) {
    for tree in tree.namespaces.values() {
        write_feature(file, tree.namespace);
        write_features(file, tree);
    }
}

fn write_feature(file: &mut std::fs::File, namespace: &str) {
    let namespace = namespace[namespace.find('.').unwrap() + 1..].replace('.', "_");

    let parent = if let Some(pos) = namespace.rfind('_') {
        format!("\"{}\"", &namespace[0..pos])
    } else {
        "".to_string()
    };

    file.write_all(format!("{} = [{}]\n", namespace, parent).as_bytes())
        .unwrap();
}

fn include_all(tree: &mut reader::TypeTree) {
    tree.include = true;

    tree.types
        .values_mut()
        .for_each(|entry| entry.include = reader::TypeInclude::Full);

    tree.namespaces
        .values_mut()
        .for_each(|tree| include_all(tree));
}

fn gen_tree(output: &std::path::Path, root: &str, tree: &reader::TypeTree) {
    println!("{}", tree.namespace);
    let mut path = std::path::PathBuf::from(output);

    if root == tree.namespace {
        std::fs::create_dir_all(&path).unwrap();
        path.push("lib.rs");
    } else {
        path.push(tree.namespace[root.len() + 1..].replace('.', "\\"));
        std::fs::create_dir_all(&path).unwrap();
        path.push("mod.rs");
    }

    let mut file = std::fs::File::create(&path).unwrap();
    let tokens = gen::gen_source_file(tree);
    file.write_all(tokens.into_string().as_bytes()).unwrap();

    tree.namespaces
        .values()
        .for_each(|tree| gen_tree(output, root, tree));
}
