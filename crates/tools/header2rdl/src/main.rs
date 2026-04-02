mod collect;
mod converter;
mod emit;
mod ir;
mod types;

use converter::Config;

fn print_usage() {
    eprintln!(
        r#"header2rdl — convert C/C++ header files to RDL

Usage:
  header2rdl [options] <header.h> [header2.h ...]

Options:
  --namespace <ns>        Dot-separated RDL namespace (e.g. Windows.Win32.System.Threading)
  --library <dll>         DLL name for extern fn declarations (e.g. kernel32.dll)
  --include <path>        Add an include directory (repeatable)
  --define <NAME[=val]>   Add a preprocessor define (repeatable)
  --output <dir>          Output directory for .rdl files (default: current directory)
  --split                 Emit one file per top-level namespace component
  --cpp                   Parse headers as C++ (enables class, virtual, __uuid, etc.)
  --arch <triple>         Target architecture triple (default: x86_64-pc-windows-msvc)
                          Recognised shortcuts: x86, x64, arm, arm64
  --help                  Print this help message
"#
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut config = Config::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            "--namespace" => {
                i += 1;
                config.namespace = next_arg(&args, i, "--namespace");
            }
            "--library" => {
                i += 1;
                config.library = next_arg(&args, i, "--library");
            }
            "--include" | "-I" => {
                i += 1;
                config.include_dirs.push(next_arg(&args, i, "--include"));
            }
            "--define" | "-D" => {
                i += 1;
                config.defines.push(next_arg(&args, i, "--define"));
            }
            "--output" | "-o" => {
                i += 1;
                config.output_dir = next_arg(&args, i, "--output");
            }
            "--split" => {
                config.split = true;
            }
            "--cpp" => {
                config.cpp = true;
            }
            "--arch" => {
                i += 1;
                let arch = next_arg(&args, i, "--arch");
                config.arch_triple = expand_arch_triple(&arch);
            }
            arg if arg.starts_with("--") => {
                eprintln!("error: unknown option `{arg}`");
                print_usage();
                std::process::exit(1);
            }
            header => {
                config.headers.push(header.to_string());
            }
        }
        i += 1;
    }

    if config.headers.is_empty() {
        eprintln!("error: no header files specified");
        print_usage();
        std::process::exit(1);
    }

    if let Err(e) = converter::convert(&config) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn next_arg(args: &[String], i: usize, flag: &str) -> String {
    if i >= args.len() {
        eprintln!("error: `{flag}` requires an argument");
        std::process::exit(1);
    }
    args[i].clone()
}

fn expand_arch_triple(arch: &str) -> String {
    match arch {
        "x86" => "i686-pc-windows-msvc".to_string(),
        "x64" => "x86_64-pc-windows-msvc".to_string(),
        "arm" => "thumbv7a-pc-windows-msvc".to_string(),
        "arm64" | "aarch64" => "aarch64-pc-windows-msvc".to_string(),
        other => other.to_string(),
    }
}
