mod generate;
mod generate_surface;
mod generate_winui;
mod helpers;
mod metadata;
mod schema;

use metadata::MetadataResolver;
use schema::workspace_path;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use windows_clang::nuget_package;

const OUTPUT: &str = "crates/libs/reactor/src/generated.rs";
const BINDINGS: &str = "crates/libs/reactor/src/native/winui/bindings.rs";
const RUNTIME_BINDINGS_FILTER: &str = "crates/tools/reactor/src/bindings.txt";
const CONTROL_BINDINGS_FILTER: &str = "crates/tools/reactor/src/control_bindings.txt";
const CANVAS_BINDINGS: &str = "crates/libs/canvas/src/reactor_bindings.rs";
const CANVAS_FILTER: &str = "crates/tools/reactor/src/canvas.txt";
const WINUI_OUTPUT: &str = "crates/libs/reactor/src/native/winui/generated.rs";
const SURFACE_OUTPUT: &str = "crates/tests/libs/reactor_surface/src/generated_surface.rs";
const WINMD: &str = "crates/tools/reactor/winmd";
const EXTRAS_RDL: &str = "crates/tools/reactor/src/extras.rdl";
const EXTRAS_WINMD: &str = "crates/tools/reactor/winmd/extras.winmd";
const SCHEMA: &str = "crates/tools/reactor/src/winui.toml";
const WINDOWS_APP_SDK_VERSION: &str = "2.4.0";
const BOOTSTRAP_DIR: &str = "crates/libs/reactor-setup/bootstrap";
const BOOTSTRAP_DLL: &str = "Microsoft.WindowsAppRuntime.Bootstrap.dll";
const BOOTSTRAP_ARCHES: &[(&str, &str)] = &[
    ("arm64", "win-arm64"),
    ("x64", "win-x64"),
    ("x86", "win-x86"),
];

fn main() {
    assert_reactor_setup_pins();
    refresh_winmd();
    generate_extras();

    let source = fs::read_to_string(workspace_path(SCHEMA)).unwrap();
    let schema = schema::Schema::parse(&source).unwrap();
    let metadata = MetadataResolver::load(&workspace_path(WINMD));
    let resolved = schema.resolve(&metadata).unwrap();
    let generated = helpers::rustfmt(&generate::generate(&resolved));

    write_if_changed(OUTPUT, &generated);
    write_if_changed(
        SURFACE_OUTPUT,
        &helpers::rustfmt(&generate_surface::generate(&resolved)),
    );
    write_if_changed(
        CONTROL_BINDINGS_FILTER,
        &generate_winui::generate_control_bindings_filter(&resolved),
    );
    write_if_changed(
        WINUI_OUTPUT,
        &helpers::rustfmt(&generate_winui::generate(&resolved)),
    );
    // A file-path pass stabilizes rustfmt's handling of deeply nested generated closures.
    format_file(WINUI_OUTPUT);

    windows_bindgen::builder()
        .input(workspace_path(WINMD))
        .input_default()
        .output(workspace_path(BINDINGS))
        .implements([
            "Microsoft.UI.Xaml.IElementFactory",
            "Microsoft.UI.Xaml.IApplicationOverrides",
            "Microsoft.UI.Xaml.Markup.IXamlMetadataProvider",
        ])
        .minimal()
        .dead_code()
        .flat()
        .filter_files([
            workspace_path(RUNTIME_BINDINGS_FILTER),
            workspace_path(CONTROL_BINDINGS_FILTER),
        ])
        .write();

    windows_bindgen::builder()
        .input(workspace_path(WINMD))
        .input_default()
        .output(workspace_path(CANVAS_BINDINGS))
        .minimal()
        .dead_code()
        .flat()
        .filter_file(workspace_path(CANVAS_FILTER))
        .write();
}

fn assert_reactor_setup_pins() {
    const REACTOR_SETUP: &str = "crates/libs/reactor-setup/src/lib.rs";
    const REACTOR_NATIVE: &str = "crates/libs/reactor/src/native/winui/mod.rs";
    const WEBVIEW_TOOL: &str = "crates/tools/webview/src/main.rs";

    let setup_webview2 = ::helpers::read_str_const(workspace_path(REACTOR_SETUP), "WEBVIEW2_VER");
    let tool_webview2 = ::helpers::read_str_const(workspace_path(WEBVIEW_TOOL), "WEBVIEW2_VERSION");
    assert_eq!(
        setup_webview2, tool_webview2,
        "WebView2 pin drift: `windows-reactor-setup` stages `{setup_webview2}` but \
         `tool_webview` generates from `{tool_webview2}`. Update `WEBVIEW2_VER` in \
         {REACTOR_SETUP} and `WEBVIEW2_VERSION` in {WEBVIEW_TOOL} together."
    );

    let runtime_ver = ::helpers::read_str_const(workspace_path(REACTOR_SETUP), "RUNTIME_VER");
    assert_eq!(
        runtime_ver, WINDOWS_APP_SDK_VERSION,
        "Windows App SDK pin drift: `tool_reactor` generates from \
         `{WINDOWS_APP_SDK_VERSION}` but `windows-reactor-setup` stages `{runtime_ver}`. Update \
         `WINDOWS_APP_SDK_VERSION` in this tool and `RUNTIME_VER` in {REACTOR_SETUP} together."
    );

    let setup_marker =
        ::helpers::read_str_const(workspace_path(REACTOR_SETUP), "SELF_CONTAINED_MARKER");
    let native_marker =
        ::helpers::read_str_const(workspace_path(REACTOR_NATIVE), "SELF_CONTAINED_MARKER");
    assert_eq!(
        setup_marker, native_marker,
        "self-contained manifest marker drift: `windows-reactor-setup` embeds `{setup_marker}` but \
         `windows-reactor` searches for `{native_marker}`. Update `SELF_CONTAINED_MARKER` in \
         {REACTOR_SETUP} and {REACTOR_NATIVE} together."
    );
}

