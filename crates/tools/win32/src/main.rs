mod manifest;

use manifest::Manifest;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let opts = parse_args(&args);

    let text = std::fs::read_to_string(&opts.manifest).unwrap_or_else(|e| {
        eprintln!("error: failed to read `{}`: {e}", opts.manifest);
        std::process::exit(1);
    });

    let manifest: Manifest = toml::from_str(&text).unwrap_or_else(|e| {
        eprintln!("error: failed to parse `{}`: {e}", opts.manifest);
        std::process::exit(1);
    });

    // Resolve include-path flags once so they can be appended to every
    // per-namespace clang args list.
    let include_flags: Vec<String> = opts
        .includes
        .iter()
        .map(|inc| format!("-I{inc}"))
        .collect();

    // The reference .winmd (either from the manifest or from the CLI override)
    // that is passed to Clang and Reader for cross-namespace type resolution.
    let reference: Option<String> = opts.reference.or_else(|| manifest.reference.clone());

    std::fs::create_dir_all(&opts.output).unwrap_or_else(|e| {
        eprintln!("error: failed to create output directory `{}`: {e}", opts.output);
        std::process::exit(1);
    });

    let mut winmd_paths: Vec<String> = Vec::new();
    let mut errors: Vec<(String, String)> = Vec::new();

    for ns in &manifest.namespace {
        // Build the combined clang args: top-level + per-namespace + include paths.
        let mut clang_args: Vec<String> = manifest.args.clone();
        clang_args.extend_from_slice(&ns.args);
        clang_args.extend_from_slice(&include_flags);

        let rdl_path = format!("{}/{}.rdl", opts.output, ns.name);
        let winmd_path = format!("{}/{}.winmd", opts.output, ns.name);

        // Step 1: Clang → RDL
        let clang_result = run_clang(ns, &clang_args, reference.as_deref(), &rdl_path);
        if handle_step_error(&ns.name, clang_result, opts.keep_going, &mut errors) {
            continue;
        }

        // Step 2: RDL → winmd
        let reader_result = run_reader(&rdl_path, reference.as_deref(), &winmd_path);
        if handle_step_error(&ns.name, reader_result, opts.keep_going, &mut errors) {
            continue;
        }

        winmd_paths.push(winmd_path);
    }

    if !errors.is_empty() {
        eprintln!("{} namespace(s) failed:", errors.len());
        for (name, msg) in &errors {
            eprintln!("  {name}: {msg}");
        }
    }

    // Step 3: merge all per-namespace winmds into one combined file.
    if !winmd_paths.is_empty() {
        let stem = opts
            .merge_output
            .as_deref()
            .unwrap_or("Windows.Win32.winmd");
        let merged = format!("{}/{stem}", opts.output);

        if let Err(e) = windows_metadata::merge()
            .inputs(winmd_paths)
            .output(&merged)
            .merge()
        {
            eprintln!("error: merge failed: {e}");
            std::process::exit(1);
        }
        println!("written: {merged}");
    }

    if !errors.is_empty() {
        std::process::exit(1);
    }
}

/// Handles the result of a per-namespace pipeline step.
///
/// On failure, prints the error, records it when `keep_going` is true, and
/// returns `true` to signal that the caller should `continue` to the next
/// namespace.  When `keep_going` is false the process exits immediately.
/// Returns `false` on success.
fn handle_step_error(
    namespace: &str,
    result: Result<(), windows_rdl::Error>,
    keep_going: bool,
    errors: &mut Vec<(String, String)>,
) -> bool {
    if let Err(e) = result {
        let msg = format!("{e}");
        eprintln!("error [{namespace}]: {msg}");
        if keep_going {
            errors.push((namespace.to_string(), msg));
            true
        } else {
            std::process::exit(1);
        }
    } else {
        false
    }
}

