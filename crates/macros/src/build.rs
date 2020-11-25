use crate::*;
use rayon::iter::ParallelIterator;
use syn::parse::{self, Parse, ParseStream};
use winrt_deps::cargo;
use winrt_gen::{NamespaceTypes, TypeLimit, TypeLimits, TypeTree};
use std::{collections::BTreeSet, path::Path, path::PathBuf};
use std::io;

/// A parsed `build!` macro
pub struct BuildMacro {
    dependencies: Dependencies,
    types: BuildLimits,
}

impl BuildMacro {
    pub fn winmd_paths(&self) -> &BTreeSet<PathBuf> {
        &self.dependencies.0
    }

    pub fn to_tokens_string(self) -> Result<String, proc_macro2::TokenStream> {
        let foundation = cargo::package_manifest().unwrap().package_name() == "winrt";
        println!("to_tokens_string: {}", cargo::package_manifest().unwrap().package_name());

        let reader = &if foundation {
            let files = std::fs::read_dir("winmds")
                .unwrap()
                .filter_map(|value| value.ok())
                .map(|value| value.path());

            winmd::TypeReader::from_iter(files)
        } else {
            // TODO: should be TypeReader::from_build() shared by build/implements macro
            winmd::TypeReader::from_iter(self.dependencies.0)
        };

        let mut limits = TypeLimits::new(reader);

        let foundation_namespaces = &[
            "Windows.Foundation",
            "Windows.Foundation.Collections",
            "Windows.Foundation.Diagnostics",
            "Windows.Foundation.Numerics",
        ];

        if foundation {
            for namespace in foundation_namespaces {
                limits
                    .insert(NamespaceTypes {
                        namespace: namespace.to_string(),
                        limit: TypeLimit::All,
                    })
                    .unwrap();
            }
        }

        for limit in self.types.0 {
            let types = limit.types;
            let syntax = limit.syntax;
            if let Err(e) = limits.insert(types).map_err(|ns| {
                syn::Error::new_spanned(syntax, format!("'{}' is not a known namespace", ns))
            }) {
                return Err(e.to_compile_error());
            };
        }

        let mut tree = TypeTree::from_limits(reader, &limits);

        if !foundation {
            for namespace in foundation_namespaces {
                tree.remove(namespace);
            }

            tree.reexport();
        }

        let ts = tree.gen().reduce(squote::TokenStream::new, |mut accum, n| {
            accum.combine(&n);
            accum
        });

        Ok(ts.into_string())
    }
}

impl Parse for BuildMacro {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let foundation = cargo::package_manifest().unwrap().package_name() == "winrt";

        let dependencies = if foundation {
            Dependencies::default()
        } else {
            Dependencies::parse()
                .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), format!("{}", e)))?
        };

        let types: BuildLimits = input.parse()?;

        Ok(BuildMacro {
            dependencies,
            types,
        })
    }
}

/// A parsed `dependencies` section of the `build!` macro
#[derive(Debug, Default)]
struct Dependencies(BTreeSet<PathBuf>);

impl Dependencies {
    fn parse() -> Result<Self, Box<dyn std::error::Error + 'static>> {
        let mut dependencies = BTreeSet::new();
        let deps = cargo::package_manifest()?.get_dependencies()?;
        for dep in deps {
            let nuget_path = std::fs::read_dir(nuget_root()).map_err(|e| format!("Did you forget to run `cargo winrt install`? Could not read nuget directory: {}", e))?;
            let name = dep.name();
            let mut dependency_path_iter = nuget_path
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_dir())
                .filter(|entry| {
                    if let Some(entry) = entry.file_name().to_str().map(str::to_owned) {
                        entry.starts_with(&name)
                    } else {
                        false
                    }
                })
                .map(|entry| entry.path());

            let path = dependency_path_iter
                .next()
                .ok_or_else(|| format!("No directory for dependency '{}'", dep.name()))?;
            // Check for multiple versions
            if dependency_path_iter.next().is_some() {
                return Err(format!("multiple nuget package versions found for '{}'", name).into());
            }

            expand_paths(path, &mut dependencies, true)
                .map_err(|e| format!("could not read dependency: {}", e))?;
        }
        Ok(Dependencies(dependencies))
    }
}

/// Returns the paths to resolved dependencies
fn expand_paths<P: AsRef<Path>>(
    dependency: P,
    result: &mut BTreeSet<PathBuf>,
    recurse: bool,
) -> io::Result<()> {
    let path = dependency.as_ref();

    if path.is_dir() {
        let paths = std::fs::read_dir(path)?;
        for path in paths {
            let path = path.expect("Could not read directory entry");
            let path = path.path();
            if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("winmd")) {
                result.insert(path);
            } else if path.is_dir() && recurse {
                expand_paths(path, result, recurse)?
            }
        }
        Ok(())
    } else if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("winmd")) {
        result.insert(path.to_path_buf());
        Ok(())
    } else if !path.exists() {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("dependency path '{}' does not exist", path.display()),
        ))
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "dependency path '{}' is not a directory or winmd file",
                path.display()
            ),
        ))
    }
}

fn nuget_root() -> PathBuf {
    let mut path = workspace_root();
    path.push("target");
    path.push("nuget");
    path
}

fn workspace_root() -> PathBuf {
    let output = std::process::Command::new("cargo")
        .args(&["metadata"])
        .output()
        .expect("Could not run `cargo metadata`")
        .stdout;
    let value: serde_json::Map<String, serde_json::Value> =
        serde_json::from_slice(&output).expect("`cargo metadata` did not return json.");
    let path = match value.get("workspace_root") {
        Some(serde_json::Value::String(s)) => s,
        _ => panic!("`cargo metadata` json was not in expected format"),
    };
    PathBuf::from(path)
}
