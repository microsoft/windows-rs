mod gen_attach;
mod gen_bindings;
mod gen_reactor_txt;
mod gen_set_prop;
mod helpers;
mod metadata;
mod schema;
mod toml_parser;

use metadata::MetadataResolver;
use std::path::{Path, PathBuf};
use windows_clang::nuget_package;

/// The directory holding the reactor's WinUI/WinAppSDK `.winmd` corpus. Committed (it is a
/// shared input for `tool_webview` and `tool_composition` too) but treated as *generated*: it
/// is refreshed from the pinned packages by [`refresh_winmd`], so `gen.yml`'s zero-diff check
/// proves it matches the pin.
const WINMD_DIR: &str = "crates/tools/reactor/winmd";

/// The pinned Windows App SDK release — the single source of truth for the reactor's WinUI
/// metadata. The umbrella `Microsoft.WindowsAppSDK` metapackage at this version pins the exact
/// component package versions (Foundation / InteractiveExperiences / WinUI) in its nuspec, and
/// those components ship the `.winmd` [`refresh_winmd`] copies into [`WINMD_DIR`]. It equals the
/// runtime `windows-reactor-setup` stages (`RUNTIME_VER`, asserted in
/// [`assert_reactor_setup_pins`]), so one edit updates metadata and runtime together.
const WINDOWS_APP_SDK_VERSION: &str = "2.1.3";

fn main() {
    let time = std::time::Instant::now();

    assert_reactor_setup_pins();
    refresh_winmd();

    let resolver = MetadataResolver::load(&PathBuf::from(WINMD_DIR));

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

    let reactor_base_path = Path::new("crates/tools/reactor/src/base.txt");
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

    // The metadata this tool generates from and the runtime `reactor-setup` stages are two
    // faces of one Windows App SDK release, so their versions must match.
    assert_eq!(
        runtime_ver, WINDOWS_APP_SDK_VERSION,
        "Windows App SDK pin drift: `tool_reactor` generates from `{WINDOWS_APP_SDK_VERSION}` but \
         `windows-reactor-setup` stages runtime `{runtime_ver}`. Update `WINDOWS_APP_SDK_VERSION` \
         in this tool and `RUNTIME_VER` in {REACTOR_SETUP} together."
    );

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

/// Refresh the committed WinUI/WinAppSDK `.winmd` corpus in [`WINMD_DIR`] from the pinned
/// packages so it is reproducible and drift-checked instead of hand-copied. The umbrella
/// `Microsoft.WindowsAppSDK` metapackage at [`WINDOWS_APP_SDK_VERSION`] pins the component
/// versions in its nuspec; this copies each component's `.winmd` (plus the WebView2 `Core.winmd`
/// at `tool_webview`'s pinned version) into place. `gen.yml` re-runs this and fails on any diff,
/// so bumping the pins is the single edit that updates the whole corpus. The generated
/// `extras.winmd` is left untouched here — it is (re)built by [`generate_reactor_bindings`].
fn refresh_winmd() {
    let umbrella = nuget_package("microsoft.windowsappsdk", WINDOWS_APP_SDK_VERSION);
    let nuspec = read_nuspec(&umbrella);
    let foundation = nuspec_dependency_version(&nuspec, "Microsoft.WindowsAppSDK.Foundation");
    let interactive =
        nuspec_dependency_version(&nuspec, "Microsoft.WindowsAppSDK.InteractiveExperiences");
    let winui = nuspec_dependency_version(&nuspec, "Microsoft.WindowsAppSDK.WinUI");
    let webview = ::helpers::read_str_const("crates/tools/webview/src/main.rs", "WEBVIEW2_VERSION");

    let dir = Path::new(WINMD_DIR);
    // Clear the current corpus (but keep the generated `extras.winmd`) so a winmd dropped from a
    // newer package is removed rather than left orphaned.
    for entry in std::fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot read `{WINMD_DIR}`: {e}"))
    {
        let path = entry.expect("read winmd dir entry").path();
        let is_winmd = path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("winmd"));
        if is_winmd && path.file_name() != Some(std::ffi::OsStr::new("extras.winmd")) {
            std::fs::remove_file(&path)
                .unwrap_or_else(|e| panic!("cannot remove `{}`: {e}", path.display()));
        }
    }

    // Foundation and WinUI keep their `.winmd` directly under `metadata/`; InteractiveExperiences
    // nests them under a Windows-SDK-version folder (e.g. `metadata/10.0.18362.0/`) — take the
    // highest, which is the newest API baseline.
    let foundation_pkg = nuget_package("microsoft.windowsappsdk.foundation", &foundation);
    copy_winmd(&foundation_pkg.join("metadata"), dir);
    copy_winmd(
        &nuget_package("microsoft.windowsappsdk.winui", &winui).join("metadata"),
        dir,
    );
    let interactive_root = nuget_package(
        "microsoft.windowsappsdk.interactiveexperiences",
        &interactive,
    )
    .join("metadata");
    copy_winmd(&newest_subdir(&interactive_root), dir);

    let core = nuget_package("microsoft.web.webview2", &webview)
        .join("lib")
        .join("Microsoft.Web.WebView2.Core.winmd");
    std::fs::copy(&core, dir.join("Microsoft.Web.WebView2.Core.winmd"))
        .unwrap_or_else(|e| panic!("cannot copy `{}`: {e}", core.display()));

    refresh_bootstrap(&foundation_pkg);
}

