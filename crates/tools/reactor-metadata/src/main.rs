use helpers::nuget_package;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

mod assets;
#[path = "../../../libs/reactor-setup/src/runtime.rs"]
mod runtime;

const WINMD: &str = "crates/tools/reactor-metadata/winmd";
const EXTRAS_RDL: &str = "crates/tools/reactor-metadata/src/extras.rdl";
const EXTRAS_WINMD: &str = "crates/tools/reactor-metadata/winmd/extras.winmd";
const SETUP_VERSION: &str = "crates/libs/reactor-setup/assets/runtime-version.txt";
const WINDOWS_APP_SDK_VERSION: &str = "2.5.1";

fn main() {
    let umbrella = nuget_package("microsoft.windowsappsdk", WINDOWS_APP_SDK_VERSION);
    let nuspec = read_nuspec(&umbrella);
    let runtime_version = nuspec_dependency_version(&nuspec, "Microsoft.WindowsAppSDK.Runtime");
    let runtime = nuget_package("microsoft.windowsappsdk.runtime", &runtime_version);
    let header_path = runtime.join("include/WindowsAppSDK-VersionInfo.h");
    let header = fs::read_to_string(&header_path)
        .unwrap_or_else(|error| panic!("cannot read `{}`: {error}", header_path.display()));

    refresh_winmd(&nuspec);
    generate_extras(&header);
    assets::generate(&nuspec, &runtime, &header);
    fs::write(workspace_path(SETUP_VERSION), runtime_version)
        .unwrap_or_else(|error| panic!("cannot write `{SETUP_VERSION}`: {error}"));
}

fn workspace_path(path: impl AsRef<Path>) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(path)
}

fn generate_extras(header: &str) {
    windows_rdl::Reader::new()
        .input(workspace_path(EXTRAS_RDL))
        .input_text(&runtime_version_rdl(header))
        .reference_bytes(windows_default::WIN32)
        .output(workspace_path(EXTRAS_WINMD))
        .write()
        .unwrap();
}

fn runtime_version_rdl(header: &str) -> String {
    let mut rdl = String::from("#[win32]\nmod extras {\n");
    for (suffix, ty) in [
        ("MAJOR", "u32"),
        ("MINOR", "u32"),
        ("BUILD", "u32"),
        ("REVISION", "u32"),
        ("UINT64", "u64"),
    ] {
        let name = format!("WINDOWSAPPSDK_RUNTIME_VERSION_{suffix}");
        let value = numeric_define(header, &name);
        writeln!(rdl, "    const {name}: {ty} = {value};").unwrap();
    }
    rdl.push_str("}\n");
    rdl
}

fn numeric_define(header: &str, name: &str) -> u64 {
    let literal = define_literal(header, name);
    let literal = literal.strip_suffix('u').unwrap_or(literal);
    if let Some(hex) = literal.strip_prefix("0x") {
        u64::from_str_radix(hex, 16)
    } else {
        literal.parse()
    }
    .unwrap_or_else(|error| panic!("invalid version define `{name}`: {error}"))
}

fn define_literal<'a>(header: &'a str, name: &str) -> &'a str {
    for line in header.lines() {
        let mut words = line.split_whitespace();
        if words.next() != Some("#define") || words.next() != Some(name) {
            continue;
        }
        let literal = words
            .next()
            .unwrap_or_else(|| panic!("empty version define `{name}`"));
        assert!(
            words.next().is_none(),
            "expected a single literal for `{name}`"
        );
        return literal;
    }
    panic!("missing version define `{name}`");
}

fn refresh_winmd(nuspec: &str) {
    let foundation = nuspec_dependency_version(nuspec, "Microsoft.WindowsAppSDK.Foundation");
    let interactive =
        nuspec_dependency_version(nuspec, "Microsoft.WindowsAppSDK.InteractiveExperiences");
    let winui = nuspec_dependency_version(nuspec, "Microsoft.WindowsAppSDK.WinUI");

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
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.extension()
                .is_some_and(|extension| extension.eq_ignore_ascii_case("nuspec"))
        })
        .unwrap_or_else(|| panic!("no `.nuspec` in `{}`", package_dir.display()));
    fs::read_to_string(&nuspec)
        .unwrap_or_else(|error| panic!("cannot read `{}`: {error}", nuspec.display()))
}

