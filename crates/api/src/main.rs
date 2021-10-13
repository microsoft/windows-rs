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

    gen_tree(
        &output,
        "Windows",
        reader.types.get_namespace("Windows").unwrap(),
    );

    output.pop();
    output.push("Cargo.toml");

    println!("Formatting...");
    let mut cmd = std::process::Command::new("cargo");
    cmd.arg("fmt");
    cmd.arg("--manifest-path");
    cmd.arg(output);
    cmd.output().unwrap();
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
