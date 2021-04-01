use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let output = std::env::args()
        .nth(1)
        .expect("Expected one command line argument for output directory");

    let output = std::path::Path::new(&output);
    let _ = std::fs::remove_dir_all(output);

    let mut path = std::path::PathBuf::from(output);
    std::fs::create_dir_all(&path)?;
    path.push("publish.bat");
    let mut file = std::fs::File::create(&path)?;
    let reader = gen::TypeReader::get();

    for namespace in namespaces(reader) {
        let crate_name = namespace.replace('.', "-").to_lowercase();
        println!("{}", crate_name);

        gen_crate(
            &output,
            &crate_name,
            reader,
            namespace,
            env!("CARGO_PKG_VERSION"),
        )?;

        file.write_all(
            format!(
                "cargo publish --no-verify --manifest-path {}\\Cargo.toml\n",
                crate_name
            )
            .as_bytes(),
        )?;
    }

    Ok(())
}

fn namespaces(reader: &'static gen::TypeReader) -> std::collections::BTreeSet<&'static str> {
    let mut set = std::collections::BTreeSet::new();

    for namespace in reader.namespaces() {
        if let Some(first) = namespace.find('.') {
            if let Some(second) = namespace[first + 1..].find('.') {
                set.insert(&namespace[..first + 1 + second]);
            }
        }
    }

    // Windows.UI.Xaml should be distinct from Windows.UI as it is so large.
    set.insert("Windows.UI.Xaml");
    set
}

fn gen_crate(
    output: &std::path::Path,
    crate_name: &str,
    reader: &'static gen::TypeReader,
    module: &'static str,
    version: &str,
) -> std::io::Result<()> {
    let mut path = std::path::PathBuf::from(output);
    path.push(&crate_name);
    std::fs::create_dir_all(&path)?;
    path.push("Cargo.toml");
    let mut file = std::fs::File::create(&path)?;

    file.write_all(
        format!(
            r#"
[package]
name = "{}"
version = "{}"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "{}"

[dependencies]
windows = "{}"
"#,
            crate_name, version, module, version
        )
        .as_bytes(),
    )?;

    path.pop();
    path.push("src");
    std::fs::create_dir_all(&path)?;
    path.push("lib.rs");
    let mut file = std::fs::File::create(&path)?;
    let mut limits = gen::TypeLimits::new(reader);

    for namespace in reader.namespaces() {
        if !namespace.starts_with(module) {
            continue;
        }

        if module == "Windows.UI" && namespace.starts_with("Windows.UI.Xaml") {
            continue;
        }

        println!("- {}", namespace);

        limits.insert(gen::NamespaceTypes {
            namespace,
            limit: gen::TypeLimit::All,
        });
    }

    let tree = gen::TypeTree::from_limits(reader, &limits);

    let ts = tree
        .gen(&tree)
        .fold(gen::TokenStream::new(), |mut accum, n| {
            accum.combine(&n);
            accum
        });

    file.write_all(ts.into_string().as_bytes())?;
    let mut cmd = ::std::process::Command::new("rustfmt");
    cmd.arg(&path);
    cmd.output()?;
    Ok(())
}
