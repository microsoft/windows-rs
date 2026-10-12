use super::*;

const SDK: &str = "crates\\tools\\win32\\src\\main.rs";
const WDK: &str = "crates\\tools\\win32\\src\\km.rs";
const WEBVIEW: &str = "crates\\tools\\webview\\src\\main.rs";

pub struct Profile {
    pub name: String,
    pub family: &'static str,
    pub architecture: i32,
    pub source: String,
    pub arguments: Vec<String>,
    pub roots: Vec<PathBuf>,
    pub root_dirs: Vec<PathBuf>,
    pub excluded: Vec<PathBuf>,
}

pub struct Selection {
    pub family: String,
    pub arch: String,
    pub headers: Option<Vec<String>>,
}

impl Selection {
    pub fn parse(arguments: impl Iterator<Item = String>) -> Result<Self> {
        let mut result = Self {
            family: "all".into(),
            arch: "all".into(),
            headers: None,
        };
        let mut arguments = arguments;
        let mut seen = BTreeSet::new();
        while let Some(option) = arguments.next() {
            if !seen.insert(option.clone()) {
                return Err(format!("duplicate option `{option}`").into());
            }
            let value = arguments.next().ok_or_else(|| usage(&option))?;
            match option.as_str() {
                "--family" if matches!(value.as_str(), "all" | "sdk" | "wdk" | "webview") => {
                    result.family = value;
                }
                "--arch" if value == "all" || helpers::Arch::known(&value).is_some() => {
                    result.arch = value;
                }
                "--headers" => {
                    let headers = value.split(',').map(str::to_string).collect::<Vec<_>>();
                    if headers.iter().any(String::is_empty)
                        || headers.iter().collect::<BTreeSet<_>>().len() != headers.len()
                    {
                        return Err("header selection must contain distinct nonempty names".into());
                    }
                    result.headers = Some(headers);
                }
                _ => return Err(usage(&option).into()),
            }
        }
        if result.headers.is_some() && result.family != "sdk" {
            return Err("--headers requires --family sdk".into());
        }
        Ok(result)
    }

    pub fn output(&self) -> PathBuf {
        Path::new("target\\clang-sources").join(format!(
            "{}-{}{}",
            self.family,
            self.arch,
            if self.headers.is_some() {
                "-selected"
            } else {
                ""
            }
        ))
    }
}

fn usage(option: &str) -> String {
    format!(
        "invalid option `{option}`; usage: tool-clang sources \
         [--family all|sdk|wdk|webview] [--arch all|x64|x86|arm64] \
         [--headers sdk-header.h,...]"
    )
}

