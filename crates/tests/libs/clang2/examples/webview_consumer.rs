use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[allow(dead_code)]
#[path = "../sdk.rs"]
mod sdk;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let [target] = args.as_slice() else {
        return Err("usage: webview_consumer <Rust target triple>".into());
    };
    if !matches!(
        target.as_str(),
        "x86_64-pc-windows-msvc" | "i686-pc-windows-msvc" | "aarch64-pc-windows-msvc"
    ) {
        return Err("expected an x64, x86, or ARM64 Windows MSVC target".into());
    }
    helpers::ensure_libclang();
    helpers::assert_libclang_version();
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .unwrap();
    let base = repo.join("target").join("clang2-webview").join(target);
    fs::create_dir_all(&base)?;
    let out = base.join(format!("run-{}", std::process::id()));
    fs::create_dir(&out)?;
    let consumer = out.join("webview");
    let loader = out.join("loader");
    fs::create_dir_all(loader.join("src"))?;
    copy_source(&repo.join("crates").join("libs").join("webview"), &consumer)?;

    let mut workspace: toml::Value = toml::from_str(&fs::read_to_string(repo.join("Cargo.toml"))?)?;
    let manifest: toml::Value = toml::from_str(&fs::read_to_string(consumer.join("Cargo.toml"))?)?;
    workspace["workspace"]["members"] = toml::Value::try_from(["webview", "loader"])?;
    let dependencies = workspace["workspace"]["dependencies"]
        .as_table_mut()
        .ok_or("missing workspace dependencies")?;
    let unused: Vec<_> = dependencies
        .keys()
        .filter(|name| manifest["dependencies"].get(name.as_str()).is_none())
        .cloned()
        .collect();
    for name in unused {
        dependencies.remove(&name);
    }
    for (_, dependency) in dependencies.iter_mut() {
        if let Some(path) = dependency.get_mut("path") {
            *path = toml::Value::String(
                repo.join(path.as_str().ok_or("invalid dependency path")?)
                    .to_str()
                    .ok_or("non-UTF-8 dependency path")?
                    .into(),
            );
        }
    }
    fs::write(out.join("Cargo.toml"), toml::to_string(&workspace)?)?;
    fs::write(
        loader.join("Cargo.toml"),
        "[package]\nname = \"clang2-webview-loader\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\
         [dependencies]\nwindows-core.workspace = true\n[lints]\nworkspace = true\n",
    )?;
    fs::write(
        loader.join("src").join("main.rs"),
        include_str!("../input/webview_loader.rs"),
    )?;

    let mut roots = sdk::webview_roots();
    roots.extend(sdk::WEBVIEW_EXPORTS);
    roots.sort_unstable();
    roots.dedup();
    let compiler_target = format!("--target={target}");
    let snapshot = sdk::capture_webview(&compiler_target, &roots, false);
    let mut options = sdk::webview_options(&compiler_target);
    // The production metadata uses flat Win32 identities, unlike the separate COM test fixture.
    for (native, reference) in &mut options.references {
        let (namespace, name) = match native.as_str() {
            "HRESULT" => ("Windows.Foundation", "HResult"),
            "_GUID" => ("System", "Guid"),
            _ => ("Windows.Win32", reference.name.as_str()),
        };
        reference.namespace = namespace.into();
        reference.name = name.into();
    }
    let resolved = snapshot.resolve()?;
    let plan = resolved.project(&options)?;
    fs::write(out.join("WebView2.rdl"), plan.rdl())?;
    let winmd = out.join("WebView2.winmd");
    windows_rdl::reader()
        .input_text(&plan.rdl())
        .reference_default()
        .output(&winmd)
        .write()?;
    let index = windows_metadata::reader::Index::read(&winmd).ok_or("invalid output metadata")?;
    for root in &roots {
        index.expect_item("WebView2", root);
    }
    let config = fs::read_to_string(sdk::tools().join("webview").join("src").join("webview.txt"))?;
    let mut input = false;
    let mut output = false;
    let mut filters = String::new();
    for line in config.lines() {
        if let Some(value) = line.strip_prefix("--in ") {
            assert!(
                !input && value == "target/webview default",
                "unexpected production input"
            );
            input = true;
        } else if let Some(value) = line.strip_prefix("--out ") {
            assert!(
                !output && value == "crates/libs/webview/src/bindings.rs",
                "unexpected production output"
            );
            output = true;
        } else {
            filters.push_str(line);
            filters.push('\n');
        }
    }
    assert!(input && output, "missing production input/output");
    let filter = out.join("webview.txt");
    fs::write(&filter, filters)?;
    windows_bindgen::bindgen([
        "--in",
        "default",
        winmd.to_str().unwrap(),
        "--out",
        consumer.join("src").join("bindings.rs").to_str().unwrap(),
        "--etc",
        filter.to_str().unwrap(),
    ]);
    windows_bindgen::bindgen([
        "--in",
        "default",
        winmd.to_str().unwrap(),
        "--out",
        loader.join("src").join("bindings.rs").to_str().unwrap(),
        "--flat",
        "--minimal",
        "--filter",
        "WebView2.CompareBrowserVersions",
        "Windows.Win32.GetModuleHandleW",
        "Windows.Win32.GetModuleFileNameW",
    ]);
    for features in ["", "system", "reactor", "system,reactor"] {
        let mut command = cargo(&out, target);
        command.args([
            "check",
            "--quiet",
            "-p",
            "windows-webview",
            "--no-default-features",
        ]);
        if !features.is_empty() {
            command.args(["--features", features]);
        }
        run(&mut command)?;
        println!("consumer {target} features={features:?}: passed");
    }
    for release in [false, true] {
        let mut command = cargo(&out, target);
        command.args(["build", "--quiet", "-p", "clang2-webview-loader"]);
        if release {
            command.arg("--release");
        }
        run(&mut command)?;
        if target.starts_with("aarch64") && std::env::consts::ARCH != "aarch64" {
            println!("loader {target}: compiled only; no local ARM64 execution");
            continue;
        }
        let library = sdk::webview_library(&compiler_target);
        let dll = library.parent().unwrap().join("WebView2Loader.dll");
        let path = std::env::join_paths(
            std::iter::once(library.parent().unwrap().to_path_buf()).chain(std::env::split_paths(
                &std::env::var_os("PATH").ok_or("missing PATH")?,
            )),
        )?;
        let exe = base
            .join("build")
            .join(target)
            .join(if release { "release" } else { "debug" })
            .join("clang2-webview-loader.exe");
        run(Command::new(exe).arg(&dll).env("PATH", path))?;
        println!("loader {target} release={release}: passed");
    }
    println!("artifacts={}", out.display());
    Ok(())
}

fn cargo(out: &Path, target: &str) -> Command {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command
        .current_dir(out)
        .env("CARGO_BUILD_TARGET", target)
        .env("CARGO_TARGET_DIR", out.parent().unwrap().join("build"));
    command
}

fn run(command: &mut Command) -> Result<()> {
    let status = command.status()?;
    if !status.success() {
        return Err(format!("{command:?} failed: {status}").into());
    }
    Ok(())
}

fn copy_source(source: &Path, destination: &Path) -> Result<()> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        if entry.file_name() == "bindings.rs" {
            continue;
        }
        let to: PathBuf = destination.join(entry.file_name());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            copy_source(&entry.path(), &to)?;
        } else if kind.is_file() {
            fs::copy(entry.path(), to)?;
        } else {
            return Err(format!("unsupported source entry: {}", entry.path().display()).into());
        }
    }
    Ok(())
}
