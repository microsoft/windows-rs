mod gen_attach;
mod gen_bindings;
mod gen_reactor_txt;
mod gen_set_prop;
mod helpers;
mod metadata;
mod schema;
mod toml_parser;

use metadata::MetadataResolver;
use std::path::PathBuf;
use tool_reactor::stage;

fn main() {
    let time = std::time::Instant::now();

    let resolver = MetadataResolver::load(stage::winmd_dir());

    let toml_path = PathBuf::from("crates/tools/reactor/src/winui.toml");
    let toml_content = std::fs::read_to_string(&toml_path)
        .unwrap_or_else(|e| panic!("Failed to read {}: {e}", toml_path.display()));
    let mut controls = toml_parser::parse(&toml_content, &resolver);

    for ctrl in &mut controls {
        let handle = ctrl.handle().to_string();
        for p in &mut ctrl.prop {
            p.resolve_defaults(Some(&resolver), &handle);
        }
    }

    resolver.report(&controls);

    let warnings = schema::validate(&controls, &resolver);
    if !warnings.is_empty() {
        eprintln!("TOML validation warnings:");
        for w in &warnings {
            eprintln!("{w}");
        }
        eprintln!();
    }

    let bindings_code = gen_bindings::generate(&controls);
    write_if_changed("crates/libs/reactor/src/generated.rs", &bindings_code, true);

    let set_prop_code = gen_set_prop::generate(&controls, &resolver);
    write_if_changed(
        "crates/libs/reactor/src/backend/winui/generated_set_prop.rs",
        &set_prop_code,
        true,
    );

    let attach_code = gen_attach::generate(&controls, &resolver);
    write_if_changed(
        "crates/libs/reactor/src/backend/winui/generated_attach_event.rs",
        &attach_code,
        true,
    );

    let reactor_base_path = std::path::Path::new("crates/tools/reactor/src/base.txt");
    let reactor_txt_content = gen_reactor_txt::generate(&controls, &resolver, reactor_base_path);
    write_if_changed(
        "crates/tools/reactor/src/generated.txt",
        &reactor_txt_content,
        false,
    );

    generate_reactor_bindings();

    println!(
        "tool_reactor: generated code for {} controls in {:.2}s",
        controls.len(),
        time.elapsed().as_secs_f32()
    );
}

/// Generate the reactor's `bindings.rs` and test bindings from winmd + filter files.
fn generate_reactor_bindings() {
    let winmd_dir = stage::winmd_dir()
        .to_str()
        .expect("winmd dir path is valid UTF-8");

    let reactor_args = [
        "--in",
        winmd_dir,
        "crates/libs/bindgen/default/Windows.winmd",
        "crates/libs/bindgen/default/Windows.Win32.winmd",
        "--out",
        "crates/libs/reactor/src/bindings.rs",
        "--implement",
        "Microsoft.UI.Xaml.IApplicationOverrides",
        "Microsoft.UI.Xaml.Markup.IXamlMetadataProvider",
        "--minimal",
        "--dead-code",
        "--flat",
        "--filter",
        "--etc",
        "crates/tools/reactor/src/base.txt",
        "crates/tools/reactor/src/generated.txt",
    ];
    windows_bindgen::bindgen(reactor_args);

    let test_args = [
        "--in",
        winmd_dir,
        "crates/libs/bindgen/default/Windows.winmd",
        "crates/libs/bindgen/default/Windows.Win32.winmd",
        "--out",
        "crates/tests/libs/reactor_selftest/src/bindings.rs",
        "--minimal",
        "--dead-code",
        "--flat",
        "--filter",
        "--etc",
        "crates/tools/reactor/src/base.txt",
        "crates/tools/reactor/src/generated.txt",
        "crates/tools/reactor/src/test.txt",
    ];
    windows_bindgen::bindgen(test_args);
}

/// Write `content` to `path` if changed. Runs `rustfmt` when `format` is true.
fn write_if_changed(path: &str, content: &str, format: bool) {
    let content = if format {
        std::borrow::Cow::Owned(rustfmt(content))
    } else {
        std::borrow::Cow::Borrowed(content)
    };
    let path_buf = PathBuf::from(path);
    if let Ok(existing) = std::fs::read_to_string(&path_buf)
        && *existing == *content
    {
        println!("  {path}: unchanged");
        return;
    }
    if let Some(parent) = path_buf.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    std::fs::write(&path_buf, &*content).unwrap_or_else(|e| panic!("Failed to write {path}: {e}"));
    println!("  {path}: written");
}

/// Run `rustfmt` on a string of Rust code. Falls back to unformatted if
/// rustfmt is unavailable.
fn rustfmt(code: &str) -> String {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let Ok(mut child) = Command::new("rustfmt")
        .arg("--edition=2024")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    else {
        return code.to_string();
    };
    child.stdin.take().unwrap().write_all(code.as_bytes()).ok();
    match child.wait_with_output() {
        Ok(out) if out.status.success() => String::from_utf8(out.stdout).unwrap_or(code.into()),
        _ => code.to_string(),
    }
}
