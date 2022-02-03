use rayon::prelude::*;
use std::io::prelude::*;

const EXCLUDE_NAMESPACES: [&str; 1] = ["Windows.Win32.Interop"];

fn main() {
    let mut output = std::path::PathBuf::from(metadata::workspace_dir());
    output.push("crates/libs/sys/src/Windows");
    let _ = std::fs::remove_dir_all(&output);
    output.pop();

    let reader = metadata::TypeReader::get();
    let root = reader.types.get_namespace("Windows").unwrap();

    let mut trees = Vec::new();
    collect_trees(&output, root.namespace, root, &mut trees);
    trees.par_iter().for_each(|tree| gen_tree(&output, root.namespace, tree));

    output.pop();
    output.push("Cargo.toml");

    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"
[package]
name = "windows-sys"
version = "0.31.1"
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
windows_i686_msvc = { path = "../../targets/i686_msvc", version = "0.31.1" }

[target.i686-uwp-windows-msvc.dependencies]
windows_i686_msvc = { path = "../../targets/i686_msvc", version = "0.31.1" }

[target.x86_64-pc-windows-msvc.dependencies]
windows_x86_64_msvc = { path = "../../targets/x86_64_msvc", version = "0.31.1" }

[target.x86_64-uwp-windows-msvc.dependencies]
windows_x86_64_msvc = { path = "../../targets/x86_64_msvc", version = "0.31.1" }

[target.aarch64-pc-windows-msvc.dependencies]
windows_aarch64_msvc = { path = "../../targets/aarch64_msvc", version = "0.31.1" }

[target.aarch64-uwp-windows-msvc.dependencies]
windows_aarch64_msvc = { path = "../../targets/aarch64_msvc", version = "0.31.1" }

[target.i686-pc-windows-gnu.dependencies]
windows_i686_gnu = { path = "../../targets/i686_gnu", version = "0.31.1" }

[target.i686-uwp-windows-gnu.dependencies]
windows_i686_gnu = { path = "../../targets/i686_gnu", version = "0.31.1" }

[target.x86_64-pc-windows-gnu.dependencies]
windows_x86_64_gnu = { path = "../../targets/x86_64_gnu", version = "0.31.1" }

[target.x86_64-uwp-windows-gnu.dependencies]
windows_x86_64_gnu = { path = "../../targets/x86_64_gnu", version = "0.31.1" }

[features]
default = []
deprecated = []
"#
        .as_bytes(),
    )
    .unwrap();

    // Skip the root Windows tree while writing features
    for tree in trees.iter().skip(1) {
        // TODO: don't include parent features automatically
        let feature = tree.namespace[root.namespace.len() + 1..].replace('.', "_");

        if let Some(pos) = feature.rfind('_') {
            let dependency = &feature[..pos];

            file.write_all(format!("{} = [\"{}\"]\n", feature, dependency).as_bytes()).unwrap();
        } else {
            file.write_all(format!("{} = []\n", feature).as_bytes()).unwrap();
        }
    }
}

fn collect_trees<'a>(output: &std::path::Path, root: &'static str, tree: &'a metadata::TypeTree, trees: &mut Vec<&'a metadata::TypeTree>) {
    if EXCLUDE_NAMESPACES.iter().any(|&x| x == tree.namespace) {
        return;
    }

    trees.push(tree);
    tree.namespaces.values().for_each(|tree| collect_trees(output, root, tree, trees));
    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();
}

fn gen_tree(output: &std::path::Path, _root: &'static str, tree: &metadata::TypeTree) {
    let mut path = std::path::PathBuf::from(output);

    path.push(tree.namespace.replace('.', "/"));
    path.push("mod.rs");

    let gen = bindgen::Gen { namespace: tree.namespace, sys: true, cfg: true, doc: true, ..Default::default() };
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
