use rayon::prelude::*;
use std::io::prelude::*;

/// Namespaces to exclude from code generation for the `windows` crate.
const EXCLUDE_NAMESPACES: [&str; 14] = ["Windows.AI.MachineLearning.Preview", "Windows.ApplicationModel.SocialInfo", "Windows.Devices.AllJoyn", "Windows.Devices.Perception", "Windows.Security.Authentication.Identity.Provider", "Windows.Services.Cortana", "Windows.System.Power.Diagnostics", "Windows.System.Preview", "Windows.UI.Xaml", "Windows.Win32.Interop", "Windows.Win32.System.Diagnostics.Debug.WebApp", "Windows.Win32.System.WinRT.Xaml", "Windows.Win32.Web.MsHtml", "Windows.Win32.UI.Xaml"];

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
    let mut output = std::path::PathBuf::from("crates/libs/windows/src/Windows");
    if namespace.is_empty() {
        _ = std::fs::remove_dir_all(&output);
    }
    output.pop();
    let files = metadata::reader::File::with_default(&[]).unwrap();
    let reader = &metadata::reader::Reader::new(&files);
    if !namespace.is_empty() {
        let tree = reader.tree(&namespace, &Default::default());
        gen_tree(reader, &output, &tree);
        return;
    }
    let root = reader.tree("Windows", &metadata::reader::Filter::new(&["Windows"], &EXCLUDE_NAMESPACES));
    let trees = root.flatten();
    trees.par_iter().for_each(|tree| gen_tree(reader, &output, tree));
    output.pop();
    output.push("Cargo.toml");
    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"
[package]
name = "windows"
version = "0.48.0"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
documentation = "https://microsoft.github.io/windows-docs-rs/"
readme = "../../../docs/readme.md"
rust-version = "1.48"
categories = ["os::windows-apis"]

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = []
rustc-args = ["--cfg", "docsrs"]

[dependencies.windows-targets]
version = "0.48.0"
path = "../targets"

[dependencies]
windows-implement = { path = "../implement",  version = "0.48.0", optional = true }
windows-interface = { path = "../interface",  version = "0.48.0", optional = true }

[features]
default = []
deprecated = []
implement = ["windows-implement", "windows-interface"]
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

fn gen_tree(reader: &metadata::reader::Reader, output: &std::path::Path, tree: &metadata::reader::Tree) {
    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();

    let mut gen = bindgen::Gen::new(reader);
    gen.namespace = tree.namespace;
    gen.doc = true;
    let mut tokens = bindgen::namespace(&gen, tree);
    tokens.push_str(r#"#[cfg(feature = "implement")] ::core::include!("impl.rs");"#);
    lib::format(tree.namespace, &mut tokens);
    std::fs::write(path.join("mod.rs"), tokens).unwrap();
    let mut tokens = bindgen::namespace_impl(&gen, tree);
    lib::format(tree.namespace, &mut tokens);
    std::fs::write(path.join("impl.rs"), tokens).unwrap();
}
