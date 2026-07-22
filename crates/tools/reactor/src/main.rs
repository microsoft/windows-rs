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

fn main() {
    let time = std::time::Instant::now();

    assert_reactor_setup_pins();

    let resolver = MetadataResolver::load(&PathBuf::from("crates/tools/reactor/winmd"));

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

/// Fails loudly if `windows-reactor-setup` — the published crate that stages the WinAppSDK and
/// WebView2 runtimes for reactor apps — drifts from the versions the rest of the toolchain is
/// pinned to. `reactor-setup` is a runtime dependency with no generated artifact of its own, so
/// this is the only place that keeps its pins honest: `tool_reactor` already runs in `gen.yml`,
/// so a drifted pin turns into a red build here rather than a silent runtime mismatch.
fn assert_reactor_setup_pins() {
    const REACTOR_SETUP: &str = "crates/libs/reactor-setup/src/lib.rs";
    const WEBVIEW_TOOL: &str = "crates/tools/webview/src/main.rs";
    const REACTOR_YML: &str = ".github/workflows/reactor.yml";

    // The WebView2 runtime `reactor-setup` stages must match the exact package `tool_webview`
    // downloads its headers from, so the COM ABI the bindings target is the ABI apps ship.
    let setup_webview2 = ::helpers::read_str_const(REACTOR_SETUP, "WEBVIEW2_VER");
    let tool_webview2 = ::helpers::read_str_const(WEBVIEW_TOOL, "WEBVIEW2_VERSION");
    assert_eq!(
        setup_webview2, tool_webview2,
        "WebView2 pin drift: `windows-reactor-setup` stages `{setup_webview2}` but `tool_webview` \
         downloads headers for `{tool_webview2}`. Update `WEBVIEW2_VER` in {REACTOR_SETUP} and \
         `WEBVIEW2_VERSION` in {WEBVIEW_TOOL} together."
    );

    // CI installs the WinAppSDK runtime before running the reactor self-tests; it must be the
    // same version `reactor-setup` stages so the tests exercise what apps actually ship. The
    // aka.ms installer URL is `.../windowsappsdk/<major.minor>/<version>/...`.
    let runtime_ver = ::helpers::read_str_const(REACTOR_SETUP, "RUNTIME_VER");
    let channel = runtime_ver
        .match_indices('.')
        .nth(1)
        .map_or(runtime_ver.as_str(), |(i, _)| &runtime_ver[..i]);
    let expected = format!("windowsappsdk/{channel}/{runtime_ver}/");
    let yml = std::fs::read_to_string(REACTOR_YML)
        .unwrap_or_else(|e| panic!("failed to read `{REACTOR_YML}`: {e}"));
    assert!(
        yml.contains(&expected),
        "WinAppSDK runtime drift: `{REACTOR_YML}` does not install `{expected}` — \
         `windows-reactor-setup` pins `RUNTIME_VER = \"{runtime_ver}\"`. Update the installer URL \
         in {REACTOR_YML} to match."
    );
}

/// Generate the reactor's `bindings.rs` and test bindings from winmd + filter files.
fn generate_reactor_bindings() {
    windows_rdl::Reader::new()
        .input("crates/tools/reactor/src/extras.rdl")
        .input("crates/libs/bindgen/default/Windows.Win32.winmd")
        .output("crates/tools/reactor/winmd/extras.winmd")
        .write()
        .unwrap();

    let reactor_args = [
        "--in",
        "crates/tools/reactor/winmd",
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
        "crates/tools/reactor/winmd",
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