/// The committed Windows App Runtime bootstrap DLLs `windows-reactor-setup` stages next to a
/// framework-dependent app. Treated as *generated* like [`WINMD_DIR`]: refreshed here from the
/// same pinned Foundation package so `gen.yml`'s zero-diff check proves they match
/// [`WINDOWS_APP_SDK_VERSION`] instead of being hand-copied from a local SDK install.
const BOOTSTRAP_DIR: &str = "crates/libs/reactor-setup/bootstrap";

/// The bootstrap DLL every arch folder holds. reactor-setup ships `arm64`/`x64`/`x86` (no
/// `arm64ec`), each copied from the Foundation package's `runtimes/<rid>/native/`.
const BOOTSTRAP_DLL: &str = "Microsoft.WindowsAppRuntime.Bootstrap.dll";
const BOOTSTRAP_ARCHES: &[(&str, &str)] = &[
    ("arm64", "win-arm64"),
    ("x64", "win-x64"),
    ("x86", "win-x86"),
];

/// Refresh the committed `windows-reactor-setup` bootstrap DLLs from the pinned Foundation
/// package so they cannot silently drift from the runtime `reactor-setup` stages. The `.winmd`
/// corpus and these DLLs are two faces of one Windows App SDK release, tied to a single pin.
fn refresh_bootstrap(foundation_pkg: &Path) {
    for (arch, rid) in BOOTSTRAP_ARCHES {
        let src = foundation_pkg
            .join("runtimes")
            .join(rid)
            .join("native")
            .join(BOOTSTRAP_DLL);
        let dest_dir = Path::new(BOOTSTRAP_DIR).join(arch);
        std::fs::create_dir_all(&dest_dir)
            .unwrap_or_else(|e| panic!("cannot create `{}`: {e}", dest_dir.display()));
        std::fs::copy(&src, dest_dir.join(BOOTSTRAP_DLL)).unwrap_or_else(|e| {
            panic!(
                "cannot copy `{}` -> `{}`: {e}",
                src.display(),
                dest_dir.display()
            )
        });
    }
}

/// Reads the single `*.nuspec` at the root of an extracted NuGet package.
fn read_nuspec(package_dir: &Path) -> String {
    let nuspec = std::fs::read_dir(package_dir)
        .unwrap_or_else(|e| panic!("cannot read `{}`: {e}", package_dir.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .find(|p| {
            p.extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("nuspec"))
        })
        .unwrap_or_else(|| panic!("no `.nuspec` in `{}`", package_dir.display()));
    std::fs::read_to_string(&nuspec)
        .unwrap_or_else(|e| panic!("cannot read `{}`: {e}", nuspec.display()))
}

/// Extracts the pinned `version` of a `<dependency id="..." version="..." />` from a nuspec,
/// stripping any NuGet version-range brackets (`[2.1.3]` → `2.1.3`).
fn nuspec_dependency_version(nuspec: &str, dependency_id: &str) -> String {
    let needle = format!("id=\"{dependency_id}\"");
    let element = nuspec.find(&needle).map_or_else(
        || panic!("nuspec has no dependency `{dependency_id}`"),
        |i| &nuspec[i..],
    );
    let after = element.find("version=\"").map_or_else(
        || panic!("dependency `{dependency_id}` has no version"),
        |i| &element[i + "version=\"".len()..],
    );
    let end = after
        .find('"')
        .unwrap_or_else(|| panic!("dependency `{dependency_id}` version is unterminated"));
    after[..end].trim_matches(['[', ']']).to_string()
}

/// The lexically-greatest immediate subdirectory of `dir` (the newest Windows-SDK-version
/// metadata folder in the InteractiveExperiences package).
fn newest_subdir(dir: &Path) -> PathBuf {
    std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("cannot read `{}`: {e}", dir.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .max()
        .unwrap_or_else(|| panic!("no metadata subdirectory in `{}`", dir.display()))
}

/// Copies every `.winmd` directly under `src` into `dest`.
fn copy_winmd(src: &Path, dest: &Path) {
    for entry in
        std::fs::read_dir(src).unwrap_or_else(|e| panic!("cannot read `{}`: {e}", src.display()))
    {
        let path = entry.expect("read winmd source entry").path();
        if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("winmd"))
        {
            let name = path.file_name().expect("winmd file name");
            std::fs::copy(&path, dest.join(name))
                .unwrap_or_else(|e| panic!("cannot copy `{}`: {e}", path.display()));
        }
    }
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