pub fn build(selection: &Selection) -> Result<Vec<Profile>> {
    let archs = strings(SDK, "ARCHS")?;
    let selected_archs = archs
        .iter()
        .filter(|arch| selection.arch == "all" || **arch == selection.arch)
        .collect::<Vec<_>>();
    if selected_archs.is_empty() {
        return Err("selected architecture is not in the configured manifest".into());
    }
    let sdk_version = pin(SDK, "SDK_VERSION")?;
    let sdk = helpers::nuget_package("microsoft.windows.sdk.cpp", &sdk_version)
        .join("c")
        .join("Include")
        .join(helpers::marketing_dir(&sdk_version));
    let sdk_dirs = ["ucrt", "um", "shared", "winrt", "cppwinrt"].map(|name| sdk.join(name));
    let resource = PathBuf::from(helpers::clang_resource_dir());
    if !resource.join("include").join("intrin.h").is_file() {
        return Err(format!(
            "missing compiler resource headers in `{}`",
            resource.display()
        )
        .into());
    }
    let mut profiles = Vec::new();
    for name in selected_archs {
        let arch = helpers::Arch::known(name).ok_or("unknown configured architecture")?;
        let common = vec![
            "-x".into(),
            "c++".into(),
            "-std=c++20".into(),
            "-ferror-limit=0".into(),
            "-fms-extensions".into(),
            format!("--target={}", arch.triple),
            "-resource-dir".into(),
            resource.to_string_lossy().into_owned(),
        ];
        if matches!(selection.family.as_str(), "sdk" | "all") {
            let configured = sdk_headers(SDK)?;
            let headers = selection.headers.as_ref().unwrap_or(&configured);
            for header in headers {
                if !configured.contains(header) {
                    return Err(format!("`{header}` is not in the SDK header manifest").into());
                }
            }
            let satellites = strings(SDK, "SATELLITE_HEADERS")?;
            let prelude = pin(SDK, "PRELUDE")?;
            let reset = pin(SDK, "GUID_RESET")?;
            let selected_satellite = headers.iter().any(|header| header == "devicetopology.h");
            for satellite in [false, true] {
                let headers = headers
                    .iter()
                    .filter(|header| satellites.contains(header) == satellite)
                    .collect::<Vec<_>>();
                if headers.is_empty() {
                    continue;
                }
                let mut source = prelude.clone();
                if satellite {
                    source.push_str(&reset);
                } else if selected_satellite {
                    source.push_str("\n#include <ks.h>");
                }
                let mut roots = Vec::new();
                for header in headers {
                    if satellite && header == "devicetopology.h" {
                        source.push_str("\n#include <ks.h>\n#define _KS_");
                    }
                    writeln!(source, "\n#include <{header}>")?;
                    if satellite {
                        source.push_str(&reset);
                    }
                    roots.push(resolve(header, &sdk_dirs)?);
                }
                let full = !satellite && selection.headers.is_none();
                let scope = strings(SDK, "SCOPE")?;
                let exclusions = strings(SDK, "EXCLUDE_HEADERS")?;
                let mut arguments = common.clone();
                includes(&mut arguments, &sdk_dirs)?;
                profiles.push(Profile {
                    name: format!(
                        "sdk-{}-{name}",
                        if satellite { "satellite" } else { "main" }
                    ),
                    family: "sdk",
                    architecture: arch.bits,
                    source,
                    arguments,
                    roots,
                    root_dirs: if full {
                        sdk_dirs
                            .iter()
                            .filter(|dir| {
                                dir.file_name().is_some_and(|name| {
                                    scope.iter().any(|scope| name == scope.as_str())
                                })
                            })
                            .cloned()
                            .collect()
                    } else {
                        Vec::new()
                    },
                    excluded: if full {
                        exclusions
                            .iter()
                            .filter_map(|header| find(header, &sdk_dirs))
                            .collect()
                    } else {
                        Vec::new()
                    },
                });
            }
        }
        if matches!(selection.family.as_str(), "wdk" | "all") {
            let wdk_version = pin(WDK, "WDK_VERSION")?;
            let wdk = helpers::nuget_package("microsoft.windows.wdk.x64", &wdk_version)
                .join("c")
                .join("Include")
                .join(helpers::marketing_dir(&wdk_version));
            let dirs = [
                wdk.join("km"),
                wdk.join("shared"),
                sdk.join("shared"),
                sdk.join("um"),
                sdk.join("ucrt"),
            ];
            let headers = strings(WDK, "SOURCE_HEADERS")?;
            let mut kernel = String::new();
            let mut roots = Vec::new();
            for header in &headers {
                if header != "offreg.h" {
                    writeln!(kernel, "#include <{header}>")?;
                    roots.push(resolve(header, &dirs)?);
                }
            }
            let mut arguments = common.clone();
            arguments.extend(strings(WDK, "CLANG_ARGS")?);
            arguments.extend(
                match name.as_str() {
                    "x64" => &["-D_AMD64_", "-DAMD64", "-D_WIN64"][..],
                    "arm64" => &["-D_ARM64_", "-DARM64", "-D_WIN64"][..],
                    "x86" => &["-D_X86_", "-Di386=1"][..],
                    _ => return Err("unknown WDK target".into()),
                }
                .iter()
                .map(|value| value.to_string()),
            );
            includes(&mut arguments, &dirs)?;
            profiles.push(Profile {
                name: format!("wdk-kernel-{name}"),
                family: "wdk",
                architecture: arch.bits,
                source: kernel,
                arguments,
                roots,
                root_dirs: vec![wdk.join("km")],
                excluded: Vec::new(),
            });
            if headers.iter().any(|header| header == "offreg.h") {
                let mut arguments = common.clone();
                includes(&mut arguments, &sdk_dirs)?;
                includes(&mut arguments, &[wdk.join("km")])?;
                profiles.push(Profile {
                    name: format!("wdk-offreg-{name}"),
                    family: "wdk",
                    architecture: arch.bits,
                    source: format!("{}\n#include <offreg.h>\n", pin(SDK, "PRELUDE")?),
                    arguments,
                    roots: vec![resolve("offreg.h", &dirs)?],
                    root_dirs: Vec::new(),
                    excluded: Vec::new(),
                });
            }
        }
        if matches!(selection.family.as_str(), "webview" | "all") {
            let version = pin(WEBVIEW, "WEBVIEW2_VERSION")?;
            let package = helpers::nuget_package("microsoft.web.webview2", &version)
                .join("build")
                .join("native");
            for (directory, header) in [
                ("include", "WebView2.h"),
                ("include-winrt", "WebView2Interop.h"),
            ] {
                let root = package.join(directory).join(header);
                if !root.is_file() {
                    return Err(format!("missing WebView2 input `{}`", root.display()).into());
                }
                let mut arguments = common.clone();
                includes(&mut arguments, &[package.join("include")])?;
                includes(&mut arguments, &sdk_dirs)?;
                profiles.push(Profile {
                    name: format!("webview-{}-{name}", header.trim_end_matches(".h")),
                    family: "webview",
                    architecture: arch.bits,
                    source: format!(
                        "#include \"{}\"\n",
                        root.to_string_lossy().replace('\\', "\\\\")
                    ),
                    arguments,
                    roots: vec![root],
                    root_dirs: Vec::new(),
                    excluded: Vec::new(),
                });
            }
        }
    }
    Ok(profiles)
}