fn run_clang(
    ns: &manifest::Namespace,
    clang_args: &[String],
    reference: Option<&str>,
    rdl_path: &str,
) -> Result<(), windows_rdl::Error> {
    let mut c = windows_rdl::clang();
    c.inputs(ns.headers.iter());
    c.namespace(&ns.name);
    c.library(&ns.library);
    c.filters(ns.filters.iter());
    c.args(clang_args.iter());
    c.output(rdl_path);

    if let Some(ref_path) = reference {
        c.input(ref_path);
    }

    c.write()
}

fn run_reader(
    rdl_path: &str,
    reference: Option<&str>,
    winmd_path: &str,
) -> Result<(), windows_rdl::Error> {
    let mut r = windows_rdl::reader();
    r.input(rdl_path);
    r.output(winmd_path);

    if let Some(ref_path) = reference {
        r.input(ref_path);
    }

    r.write()
}

// ---------------------------------------------------------------------------
// CLI parsing
// ---------------------------------------------------------------------------

struct Opts {
    /// Path to the manifest TOML file (required positional argument).
    manifest: String,
    /// Output directory for .rdl and .winmd files.
    output: String,
    /// Optional override for the reference .winmd path.
    reference: Option<String>,
    /// Include directories passed as `-I<dir>` to libclang.
    includes: Vec<String>,
    /// File name (not path) of the merged output, relative to `output`.
    merge_output: Option<String>,
    /// Continue processing namespaces after a failure.
    keep_going: bool,
}

fn parse_args(args: &[String]) -> Opts {
    let mut manifest: Option<String> = None;
    let mut output = String::from("target/win32");
    let mut reference: Option<String> = None;
    let mut includes: Vec<String> = Vec::new();
    let mut merge_output: Option<String> = None;
    let mut keep_going = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--output" | "-o" => {
                i += 1;
                output = require_arg_value(args, i, "--output");
            }
            "--reference" | "-r" => {
                i += 1;
                reference = Some(require_arg_value(args, i, "--reference"));
            }
            "--include" | "-I" => {
                i += 1;
                includes.push(require_arg_value(args, i, "--include"));
            }
            "--merge-output" => {
                i += 1;
                merge_output = Some(require_arg_value(args, i, "--merge-output"));
            }
            "--keep-going" | "-k" => {
                keep_going = true;
            }
            "--help" | "-h" => {
                print_help();
                std::process::exit(0);
            }
            flag if flag.starts_with("-I") => {
                // Support -I<path> without a space.
                includes.push(flag[2..].to_string());
            }
            other => {
                if manifest.is_some() {
                    eprintln!("error: unexpected argument `{other}`");
                    std::process::exit(1);
                }
                manifest = Some(other.to_string());
            }
        }
        i += 1;
    }

    let manifest = manifest.unwrap_or_else(|| {
        eprintln!("error: a manifest file is required");
        print_help();
        std::process::exit(1);
    });

    // Verify the manifest path exists and is a file so we get a clear error
    // message before attempting to parse it.
    if !Path::new(&manifest).is_file() {
        eprintln!("error: manifest `{manifest}` not found");
        std::process::exit(1);
    }

    Opts {
        manifest,
        output,
        reference,
        includes,
        merge_output,
        keep_going,
    }
}

fn require_arg_value(args: &[String], i: usize, flag: &str) -> String {
    if i >= args.len() {
        eprintln!("error: `{flag}` requires a value");
        std::process::exit(1);
    }
    args[i].clone()
}

fn print_help() {
    println!(
        r#"Usage: tool_win32 <manifest.toml> [OPTIONS]

Arguments:
  <manifest.toml>           Path to the win32 namespace manifest (required)

Options:
  -o, --output <dir>        Output directory for .rdl/.winmd files
                            [default: target/win32]
  -r, --reference <file>    Path to a reference .winmd for cross-namespace
                            type resolution (overrides manifest `reference`)
  -I, --include <dir>       SDK include directory; may be repeated
      --merge-output <name> File name of the merged output winmd
                            [default: Windows.Win32.winmd]
  -k, --keep-going          Continue after per-namespace errors
  -h, --help                Print this help message
"#
    );
}
