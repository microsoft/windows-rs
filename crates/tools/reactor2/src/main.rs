use std::path::{Path, PathBuf};
use windows_bindgen::*;
use windows_clang::nuget_package;

const REACTOR_RDL: &str = "crates/tools/reactor2/src/reactor.rdl";
const REACTOR_FILTER: &str = "crates/tools/reactor2/src/reactor.txt";

fn main() {
    let output = Path::new("target/reactor2-bindings");
    std::fs::create_dir_all(output).unwrap();

    let rdl = output.join("reactor.rdl");
    let winmd = output.join("reactor.winmd");
    std::fs::write(&rdl, reactor_rdl()).unwrap();
    windows_rdl::Reader::new()
        .input(&rdl)
        .reference_default()
        .output(&winmd)
        .write()
        .unwrap();

    let metadata = windows_app_sdk_metadata();
    let webview_version =
        helpers::read_str_const("crates/libs/reactor-setup/src/lib.rs", "WEBVIEW2_VER");
    let webview = nuget_package("microsoft.web.webview2", &webview_version)
        .join("lib")
        .join("Microsoft.Web.WebView2.Core.winmd");

    builder()
        .inputs([
            metadata.foundation,
            metadata.interactive,
            metadata.winui,
            webview,
            winmd,
        ])
        .input_default()
        .output("crates/libs/reactor2/src/bindings.rs")
        .implements([
            "Microsoft.UI.Xaml.IApplicationOverrides",
            "Microsoft.UI.Xaml.Markup.IXamlMetadataProvider",
        ])
        .minimal()
        .dead_code()
        .flat()
        .filter_file(REACTOR_FILTER)
        .write();
}

struct WindowsAppSdkMetadata {
    foundation: PathBuf,
    interactive: PathBuf,
    winui: PathBuf,
}

fn windows_app_sdk_metadata() -> WindowsAppSdkMetadata {
    let version = helpers::read_str_const("crates/libs/reactor-setup/src/lib.rs", "RUNTIME_VER");
    let umbrella = nuget_package("microsoft.windowsappsdk", &version);
    let nuspec = std::fs::read_dir(&umbrella)
        .unwrap_or_else(|error| panic!("cannot read `{}`: {error}", umbrella.display()))
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            path.extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("nuspec"))
        })
        .unwrap_or_else(|| panic!("no `.nuspec` in `{}`", umbrella.display()));
    let nuspec = std::fs::read_to_string(&nuspec)
        .unwrap_or_else(|error| panic!("cannot read `{}`: {error}", nuspec.display()));
    let dependency_version = |name| {
        let needle = format!("id=\"{name}\"");
        let dependency = nuspec.find(&needle).map_or_else(
            || panic!("nuspec has no dependency `{name}`"),
            |index| &nuspec[index..],
        );
        let version = dependency.find("version=\"").map_or_else(
            || panic!("dependency `{name}` has no version"),
            |index| &dependency[index + "version=\"".len()..],
        );
        let end = version
            .find('"')
            .unwrap_or_else(|| panic!("dependency `{name}` version is unterminated"));
        version[..end].trim_matches(['[', ']']).to_string()
    };

    let foundation_version = dependency_version("Microsoft.WindowsAppSDK.Foundation");
    let interactive_version = dependency_version("Microsoft.WindowsAppSDK.InteractiveExperiences");
    let winui_version = dependency_version("Microsoft.WindowsAppSDK.WinUI");
    let interactive_root = nuget_package(
        "microsoft.windowsappsdk.interactiveexperiences",
        &interactive_version,
    )
    .join("metadata");
    let interactive = std::fs::read_dir(&interactive_root)
        .unwrap_or_else(|error| panic!("cannot read `{}`: {error}", interactive_root.display()))
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .max()
        .unwrap_or_else(|| {
            panic!(
                "no metadata subdirectory in `{}`",
                interactive_root.display()
            )
        });

    WindowsAppSdkMetadata {
        foundation: nuget_package("microsoft.windowsappsdk.foundation", &foundation_version)
            .join("metadata"),
        interactive,
        winui: nuget_package("microsoft.windowsappsdk.winui", &winui_version).join("metadata"),
    }
}

fn reactor_rdl() -> String {
    let version = helpers::read_str_const("crates/libs/reactor-setup/src/lib.rs", "RUNTIME_VER");
    let parts = version
        .split('.')
        .map(|part| part.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        matches!(parts.as_slice(), [_, _, _] | [_, _, _, _]),
        "Windows App SDK runtime version must have three or four components"
    );
    let major = parts[0];
    let minor = parts[1];
    let build = parts[2];
    let revision = parts.get(3).copied().unwrap_or(0);
    let major_minor = (major << 16) | minor;
    let packed = (major << 48) | (minor << 32) | (build << 16) | revision;

    std::fs::read_to_string(REACTOR_RDL)
        .unwrap()
        .replace("@VERSION@", &version)
        .replace("@MAJOR@", &major.to_string())
        .replace("@MINOR@", &minor.to_string())
        .replace("@BUILD@", &build.to_string())
        .replace("@REVISION@", &revision.to_string())
        .replace("@MAJOR_MINOR@", &major_minor.to_string())
        .replace("@PACKED_VERSION@", &packed.to_string())
}
