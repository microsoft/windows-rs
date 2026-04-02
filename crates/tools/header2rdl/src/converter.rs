/// Top-level orchestration: parse C/C++ headers with Clang, collect IR, emit RDL.
use std::collections::HashSet;
use std::path::Path;

use crate::collect::collect;
use crate::emit::emit_rdl;

/// Configuration for a single conversion run.
pub struct Config {
    /// Input header file paths.
    pub headers: Vec<String>,
    /// Dot-separated RDL namespace, e.g. `Windows.Win32.System.Threading`.
    pub namespace: String,
    /// DLL name used for `#[library("...")]` on function declarations.
    pub library: String,
    /// Additional include directories passed to Clang.
    pub include_dirs: Vec<String>,
    /// Preprocessor macro definitions, e.g. `["WIN32", "NDEBUG=1"]`.
    pub defines: Vec<String>,
    /// Target architecture triple, e.g. `x86_64-pc-windows-msvc`.
    pub arch_triple: String,
    /// Output directory for `.rdl` files.
    pub output_dir: String,
    /// When true, emit one file per top-level namespace component.
    pub split: bool,
    /// When true, parse headers as C++ (enables `class`, `virtual`, `__uuid`, etc.).
    pub cpp: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            headers: Vec::new(),
            namespace: String::new(),
            library: String::new(),
            include_dirs: Vec::new(),
            defines: Vec::new(),
            arch_triple: "x86_64-pc-windows-msvc".to_string(),
            output_dir: ".".to_string(),
            split: false,
            cpp: false,
        }
    }
}

/// Run the full conversion pipeline for the given `config`.
///
/// # Errors
///
/// Returns a string describing the first error encountered.
pub fn convert(config: &Config) -> Result<(), String> {
    let clang = clang::Clang::new().map_err(|e| format!("failed to load libclang: {e}"))?;
    let index = clang::Index::new(&clang, false, true);

    // Build the set of canonical source paths so the collector can filter to them.
    let source_paths: HashSet<String> = config
        .headers
        .iter()
        .filter_map(|h| {
            std::fs::canonicalize(h)
                .ok()
                .map(|p| p.to_string_lossy().replace('\\', "/"))
        })
        .collect();

    if source_paths.is_empty() {
        return Err("no valid header files found".to_string());
    }

    // Build Clang arguments.
    let mut clang_args: Vec<String> = vec![
        "-x".to_string(),
        if config.cpp { "c++-header" } else { "c-header" }.to_string(),
        format!("--target={}", config.arch_triple),
        "-fms-extensions".to_string(),
        "-fms-compatibility".to_string(),
        "-fms-compatibility-version=19".to_string(),
        "-D_WIN32".to_string(),
        "-DWIN32".to_string(),
        "-D_WIN64".to_string(),
        "-DUNICODE".to_string(),
        "-D_UNICODE".to_string(),
        // Suppress SAL macro redefinition warnings.
        "-Wno-macro-redefined".to_string(),
        "-Wno-ignored-attributes".to_string(),
    ];
    for dir in &config.include_dirs {
        clang_args.push(format!("-I{dir}"));
    }
    for def in &config.defines {
        clang_args.push(format!("-D{def}"));
    }

    // When there are multiple headers, create a synthetic "umbrella" header that
    // `#include`s all of them.  This lets Clang see all declarations in one TU.
    let (tu_path, _tmp) = if config.headers.len() == 1 {
        (config.headers[0].clone(), None)
    } else {
        let tmp = build_umbrella_header(&config.headers)?;
        let path = tmp.clone();
        (path, Some(tmp))
    };

    let args_refs: Vec<&str> = clang_args.iter().map(String::as_str).collect();

    let tu = index
        .parser(&tu_path)
        .arguments(&args_refs)
        .skip_function_bodies(true)
        .parse()
        .map_err(|e| format!("failed to parse {tu_path}: {e:?}"))?;

    // Report any Clang diagnostics at error/fatal severity.
    for diag in tu.get_diagnostics() {
        use clang::diagnostic::Severity;
        match diag.get_severity() {
            Severity::Error | Severity::Fatal => {
                eprintln!("clang: {}", diag.get_text());
            }
            _ => {}
        }
    }

    let items = collect(tu.get_entity(), &source_paths, &config.library);

    if items.is_empty() {
        eprintln!(
            "warning: no items collected from the provided headers; check that the paths are correct"
        );
    }

    let rdl = emit_rdl(&config.namespace, &items);

    // Determine output file path.
    let output_path = build_output_path(&config.output_dir, &config.namespace, config.split);
    write_output(&output_path, &rdl)?;

    println!("wrote {output_path}");
    Ok(())
}

/// Build a temporary umbrella header that `#include`s all given headers.
/// Returns the path to the temp file.
fn build_umbrella_header(headers: &[String]) -> Result<String, String> {
    use std::io::Write;

    let dir = std::env::temp_dir();
    let path = dir.join("__header2rdl_umbrella.h");
    let mut f = std::fs::File::create(&path)
        .map_err(|e| format!("failed to create umbrella header: {e}"))?;
    for h in headers {
        let abs = std::fs::canonicalize(h).unwrap_or_else(|_| Path::new(h).to_path_buf());
        writeln!(
            f,
            "#include \"{}\"",
            abs.to_string_lossy().replace('\\', "/")
        )
        .map_err(|e| format!("failed to write umbrella header: {e}"))?;
    }
    Ok(path.to_string_lossy().into_owned())
}

/// Compute the output `.rdl` file path.
fn build_output_path(output_dir: &str, namespace: &str, split: bool) -> String {
    let stem = if namespace.is_empty() {
        "out".to_string()
    } else if split {
        // Use just the first component of the namespace.
        namespace.split('.').next().unwrap_or("out").to_string()
    } else {
        namespace.replace('.', "_")
    };

    format!("{output_dir}/{stem}.rdl")
}

fn write_output(path: &str, content: &str) -> Result<(), String> {
    // Create parent directory if needed.
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("failed to create output directory: {e}"))?;
        }
    }
    std::fs::write(path, content).map_err(|e| format!("failed to write {path}: {e}"))
}