fn nuspec_dependency_version(nuspec: &str, dependency_id: &str) -> String {
    let document = roxmltree::Document::parse(nuspec).unwrap();
    let dependency = document
        .descendants()
        .find(|node| node.has_tag_name("dependency") && node.attribute("id") == Some(dependency_id))
        .unwrap_or_else(|| panic!("nuspec has no dependency `{dependency_id}`"));
    let version = dependency
        .attribute("version")
        .unwrap_or_else(|| panic!("dependency `{dependency_id}` has no version"));
    let version = version
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .unwrap_or(version);
    assert!(
        !version.is_empty()
            && version
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '+')),
        "dependency `{dependency_id}` must specify a version, not a range: `{version}`"
    );
    version.to_string()
}

fn newest_subdir(dir: &Path) -> PathBuf {
    fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("cannot read `{}`: {error}", dir.display()))
        .filter_map(|entry| {
            let entry = entry.unwrap();
            entry.file_type().unwrap().is_dir().then(|| entry.path())
        })
        .max_by_key(|path| metadata_version(path.file_name().unwrap().to_str().unwrap()))
        .unwrap_or_else(|| panic!("no metadata subdirectory in `{}`", dir.display()))
}

fn metadata_version(name: &str) -> [u32; 4] {
    name.split('.')
        .map(|part| {
            part.parse()
                .unwrap_or_else(|error| panic!("invalid metadata version `{name}`: {error}"))
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap_or_else(|_| panic!("metadata version must have four components: `{name}`"))
}

fn copy_winmd(source: &Path, destination: &Path) {
    for path in tool_reactor_metadata::winmd_paths(source) {
        let name = path.file_name().unwrap();
        fs::copy(&path, destination.join(name))
            .unwrap_or_else(|error| panic!("cannot copy `{}`: {error}", path.display()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_versions_sort_numerically() {
        assert!(metadata_version("10.0.10000.0") > metadata_version("10.0.9000.0"));
        assert!(metadata_version("10.0.18362.0") > metadata_version("10.0.17763.0"));
        for invalid in ["", "10.0.1", "10.0.1.0.0", "10.0.latest.0"] {
            assert!(std::panic::catch_unwind(|| metadata_version(invalid)).is_err());
        }
    }

    #[test]
    fn runtime_dependency_version() {
        let nuspec = r#"<dependencies>
            <dependency id="Microsoft.WindowsAppSDK.WinUI" version="3.4.5" />
            <dependency id="Microsoft.WindowsAppSDK.Runtime" version="[6.7.8]" />
        </dependencies>"#;
        assert_eq!(
            nuspec_dependency_version(nuspec, "Microsoft.WindowsAppSDK.Runtime"),
            "6.7.8"
        );
    }

    #[test]
    fn dependency_versions_are_bound_to_their_element() {
        assert_eq!(
            nuspec_dependency_version("<dependency version='[1.2.3]' id='Runtime' />", "Runtime"),
            "1.2.3"
        );
        for text in [
            "<dependencies><dependency id='Runtime'/><dependency id='Other' version='1.2.3'/></dependencies>",
            "<dependency id='Runtime' version='[1.0,2.0)'/>",
            "<dependency id='Runtime' version=''/>",
        ] {
            assert!(
                std::panic::catch_unwind(|| nuspec_dependency_version(text, "Runtime")).is_err()
            );
        }
    }

    #[test]
    fn runtime_version_defines() {
        let header = "#define OTHER 99u\n#define VERSION_MAJOR 12u\n\
                      \t#define\tVERSION\t0x000C00220038004Eu\n";
        assert_eq!(numeric_define(header, "VERSION_MAJOR"), 12);
        assert_eq!(numeric_define(header, "VERSION"), 0x000C_0022_0038_004E);
        assert_eq!(numeric_define("#define VERSION 0", "VERSION"), 0);
    }

    #[test]
    fn invalid_runtime_version_defines() {
        for header in [
            "",
            "#define VERSION_MAJOR 1u",
            "#define VERSION",
            "#define VERSION not_a_number",
            "#define VERSION 1u + 2u",
            "#define VERSION 18446744073709551616u",
        ] {
            assert!(std::panic::catch_unwind(|| numeric_define(header, "VERSION")).is_err());
        }
    }
}
