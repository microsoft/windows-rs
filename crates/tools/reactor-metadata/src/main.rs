use helpers::nuget_package;
use std::fs;
use std::path::{Path, PathBuf};

const WINMD: &str = "crates/tools/reactor-metadata/winmd";
const EXTRAS_RDL: &str = "crates/tools/reactor-metadata/src/extras.rdl";
const EXTRAS_WINMD: &str = "crates/tools/reactor-metadata/winmd/extras.winmd";
const WINDOWS_APP_SDK_VERSION: &str = "2.5.1";

fn main() {
    assert_runtime_pins();
    refresh_winmd();
    generate_extras();
}

fn workspace_path(path: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(path)
}

fn assert_runtime_pins() {
    const REACTOR_SETUP: &str = "crates/libs/reactor-setup/src/lib.rs";

    let runtime_ver = helpers::read_str_const(workspace_path(REACTOR_SETUP), "RUNTIME_VER");
    assert_eq!(
        runtime_ver, WINDOWS_APP_SDK_VERSION,
        "Windows App SDK pin drift: `tool-reactor-metadata` refreshes \
         `{WINDOWS_APP_SDK_VERSION}` metadata but `windows-reactor-setup` stages `{runtime_ver}`. \
         Update `WINDOWS_APP_SDK_VERSION` in this tool and `RUNTIME_VER` in {REACTOR_SETUP} \
         together."
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

    let dir = workspace_path(WINMD);
    for entry in fs::read_dir(&dir).unwrap_or_else(|error| panic!("cannot read `{WINMD}`: {error}"))
    {
        let path = entry.unwrap().path();
        let is_winmd = path
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("winmd"));
        if is_winmd
            && !matches!(
                path.file_name().and_then(|name| name.to_str()),
                Some("extras.winmd" | "Microsoft.Web.WebView2.Core.winmd")
            )
        {
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
