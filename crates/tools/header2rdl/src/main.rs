//! Parses C/C++ header files and emits Windows RDL (`.rdl`) source.
//!
//! # Usage
//!
//! ```text
//! tool_header2rdl [OPTIONS] <input.h>...
//!
//! Options:
//!   --namespace <NS>   Target namespace, dot-separated (e.g. Contoso.Widgets)  [required]
//!   --library   <LIB>  Import-library name for generated functions (e.g. contoso.dll)
//!   --cpp              Enable C++ mode – detects COM interfaces
//!   --include   <DIR>  Extra include search path (repeatable)
//!   --define    <DEF>  Preprocessor define, may include value (e.g. MY_DEF=1) (repeatable)
//!   --arch      <A>    Target architecture: x86 | x64 | arm64  [default: x64]
//!   --output    <PATH> Output file (default: output.rdl) or directory when --split
//!   --split            Write one .rdl file per input into the --output directory
//! ```
//!
//! # Notes
//!
//! Requires `libclang` at build and run time.  On Ubuntu 22.04+ set
//! `LIBCLANG_PATH=/usr/lib/llvm-18/lib` (or the installed LLVM version).

use clang::{Clang, Entity, EntityKind, Index, TypeKind};
use std::collections::BTreeSet;

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = run(&argv) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

#[derive(Default)]
struct Config {
    inputs: Vec<String>,
    output: String,
    namespace: String,
    library: String,
    cpp: bool,
    includes: Vec<String>,
    defines: Vec<String>,
    arch: String,
    split: bool,
}

fn parse_args(args: &[String]) -> Result<Config, String> {
    let mut config = Config {
        arch: "x64".into(),
        output: "output.rdl".into(),
        ..Default::default()
    };

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--namespace" => {
                i += 1;
                config.namespace = args
                    .get(i)
                    .cloned()
                    .ok_or("expected value for --namespace")?;
            }
            "--library" => {
                i += 1;
                config.library = args.get(i).cloned().ok_or("expected value for --library")?;
            }
            "--cpp" => config.cpp = true,
            "--include" => {
                i += 1;
                config
                    .includes
                    .push(args.get(i).cloned().ok_or("expected value for --include")?);
            }
            "--define" => {
                i += 1;
                config
                    .defines
                    .push(args.get(i).cloned().ok_or("expected value for --define")?);
            }
            "--arch" => {
                i += 1;
                config.arch = args.get(i).cloned().ok_or("expected value for --arch")?;
            }
            "--output" => {
                i += 1;
                config.output = args.get(i).cloned().ok_or("expected value for --output")?;
            }
            "--split" => config.split = true,
            arg if !arg.starts_with('-') => config.inputs.push(arg.to_string()),
            arg => return Err(format!("unknown argument: {arg}")),
        }
        i += 1;
    }

    Ok(config)
}

fn arch_triple(arch: &str) -> &str {
    match arch {
        "x86" | "i686" => "i686-pc-windows-msvc",
        "arm64" | "aarch64" => "aarch64-pc-windows-msvc",
        _ => "x86_64-pc-windows-msvc",
    }
}

// ---------------------------------------------------------------------------
// RDL type representations
// ---------------------------------------------------------------------------

struct RdlEnum {
    name: String,
    repr: &'static str,
    variants: Vec<(String, i64)>,
}

struct RdlStruct {
    name: String,
    is_union: bool,
    fields: Vec<(String, String)>,
}

/// A free function or an interface method (both share the same shape).
struct RdlFn {
    name: String,
    params: Vec<(String, String)>,
    ret: String,
}

struct RdlInterface {
    name: String,
    base: Option<String>,
    methods: Vec<RdlFn>,
}

// ---------------------------------------------------------------------------
// Collector
// ---------------------------------------------------------------------------

#[derive(Default)]
struct Collector {
    enums: Vec<RdlEnum>,
    structs: Vec<RdlStruct>,
    functions: Vec<RdlFn>,
    interfaces: Vec<RdlInterface>,
    seen: BTreeSet<String>,
    cpp: bool,
}

