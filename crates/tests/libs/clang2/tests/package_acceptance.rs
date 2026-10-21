use std::path::Path;

const OUTPUT: &str = "target\\test-clang2-package-acceptance";
const COMBINED: &str = "target\\test-clang2-package-acceptance\\combined.winmd";

#[test]
#[ignore = "generates full candidate packages and requires all three MSVC Rust targets"]
fn candidate_packages() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..\\..\\..\\..")
        .canonicalize()
        .unwrap();
    std::env::set_current_dir(&repo).unwrap();
    let output = repo.join(OUTPUT);
    for name in ["input", "sys", "windows"] {
        let path = output.join(name);
        if path.exists() {
            std::fs::remove_dir_all(&path).unwrap();
        }
        std::fs::create_dir_all(path).unwrap();
    }
    windows_rdl::reader()
        .input("metadata\\metadata.rdl")
        .input("metadata\\win32")
        .input("metadata\\wdk")
        .reference(tool_package::WINRT_WINMD)
        .output(COMBINED)
        .write()
        .unwrap();
    let mut plan = tool_package::remap_plan();
    plan.winmd = COMBINED;
    let remapped = output.join("input\\Windows.Win32.winmd");
    let summary = tool_package::remap::run(&plan, remapped.to_str().unwrap());
    assert!(!summary.is_empty());
    let index = windows_metadata::reader::Index::read(&remapped).unwrap();
    assert!(!index.contains_namespace("Windows.Win32"));
    std::fs::copy(
        tool_package::WINRT_WINMD,
        output.join("input\\Windows.winmd"),
    )
    .unwrap();

    let manifest = std::fs::read_to_string(repo.join("Cargo.toml")).unwrap();
    let (_, shared) = manifest.split_once("[workspace.lints.rust]").unwrap();
    let mut manifest = String::from(
        "[workspace]\nresolver = \"3\"\nmembers = [\"sys\", \"windows\"]\n\n[workspace.lints.rust]",
    );
    for line in shared.lines() {
        if let Some((_, tail)) = line.split_once("path = \"") {
            let (relative, _) = tail.split_once('"').unwrap();
            let absolute = repo.join(relative);
            manifest.push_str(&line.replace(
                &format!("path = \"{relative}\""),
                &format!("path = '{}'", absolute.display()),
            ));
        } else {
            manifest.push_str(line);
        }
        manifest.push('\n');
    }
    std::fs::write(output.join("Cargo.toml"), manifest).unwrap();
    for name in ["sys", "windows"] {
        let destination = output.join(name);
        std::fs::create_dir_all(destination.join("src")).unwrap();
        let source = repo.join("crates\\libs").join(name);
        for file in ["Cargo.toml", "src\\lib.rs"] {
            std::fs::copy(source.join(file), destination.join(file)).unwrap();
        }
        if name == "sys" {
            copy_tree(&source.join("src\\core"), &destination.join("src\\core"));
        }
        let settings = std::fs::read_to_string(
            repo.join("crates\\tools\\package\\src")
                .join(format!("{name}.txt")),
        )
        .unwrap();
        let mut arguments: Vec<String> = settings.split_whitespace().map(str::to_owned).collect();
        for (flag, value) in [("--in", output.join("input")), ("--out", destination)] {
            let position = arguments
                .iter()
                .position(|argument| argument == flag)
                .unwrap();
            arguments[position + 1] = value.to_str().unwrap().into();
        }
        windows_bindgen::bindgen(arguments);
    }
    for target in [
        "i686-pc-windows-msvc",
        "x86_64-pc-windows-msvc",
        "aarch64-pc-windows-msvc",
    ] {
        let result = std::process::Command::new("cargo")
            .args([
                "check",
                "--quiet",
                "--offline",
                "--workspace",
                "--all-features",
            ])
            .arg("--manifest-path")
            .arg(output.join("Cargo.toml"))
            .arg("--target-dir")
            .arg(output.join("build"))
            .args(["--target", target])
            .env("RUSTFLAGS", "-D warnings")
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{target}:\n{}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr),
        );
    }
}

fn copy_tree(source: &Path, destination: &Path) {
    std::fs::create_dir_all(destination).unwrap();
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let destination = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &destination);
        } else {
            std::fs::copy(entry.path(), destination).unwrap();
        }
    }
}
