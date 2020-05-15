extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use winmd::{TypeLimits, TypeReader, TypeStage};

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// A macro for generating WinRT modules into the current module
#[proc_macro]
pub fn import(stream: TokenStream) -> TokenStream {
    let (dependencies, namespaces) = parse_import_stream(stream);

    let dependencies = dependencies
        .into_iter()
        .map(|p| winmd::WinmdFile::new(p))
        .collect();
    let reader = &TypeReader::new(dependencies);

    let mut limits = TypeLimits::default();

    for namespace in namespaces {
        limits.insert(reader, &namespace);
    }

    let stage = TypeStage::from_limits(reader, &limits);
    let tree = stage.into_tree();
    let stream = tree.to_tokens();
    //std::fs::write(r"c:\git\rust\dump.rs", stream.to_string()).unwrap();
    stream.into()
}

#[derive(PartialEq)]
enum ImportCategory {
    Dependency,
    Namespace,
}

#[derive(PartialEq, Clone, Copy)]
enum ParseState {
    Neither,
    ParsedNamespace,
    ParsedDependency,
    Both,
}
impl ParseState {
    fn parsed_namespace(self) -> Self {
        match self {
            ParseState::Neither => ParseState::ParsedNamespace,
            ParseState::ParsedDependency => ParseState::Both,
            _ => self,
        }
    }
    fn parsed_dependency(self) -> Self {
        match self {
            ParseState::Neither => ParseState::ParsedDependency,
            ParseState::ParsedNamespace => ParseState::Both,
            _ => self,
        }
    }
}

/// Parse `import!` macro and return a set of paths to dependencies and
/// a set to all the namespaces referenced
fn parse_import_stream(stream: TokenStream) -> (BTreeSet<PathBuf>, BTreeSet<String>) {
    let mut dependencies = BTreeSet::<PathBuf>::new();
    let mut modules = BTreeSet::<String>::new();
    let mut stream = stream.into_iter().peekable();
    let mut state = ParseState::Neither;

    loop {
        if state == ParseState::Both {
            let next = stream.next();
            assert!(
                next.is_none(),
                "Unexpected input at the end of the winrt::import: '{}'",
                next.unwrap()
            );
            break;
        }
        let category = parse_category(&mut stream);
        match category {
            ImportCategory::Namespace => {
                modules.extend(parse_namespace(&mut stream));
                state = state.parsed_namespace();
            }
            ImportCategory::Dependency => {
                dependencies.extend(parse_dependencies(&mut stream));
                state = state.parsed_dependency();
            }
        }
    }

    (dependencies, modules)
}

fn parse_category(
    stream: &mut std::iter::Peekable<impl std::iter::Iterator<Item = TokenTree>>,
) -> ImportCategory {
    let token = stream.next().expect(
        "Unexpected end of winrt::import macro. Expected either `dependencies` or `modules`",
    );
    match token {
        TokenTree::Ident(value) => {
            let category = match value.to_string().as_str() {
                "dependencies" => ImportCategory::Dependency,
                "modules" => ImportCategory::Namespace,
                value => panic!(
                    "winrt::import macro expects either `dependencies` or `modules` but found `{}`",
                    value
                ),
            };
            if let Some(TokenTree::Punct(p)) = stream.peek() {
                if p.as_char() == ':' {
                    let _ = stream.next();
                }
            }
            category
        }
        _ => {
            panic!(
                "winrt::import macro encountered an unrecognized token: '{}'. Expected `dependencies` or `modules`",
                token
            );
        }
    }
}

fn parse_namespace(
    stream: &mut std::iter::Peekable<impl std::iter::Iterator<Item = TokenTree>>,
) -> BTreeSet<String> {
    let mut modules = BTreeSet::<String>::new();
    loop {
        let token = stream.peek();
        match token {
            Some(TokenTree::Literal(value)) => {
                modules.insert(namespace_literal_to_rough_namespace(&value.to_string()));
                let _ = stream.next();
            }
            _ => break,
        }
    }
    modules
}

fn parse_dependencies(
    stream: &mut std::iter::Peekable<impl std::iter::Iterator<Item = TokenTree>>,
) -> BTreeSet<PathBuf> {
    let mut dependencies = BTreeSet::<PathBuf>::new();

    loop {
        let token = stream.peek();
        match token {
            Some(TokenTree::Literal(value)) => {
                expand_paths(
                    value.to_string().trim_matches('"'),
                    &mut dependencies,
                    false,
                );
                let _literal = stream.next();
            }
            Some(TokenTree::Ident(value)) if value.to_string().as_str() == "os" => {
                let mut path = PathBuf::new();
                let wind_dir_env = std::env::var("windir")
                    .unwrap_or_else(|_| panic!("No `windir` environment variable found"));
                path.push(wind_dir_env);
                path.push(SYSTEM32);
                path.push("winmetadata");
                expand_paths(path, &mut dependencies, false);
                let _os = stream.next();
            }
            Some(TokenTree::Ident(value)) if value.to_string().as_str() == "nuget" => {
                let _nuget = stream.next();
                let colon = stream.next();
                assert!(
                    match colon {
                        Some(TokenTree::Punct(value)) if value.as_char() == ':' => true,
                        _ => false,
                    },
                    "`nuget` must be followed by a `:`"
                );
                let mut path = workspace_root();

                path.push("target");
                path.push("nuget");

                let mut name = String::new();
                while {
                    match stream.next() {
                        Some(TokenTree::Ident(value)) => name.push_str(&value.to_string()),
                        Some(_) => panic!("Unexpected input: a period seperated list of indentifiers must follow `nuget:`"),
                        None => panic!("Unexpected end of input: a nuget package name must follow `nuget:`"),
                    };
                    match stream.peek() {
                        Some(TokenTree::Punct(value)) if value.as_char() == '.' => true,
                        _ => false,
                    }
                } {
                    let _period = stream.next();
                    name.push('.');
                }
                path.push(name);

                expand_paths(path, &mut dependencies, true);
            }
            _ => break,
        }
    }
    dependencies
}

/// Returns the paths to resolved dependencies
fn expand_paths<P: AsRef<Path>>(dependency: P, result: &mut BTreeSet<PathBuf>, recurse: bool) {
    let path = dependency.as_ref();

    if path.is_dir() {
        let paths = std::fs::read_dir(path).unwrap_or_else(|e| {
            panic!(
                "Could not read dependecy directory at path {:?}: {}",
                path, e
            )
        });
        for path in paths {
            let path = path.expect("Could not read directory entry");
            let path = path.path();
            if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("winmd")) {
                result.insert(path);
            } else if path.is_dir() && recurse {
                expand_paths(path, result, recurse)
            }
        }
    } else if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("winmd")) {
        result.insert(path.to_path_buf());
    } else {
        panic!("Dependency {} is not a file or directory", path.display());
    }
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

// Snake <-> camel casing is lossy so we go for character but not case conversion
// and deal with casing once we have an index of namespaces to compare against.
fn namespace_literal_to_rough_namespace(namespace: &str) -> String {
    let mut result = String::with_capacity(namespace.len());
    for c in namespace.chars() {
        if c != '"' && c != '_' {
            result.extend(c.to_lowercase());
        }
    }
    result
}

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