impl Collector {
    fn new(cpp: bool) -> Self {
        Self {
            cpp,
            ..Default::default()
        }
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn run(args: &[String]) -> Result<(), String> {
    let config = parse_args(args)?;

    if config.inputs.is_empty() {
        return Err("no input files specified".into());
    }
    if config.namespace.is_empty() {
        return Err("--namespace is required".into());
    }

    let clang = Clang::new().map_err(|e| format!("failed to initialize libclang: {e}"))?;
    let index = Index::new(&clang, false, false);

    let mut clang_args: Vec<String> = vec![
        format!("--target={}", arch_triple(&config.arch)),
        "-fms-extensions".into(),
        "-fms-compatibility".into(),
        "-D_WIN32".into(),
        "-DWIN32".into(),
    ];

    for inc in &config.includes {
        clang_args.push(format!("-I{inc}"));
    }
    for def in &config.defines {
        clang_args.push(format!("-D{def}"));
    }

    if config.cpp {
        clang_args.extend(["-x".into(), "c++".into(), "-std=c++17".into()]);
    } else {
        clang_args.extend(["-x".into(), "c".into()]);
    }

    let clang_arg_refs: Vec<&str> = clang_args.iter().map(String::as_str).collect();
    let mut collector = Collector::new(config.cpp);

    for input in &config.inputs {
        let tu = index
            .parser(input.as_str())
            .arguments(&clang_arg_refs)
            .parse()
            .map_err(|_| format!("failed to parse `{input}`"))?;

        collect(tu.get_entity(), &mut collector);
    }

    let rdl = emit(&config, &collector);
    let formatted = windows_rdl::formatter::format(&rdl);

    if config.split {
        let dir = &config.output;
        std::fs::create_dir_all(dir)
            .map_err(|e| format!("failed to create directory `{dir}`: {e}"))?;
        let filename = format!("{}.rdl", config.namespace);
        let path = std::path::Path::new(dir).join(&filename);
        std::fs::write(&path, &formatted)
            .map_err(|e| format!("failed to write `{}`: {e}", path.display()))?;
    } else {
        if let Some(parent) = std::path::Path::new(&config.output).parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("failed to create parent directory: {e}"))?;
            }
        }
        std::fs::write(&config.output, &formatted)
            .map_err(|e| format!("failed to write `{}`: {e}", config.output))?;
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// AST collection
// ---------------------------------------------------------------------------

fn collect(entity: Entity, collector: &mut Collector) {
    for child in entity.get_children() {
        if child.is_in_system_header() {
            continue;
        }

        match child.get_kind() {
            // Plain struct / union – skip underscored names; they will be
            // handled when the matching TypedefDecl is visited.
            EntityKind::StructDecl | EntityKind::UnionDecl => {
                let is_union = child.get_kind() == EntityKind::UnionDecl;
                if collector.cpp && is_com_interface(&child) {
                    if let Some(name) = non_underscore_name(&child) {
                        if collector.seen.insert(name.clone()) {
                            if let Some(iface) = collect_interface(&child, name) {
                                collector.interfaces.push(iface);
                            }
                        }
                    }
                } else if let Some(name) = non_underscore_name(&child) {
                    if collector.seen.insert(name.clone()) {
                        if let Some(s) = collect_struct(&child, name, is_union) {
                            collector.structs.push(s);
                        }
                    }
                }
            }
            // C++ class – same logic as StructDecl in cpp mode.
            EntityKind::ClassDecl if collector.cpp => {
                if let Some(name) = non_underscore_name(&child) {
                    if collector.seen.insert(name.clone()) {
                        if is_com_interface(&child) {
                            if let Some(iface) = collect_interface(&child, name) {
                                collector.interfaces.push(iface);
                            }
                        } else if let Some(s) = collect_struct(&child, name, false) {
                            collector.structs.push(s);
                        }
                    }
                }
            }
            EntityKind::EnumDecl => {
                if let Some(name) = non_underscore_name(&child) {
                    if collector.seen.insert(name.clone()) {
                        if let Some(e) = collect_enum(&child, name) {
                            collector.enums.push(e);
                        }
                    }
                }
            }
            EntityKind::TypedefDecl => {
                collect_typedef(&child, collector);
            }
            EntityKind::FunctionDecl => {
                if let Some(name) = child.get_name() {
                    if collector.seen.insert(name.clone()) {
                        if let Some(f) = collect_function(&child, name) {
                            collector.functions.push(f);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// Returns the entity's name only when it doesn't start with `_`.
fn non_underscore_name(entity: &Entity) -> Option<String> {
    entity
        .get_name()
        .filter(|n| !n.is_empty() && !n.starts_with('_'))
}

/// Checks whether a C++ struct / class looks like a COM interface:
/// it must have at least one pure-virtual method (with or without a base class).
fn is_com_interface(entity: &Entity) -> bool {
    entity
        .get_children()
        .iter()
        .any(|c| c.get_kind() == EntityKind::Method && c.is_pure_virtual_method())
}

fn collect_typedef(entity: &Entity, collector: &mut Collector) {
    let name = match entity.get_name() {
        Some(n) if !n.is_empty() && !n.starts_with('_') => n,
        _ => return,
    };

    if collector.seen.contains(&name) {
        return;
    }

    for child in entity.get_children() {
        match child.get_kind() {
            EntityKind::StructDecl | EntityKind::UnionDecl => {
                let is_union = child.get_kind() == EntityKind::UnionDecl;
                if let Some(s) = collect_struct(&child, name.clone(), is_union) {
                    // Also mark the underscored inner name as seen.
                    if let Some(inner) = child.get_name() {
                        collector.seen.insert(inner);
                    }
                    collector.seen.insert(name);
                    collector.structs.push(s);
                }
                return;
            }
            EntityKind::EnumDecl => {
                if let Some(e) = collect_enum(&child, name.clone()) {
                    if let Some(inner) = child.get_name() {
                        collector.seen.insert(inner);
                    }
                    collector.seen.insert(name);
                    collector.enums.push(e);
                }
                return;
            }
            _ => {}
        }
    }

    // No struct/enum child – just a type alias; mark as seen.
    collector.seen.insert(name);
}

fn collect_struct(entity: &Entity, name: String, is_union: bool) -> Option<RdlStruct> {
    if !entity.is_definition() {
        return None;
    }

    let mut fields = vec![];
    for (i, child) in entity.get_children().iter().enumerate() {
        if child.get_kind() == EntityKind::FieldDecl {
            let field_name = child
                .get_name()
                .unwrap_or_else(|| format!("field_{}", i + 1));
            if let Some(ty) = child.get_type() {
                fields.push((field_name, map_type(&ty)));
            }
        }
    }

    Some(RdlStruct {
        name,
        is_union,
        fields,
    })
}

fn collect_enum(entity: &Entity, name: String) -> Option<RdlEnum> {
    if !entity.is_definition() {
        return None;
    }

    let repr = match entity.get_enum_underlying_type() {
        Some(ty) => match ty.get_kind() {
            TypeKind::Int | TypeKind::Long => "i32",
            TypeKind::UInt | TypeKind::ULong => "u32",
            TypeKind::Short => "i16",
            TypeKind::UShort => "u16",
            TypeKind::CharS | TypeKind::SChar => "i8",
            TypeKind::CharU | TypeKind::UChar => "u8",
            TypeKind::LongLong => "i64",
            TypeKind::ULongLong => "u64",
            _ => "i32",
        },
        None => "i32",
    };

    let mut variants = vec![];
    for child in entity.get_children() {
        if child.get_kind() == EntityKind::EnumConstantDecl {
            if let (Some(vname), Some((signed, _))) =
                (child.get_name(), child.get_enum_constant_value())
            {
                variants.push((vname, signed));
            }
        }
    }

    Some(RdlEnum {
        name,
        repr,
        variants,
    })
}

fn collect_function(entity: &Entity, name: String) -> Option<RdlFn> {
    let ty = entity.get_type()?;
    let ret_ty = ty.get_result_type()?;
    let ret = map_type(&ret_ty);

    let mut params = vec![];
    for (i, child) in entity.get_children().iter().enumerate() {
        if child.get_kind() == EntityKind::ParmDecl {
            let pname = child.get_name().unwrap_or_else(|| format!("p_{}", i + 1));
            if let Some(pty) = child.get_type() {
                params.push((pname, map_type(&pty)));
            }
        }
    }

    Some(RdlFn { name, params, ret })
}

fn collect_interface(entity: &Entity, name: String) -> Option<RdlInterface> {
    if !entity.is_definition() {
        return None;
    }

    let children = entity.get_children();
    let base = children
        .iter()
        .find(|c| c.get_kind() == EntityKind::BaseSpecifier)
        .and_then(|c| c.get_name());

    // Skip the standard IUnknown methods – every COM interface inherits them.
    const IUNKNOWN: [&str; 3] = ["QueryInterface", "AddRef", "Release"];

    let mut methods = vec![];
    for child in &children {
        if child.get_kind() != EntityKind::Method || !child.is_pure_virtual_method() {
            continue;
        }
        let mname = match child.get_name() {
            Some(n) if !IUNKNOWN.contains(&n.as_str()) => n,
            _ => continue,
        };
        let fn_type = child.get_type()?;
        let ret_ty = fn_type.get_result_type()?;
        let ret = map_type(&ret_ty);

        let mut params = vec![];
        for (i, p) in child.get_children().iter().enumerate() {
            if p.get_kind() == EntityKind::ParmDecl {
                let pname = p.get_name().unwrap_or_else(|| format!("p_{}", i + 1));
                if let Some(pty) = p.get_type() {
                    params.push((pname, map_type(&pty)));
                }
            }
        }

        methods.push(RdlFn {
            name: mname,
            params,
            ret,
        });
    }

    Some(RdlInterface {
        name,
        base,
        methods,
    })
}

// ---------------------------------------------------------------------------
// Type mapping  (clang type → RDL type string)
// ---------------------------------------------------------------------------

fn map_type(ty: &clang::Type) -> String {
    match ty.get_kind() {
        TypeKind::Void => "()".into(),
        TypeKind::Bool => "bool".into(),
        TypeKind::CharS | TypeKind::SChar => "i8".into(),
        TypeKind::CharU | TypeKind::UChar => "u8".into(),
        TypeKind::Short => "i16".into(),
        TypeKind::UShort => "u16".into(),
        TypeKind::Int => "i32".into(),
        TypeKind::UInt => "u32".into(),
        // Windows uses the LLP64 model: `long` is always 32-bit on all targets.
        TypeKind::Long => "i32".into(),
        TypeKind::ULong => "u32".into(),
        TypeKind::LongLong => "i64".into(),
        TypeKind::ULongLong => "u64".into(),
        TypeKind::Float => "f32".into(),
        TypeKind::Double => "f64".into(),
        TypeKind::Pointer => map_pointer(ty),
        TypeKind::ConstantArray => {
            let size = ty.get_size().unwrap_or(0);
            let elem = ty
                .get_element_type()
                .as_ref()
                .map(map_non_void)
                .unwrap_or_else(|| "u8".into());
            format!("[{elem}; {size}]")
        }
        // Preserve typedef / elaborated type names (e.g. DWORD, HANDLE, POINT)
        // so that cross-references resolve via windows-rdl's metadata search.
        TypeKind::Elaborated | TypeKind::Typedef => {
            let name = strip_elaboration(&ty.get_display_name());
            if name.starts_with('(') {
                "*mut u8".into()
            } else {
                name
            }
        }
        TypeKind::Record | TypeKind::Enum => {
            let name = strip_elaboration(&ty.get_display_name());
            // Anonymous types (clang uses "(unnamed ...)" or "(anonymous ...)") are
            // replaced with an opaque byte pointer since they cannot be named in RDL.
            if name.starts_with('(') {
                "*mut u8".into()
            } else {
                name
            }
        }
        _ => {
            // Fall back to the canonical type; if that still doesn't resolve,
            // emit an opaque pointer type.
            let canon = ty.get_canonical_type();
            if canon.get_kind() != ty.get_kind() {
                map_type(&canon)
            } else {
                "*mut u8".into()
            }
        }
    }
}

fn map_pointer(ty: &clang::Type) -> String {
    let inner = match ty.get_pointee_type() {
        Some(t) => t,
        None => return "*mut u8".into(),
    };
    let is_const = inner.is_const_qualified();

    match inner.get_kind() {
        TypeKind::Void => {
            if is_const {
                "*const u8".into()
            } else {
                "*mut u8".into()
            }
        }
        TypeKind::FunctionPrototype | TypeKind::FunctionNoPrototype => "*mut u8".into(),
        _ => {
            let inner_str = map_non_void(&inner);
            if is_const {
                format!("*const {inner_str}")
            } else {
                format!("*mut {inner_str}")
            }
        }
    }
}

/// Like `map_type` but converts the unit type `"()"` to `"u8"` for use in
/// pointer / array element contexts where a concrete type is required.
fn map_non_void(ty: &clang::Type) -> String {
    let s = map_type(ty);
    if s == "()" {
        "u8".into()
    } else {
        s
    }
}

/// Strip `struct`/`enum`/`union`/`const` elaboration prefixes from a clang
/// display name so only the bare type identifier remains.
fn strip_elaboration(name: &str) -> String {
    let name = name.trim();
    let name = name.strip_prefix("struct ").unwrap_or(name);
    let name = name.strip_prefix("enum ").unwrap_or(name);
    let name = name.strip_prefix("union ").unwrap_or(name);
    let name = name.strip_prefix("const ").unwrap_or(name);
    name.to_string()
}

// ---------------------------------------------------------------------------
// RDL emission
// ---------------------------------------------------------------------------

fn emit(config: &Config, collector: &Collector) -> String {
    let mut body = String::new();

    for e in &collector.enums {
        body.push_str(&emit_enum(e));
    }
    for s in &collector.structs {
        body.push_str(&emit_struct(s));
    }
    for f in &collector.functions {
        body.push_str(&emit_fn(f, &config.library));
    }
    for iface in &collector.interfaces {
        body.push_str(&emit_interface(iface));
    }

    wrap_namespace(&config.namespace, &body)
}

fn emit_enum(e: &RdlEnum) -> String {
    let mut out = format!("#[repr({})] enum {} {{", e.repr, e.name);
    for (name, value) in &e.variants {
        out.push_str(&format!("{name} = {value},"));
    }
    out.push('}');
    out
}

fn emit_struct(s: &RdlStruct) -> String {
    let kw = if s.is_union { "union" } else { "struct" };
    let mut out = format!("{kw} {} {{", s.name);
    for (name, ty) in &s.fields {
        out.push_str(&format!("{name}: {ty},"));
    }
    out.push('}');
    out
}

fn emit_fn(f: &RdlFn, library: &str) -> String {
    let params = f
        .params
        .iter()
        .map(|(n, t)| format!("{n}: {t}"))
        .collect::<Vec<_>>()
        .join(", ");

    let ret = if f.ret == "()" {
        String::new()
    } else {
        format!(" -> {}", f.ret)
    };

    let link = if library.is_empty() {
        String::new()
    } else {
        format!("#[link(name = \"{library}\", abi = \"system\")] ")
    };

    format!("{link}fn {}({params}){ret};", f.name)
}

fn emit_interface(iface: &RdlInterface) -> String {
    let base_part = iface
        .base
        .as_deref()
        .map(|b| format!(" : {b}"))
        .unwrap_or_default();

    let mut out = format!("#[no_guid] interface {}{} {{", iface.name, base_part);

    for m in &iface.methods {
        let mut param_str = "&self".to_string();
        for (n, t) in &m.params {
            param_str.push_str(&format!(", {n}: {t}"));
        }
        let ret = if m.ret == "()" {
            String::new()
        } else {
            format!(" -> {}", m.ret)
        };
        out.push_str(&format!("fn {}({param_str}){ret};", m.name));
    }

    out.push('}');
    out
}

/// Wrap `body` in nested `mod` blocks according to the dot-separated
/// `namespace`, annotating the outermost module with `#[win32]`.
fn wrap_namespace(namespace: &str, body: &str) -> String {
    if body.is_empty() {
        return String::new();
    }

    let parts: Vec<&str> = namespace.split('.').filter(|s| !s.is_empty()).collect();

    if parts.is_empty() {
        return body.to_string();
    }

    let mut out = format!("#[win32] mod {} {{", parts[0]);
    for part in &parts[1..] {
        out.push_str(&format!("mod {} {{", part));
    }
    out.push_str(body);
    for _ in 0..parts.len() {
        out.push('}');
    }
    out
}