fn generate_extras() {
    windows_rdl::Reader::new()
        .input(workspace_path(EXTRAS_RDL))
        .reference_bytes(windows_default::WIN32)
        .output(workspace_path(EXTRAS_WINMD))
        .write()
        .unwrap();
}

fn refresh_winmd() {
    let umbrella = nuget_package("microsoft.windowsappsdk", WINDOWS_APP_SDK_VERSION);
    let nuspec = read_nuspec(&umbrella);
    let foundation = nuspec_dependency_version(&nuspec, "Microsoft.WindowsAppSDK.Foundation");
    let interactive =
        nuspec_dependency_version(&nuspec, "Microsoft.WindowsAppSDK.InteractiveExperiences");
    let winui = nuspec_dependency_version(&nuspec, "Microsoft.WindowsAppSDK.WinUI");
    let webview = ::helpers::read_str_const(
        workspace_path("crates/tools/webview/src/main.rs"),
        "WEBVIEW2_VERSION",
    );

    let dir = workspace_path(WINMD);
    for entry in fs::read_dir(&dir).unwrap_or_else(|error| panic!("cannot read `{WINMD}`: {error}"))
    {
        let path = entry.unwrap().path();
        let is_winmd = path
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("winmd"));
        if is_winmd && path.file_name() != Some(std::ffi::OsStr::new("extras.winmd")) {
            fs::remove_file(&path)
                .unwrap_or_else(|error| panic!("cannot remove `{}`: {error}", path.display()));
        }
    }

    let foundation_package = nuget_package("microsoft.windowsappsdk.foundation", &foundation);
    copy_winmd(&foundation_package.join("metadata"), &dir);
    copy_winmd(
        &nuget_package("microsoft.windowsappsdk.winui", &winui).join("metadata"),
        &dir,
    );
    let interactive_metadata = nuget_package(
        "microsoft.windowsappsdk.interactiveexperiences",
        &interactive,
    )
    .join("metadata");
    copy_winmd(&newest_subdir(&interactive_metadata), &dir);

    let webview_core = nuget_package("microsoft.web.webview2", &webview)
        .join("lib")
        .join("Microsoft.Web.WebView2.Core.winmd");
    fs::copy(&webview_core, dir.join("Microsoft.Web.WebView2.Core.winmd"))
        .unwrap_or_else(|error| panic!("cannot copy `{}`: {error}", webview_core.display()));

    refresh_bootstrap(&foundation_package);
}

fn refresh_bootstrap(foundation_package: &Path) {
    for (architecture, runtime_id) in BOOTSTRAP_ARCHES {
        let source = foundation_package
            .join("runtimes")
            .join(runtime_id)
            .join("native")
            .join(BOOTSTRAP_DLL);
        let destination = workspace_path(BOOTSTRAP_DIR).join(architecture);
        fs::create_dir_all(&destination)
            .unwrap_or_else(|error| panic!("cannot create `{}`: {error}", destination.display()));
        fs::copy(&source, destination.join(BOOTSTRAP_DLL))
            .unwrap_or_else(|error| panic!("cannot copy `{}`: {error}", source.display()));
    }
}

fn read_nuspec(package_dir: &Path) -> String {
    let nuspec = fs::read_dir(package_dir)
        .unwrap_or_else(|error| panic!("cannot read `{}`: {error}", package_dir.display()))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .find(|path| {
            path.extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("nuspec"))
        })
        .unwrap_or_else(|| panic!("no `.nuspec` in `{}`", package_dir.display()));
    fs::read_to_string(&nuspec)
        .unwrap_or_else(|error| panic!("cannot read `{}`: {error}", nuspec.display()))
}

fn nuspec_dependency_version(nuspec: &str, dependency_id: &str) -> String {
    let needle = format!("id=\"{dependency_id}\"");
    let element = nuspec.find(&needle).map_or_else(
        || panic!("nuspec has no dependency `{dependency_id}`"),
        |index| &nuspec[index..],
    );
    let after = element.find("version=\"").map_or_else(
        || panic!("dependency `{dependency_id}` has no version"),
        |index| &element[index + "version=\"".len()..],
    );
    let end = after
        .find('"')
        .unwrap_or_else(|| panic!("dependency `{dependency_id}` version is unterminated"));
    after[..end].trim_matches(['[', ']']).to_string()
}

fn newest_subdir(dir: &Path) -> PathBuf {
    fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("cannot read `{}`: {error}", dir.display()))
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.is_dir())
        .max()
        .unwrap_or_else(|| panic!("no metadata subdirectory in `{}`", dir.display()))
}

fn copy_winmd(source: &Path, destination: &Path) {
    for entry in fs::read_dir(source)
        .unwrap_or_else(|error| panic!("cannot read `{}`: {error}", source.display()))
    {
        let path = entry.unwrap().path();
        if path
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("winmd"))
        {
            let name = path.file_name().unwrap();
            fs::copy(&path, destination.join(name))
                .unwrap_or_else(|error| panic!("cannot copy `{}`: {error}", path.display()));
        }
    }
}

fn write_if_changed(path: &str, value: &str) {
    let path = workspace_path(path);
    if !matches!(fs::read_to_string(&path).as_deref(), Ok(current) if current == value) {
        fs::write(path, value).unwrap();
    }
}

fn format_file(path: &str) {
    let status = Command::new("rustfmt")
        .arg("--edition=2024")
        .arg("--config-path")
        .arg(workspace_path("rustfmt.toml"))
        .arg(workspace_path(path))
        .status()
        .unwrap();
    assert!(status.success(), "failed to format {path}");
}