fn includes(arguments: &mut Vec<String>, dirs: &[PathBuf]) -> Result<()> {
    for dir in dirs {
        if !dir.is_dir() {
            return Err(format!("missing include directory `{}`", dir.display()).into());
        }
        arguments.extend(["-isystem".into(), dir.to_string_lossy().into_owned()]);
    }
    Ok(())
}

fn find(header: &str, dirs: &[PathBuf]) -> Option<PathBuf> {
    dirs.iter()
        .map(|dir| dir.join(header))
        .find(|path| path.is_file())
}

fn resolve(header: &str, dirs: &[PathBuf]) -> Result<PathBuf> {
    find(header, dirs).ok_or_else(|| format!("missing configured header `{header}`").into())
}

fn strings(path: &str, name: &str) -> Result<Vec<String>> {
    let file = syn::parse_file(&std::fs::read_to_string(path)?)?;
    for item in file.items {
        if let syn::Item::Const(item) = item
            && item.ident == name
        {
            let expression = match &*item.expr {
                syn::Expr::Reference(reference) => &*reference.expr,
                expression => expression,
            };
            let syn::Expr::Array(array) = expression else {
                return Err(format!("manifest `{name}` in `{path}` is not a literal array").into());
            };
            return array
                .elems
                .iter()
                .map(|expression| {
                    if let syn::Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Str(value),
                        ..
                    }) = expression
                    {
                        Ok(value.value())
                    } else {
                        Err(format!("nonliteral member in `{name}` in `{path}`").into())
                    }
                })
                .collect();
        }
    }
    Err(format!("missing manifest `{name}` in `{path}`").into())
}

fn sdk_headers(path: &str) -> Result<Vec<String>> {
    let mut headers = strings(path, "HEADERS")?;
    headers.extend(strings(path, "SATELLITE_HEADERS")?);
    let mut seen = BTreeSet::new();
    for header in &headers {
        if !seen.insert(header) {
            return Err(format!("duplicate SDK manifest header `{header}`").into());
        }
    }
    Ok(headers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selections_are_explicit_and_validated() {
        let selection = Selection::parse(
            ["--family", "sdk", "--arch", "x86", "--headers", "a.h,b.h"]
                .into_iter()
                .map(str::to_string),
        )
        .unwrap();
        assert_eq!(selection.headers.unwrap(), ["a.h", "b.h"]);
        for arguments in [
            vec!["--arch", "mips"],
            vec!["--headers", "a.h"],
            vec!["--family", "sdk", "--headers", "a.h,a.h"],
            vec!["--family", "sdk", "--headers", "a.h,"],
            vec!["--arch"],
            vec!["--arch", "x64", "--arch", "x86"],
        ] {
            assert!(Selection::parse(arguments.into_iter().map(str::to_string)).is_err());
        }
    }

    #[test]
    fn reads_actual_generator_manifests() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..\\win32\\src\\main.rs");
        let headers = strings(root.to_str().unwrap(), "HEADERS").unwrap();
        assert!(headers.contains(&"shellscalingapi.h".to_string()));
        assert!(headers.contains(&"tlhelp32.h".to_string()));
        let satellites = strings(root.to_str().unwrap(), "SATELLITE_HEADERS").unwrap();
        assert!(satellites.contains(&"devicetopology.h".to_string()));
        let combined = sdk_headers(root.to_str().unwrap()).unwrap();
        assert_eq!(combined.len(), headers.len() + satellites.len());
        assert!(satellites.iter().all(|header| combined.contains(header)));
        assert_eq!(
            strings(root.to_str().unwrap(), "ARCHS").unwrap(),
            ["x64", "arm64", "x86"]
        );
    }
}
