//! Parses C/C++ header files and emits Windows RDL (`.rdl`) source.
//!
//! # Example
//!
//! ```no_run
//! tool_header2rdl::converter()
//!     .file("widget.h")
//!     .namespace("Contoso.Widgets")
//!     .library("contoso.dll")
//!     .output("contoso.rdl")
//!     .write()
//!     .unwrap();
//! ```
//!
//! # Notes
//!
//! Requires `libclang` at build and run time.  On Ubuntu 22.04+ set
//! `LIBCLANG_PATH=/usr/lib/llvm-20/lib` (or the installed LLVM version).

use clang::{diagnostic::Severity, Clang, Entity, EntityKind, Index, TypeKind};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Public builder
// ---------------------------------------------------------------------------

/// Returns a new [`Converter`] builder.
pub fn converter() -> Converter {
    Converter::new()
}

/// Returns `true` if `libclang` can be successfully loaded.
///
/// Use this to gracefully skip tests on platforms where `libclang` is not
/// available (e.g. 32-bit runners where only a 64-bit DLL is present, or
/// ARM64 runners where no supported `libclang` build is available).
///
/// This function only attempts to load libclang when `LIBCLANG_PATH` is
/// explicitly set in the environment.  On platforms where libclang is not
/// installed (and `LIBCLANG_PATH` is therefore unset) this avoids a potential
/// crash that can occur when the dynamic loader encounters an incompatible
/// system libclang binary.
pub fn is_available() -> bool {
    if std::env::var("LIBCLANG_PATH").is_err() {
        return false;
    }
    Clang::new().is_ok()
}

/// Builder for converting C/C++ header files to Windows RDL source.
///
/// Call builder methods to configure the conversion, then call
/// [`convert`](Converter::convert) to obtain the RDL string or
/// [`write`](Converter::write) to write directly to a file.
#[derive(Default)]
pub struct Converter {
    files: Vec<PathBuf>,
    namespace: String,
    library: String,
    cpp: bool,
    includes: Vec<PathBuf>,
    system_includes: Vec<PathBuf>,
    defines: Vec<(String, Option<String>)>,
    arch: String,
    output: PathBuf,
    split: bool,
}

impl Converter {
    /// Creates a new [`Converter`] with default settings (`arch = "x64"`).
    pub fn new() -> Self {
        Self {
            arch: "x64".into(),
            ..Default::default()
        }
    }

    /// Add one input header file.
    pub fn file<P: AsRef<Path>>(&mut self, file: P) -> &mut Self {
        self.files.push(file.as_ref().into());
        self
    }

    /// Add multiple input header files.
    pub fn files<I>(&mut self, files: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: AsRef<Path>,
    {
        for f in files {
            self.files.push(f.as_ref().into());
        }
        self
    }

    /// Set the target namespace (dot-separated, e.g. `"Contoso.Widgets"`).
    /// This is required; [`write`](Converter::write) and
    /// [`convert`](Converter::convert) return an error if it is empty.
    pub fn namespace(&mut self, namespace: &str) -> &mut Self {
        self.namespace = namespace.to_string();
        self
    }

    /// Set the import-library name used in `#[link(name = "...", abi = "system")]`
    /// annotations on generated functions.  If empty, no `#[link]` attribute is emitted.
    pub fn library(&mut self, library: &str) -> &mut Self {
        self.library = library.to_string();
        self
    }

    /// Enable C++ mode.  When `true`, the input is parsed as C++17 and structs
    /// with pure-virtual methods are emitted as COM `interface` declarations.
    pub fn cpp(&mut self, cpp: bool) -> &mut Self {
        self.cpp = cpp;
        self
    }

    /// Add one extra include directory passed to clang as `-I<dir>`.
    pub fn include<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.includes.push(dir.as_ref().into());
        self
    }

    /// Add an include directory passed to clang as `-isystem<dir>`.
    ///
    /// Entities defined in headers under this directory are treated as
    /// system-header definitions and are **not** emitted in the RDL output.
    /// Use this for shim/compatibility headers (e.g. a minimal `windows.h`)
    /// that need to be visible to the parsed header but should not generate
    /// new type declarations.
    pub fn system_include<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.system_includes.push(dir.as_ref().into());
        self
    }

    /// Add a preprocessor definition passed to clang as `-D<var>` or `-D<var>=<val>`.
    ///
    /// ```no_run
    /// tool_header2rdl::converter()
    ///     .file("my.h")
    ///     .namespace("My")
    ///     .define("MY_FLAG", None)
    ///     .define("MY_VALUE", "42")
    ///     .write()
    ///     .unwrap();
    /// ```
    pub fn define<'a, V: Into<Option<&'a str>>>(&mut self, var: &str, val: V) -> &mut Self {
        self.defines
            .push((var.to_string(), val.into().map(|v| v.to_string())));
        self
    }

    /// Set the target architecture: `"x86"`, `"x64"` (default), or `"arm64"`.
    pub fn arch(&mut self, arch: &str) -> &mut Self {
        self.arch = arch.to_string();
        self
    }

    /// Set the output file path (used by [`write`](Converter::write)).
    /// When [`split`](Converter::split) is `true` this is treated as a directory.
    pub fn output<P: AsRef<Path>>(&mut self, output: P) -> &mut Self {
        self.output = output.as_ref().into();
        self
    }

    /// When `true`, [`write`](Converter::write) writes one `<namespace>.rdl`
    /// file into the [`output`](Converter::output) directory instead of a
    /// single file.
    pub fn split(&mut self, split: bool) -> &mut Self {
        self.split = split;
        self
    }

    /// Parse the input headers and return the formatted RDL source string.
    pub fn convert(&self) -> Result<String, String> {
        if self.files.is_empty() {
            return Err("no input files specified".into());
        }
        if self.namespace.is_empty() {
            return Err("namespace is required".into());
        }
        generate(self)
    }

    /// Parse the input headers and write the formatted RDL to
    /// [`output`](Converter::output).
    pub fn write(&self) -> Result<(), String> {
        let rdl = self.convert()?;

        if self.split {
            let dir = &self.output;
            std::fs::create_dir_all(dir)
                .map_err(|e| format!("failed to create directory `{}`: {e}", dir.display()))?;
            let filename = format!("{}.rdl", self.namespace);
            let path = dir.join(&filename);
            std::fs::write(&path, &rdl)
                .map_err(|e| format!("failed to write `{}`: {e}", path.display()))?;
        } else {
            if let Some(parent) = self.output.parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("failed to create parent directory: {e}"))?;
                }
            }
            std::fs::write(&self.output, &rdl)
                .map_err(|e| format!("failed to write `{}`: {e}", self.output.display()))?;
        }

        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Core generation
// ---------------------------------------------------------------------------

fn arch_triple(arch: &str) -> &str {
    match arch {
        "x86" | "i686" => "i686-pc-windows-msvc",
        "arm64" | "aarch64" => "aarch64-pc-windows-msvc",
        _ => "x86_64-pc-windows-msvc",
    }
}

fn generate(c: &Converter) -> Result<String, String> {
    let clang = Clang::new().map_err(|e| format!("failed to initialize libclang: {e}"))?;
    let index = Index::new(&clang, false, false);

    let mut clang_args: Vec<String> = vec![
        format!("--target={}", arch_triple(&c.arch)),
        "-fms-extensions".into(),
        "-fms-compatibility".into(),
        "-D_WIN32".into(),
        "-DWIN32".into(),
    ];

    for inc in &c.includes {
        clang_args.push(format!("-I{}", inc.to_string_lossy()));
    }

    // System-include directories (-isystem): headers found there are treated as
    // system headers and their definitions are suppressed in the RDL output.
    for inc in &c.system_includes {
        clang_args.push(format!("-isystem{}", inc.to_string_lossy()));
    }

    // On Windows the INCLUDE environment variable contains the semicolon-separated
    // list of SDK/VC++ header directories (set by vcvarsall.bat or the VS build
    // environment).  libclang does not consult INCLUDE automatically, so we add
    // each path explicitly so that `#include <windows.h>` and its transitive
    // includes resolve correctly.  Windows SDK paths are added as system includes
    // so that SDK types are not emitted in the RDL output.
    if let Ok(include_env) = std::env::var("INCLUDE") {
        for path in include_env.split(';') {
            let path = path.trim();
            if !path.is_empty() {
                clang_args.push(format!("-isystem{path}"));
            }
        }
    }
    for (name, val) in &c.defines {
        if let Some(v) = val {
            clang_args.push(format!("-D{name}={v}"));
        } else {
            clang_args.push(format!("-D{name}"));
        }
    }

    if c.cpp {
        clang_args.extend(["-x".into(), "c++".into(), "-std=c++17".into()]);
    } else {
        clang_args.extend(["-x".into(), "c".into()]);
    }

    let clang_arg_refs: Vec<&str> = clang_args.iter().map(String::as_str).collect();
    let mut collector = Collector::new(c.cpp);

    for input in &c.files {
        let input_str = input.to_string_lossy();
        let tu = index
            .parser(&*input_str)
            .arguments(&clang_arg_refs)
            .parse()
            .map_err(|_| format!("failed to parse `{}`", input.display()))?;

        let errors: Vec<String> = tu
            .get_diagnostics()
            .into_iter()
            .filter(|d| matches!(d.get_severity(), Severity::Error | Severity::Fatal))
            .map(|d| {
                let loc = d.get_location().get_file_location();
                let file = loc
                    .file
                    .map(|f| {
                        // Normalize the path: collect components to remove any redundant
                        // separators that libclang sometimes produces on Windows
                        // (e.g. `C:\Windows Kits\10\\include\...`), then convert to
                        // forward slashes so that VS Code terminal link detection works
                        // correctly even for paths containing spaces
                        // (e.g. `C:\Program Files (x86)\...`).
                        let normalized: PathBuf = f.get_path().components().collect();
                        normalized.to_string_lossy().replace('\\', "/")
                    })
                    .unwrap_or_default();
                let line = loc.line;
                let col = loc.column;
                format!("error: {}\n --> {file}:{line}:{col}", d.get_text())
            })
            .collect();
        if !errors.is_empty() {
            return Err(errors.join("\n"));
        }

        collect(tu.get_entity(), &mut collector);
    }

    let rdl = emit(c, &collector);
    Ok(windows_rdl::formatter::format(&rdl))
}

// ---------------------------------------------------------------------------
// RDL type representations
// ---------------------------------------------------------------------------

struct RdlEnum {
    name: String,
    repr: &'static str,
    variants: Vec<(String, i64)>,
}

struct RdlTypedef {
    name: String,
    value: String,
}

struct RdlStruct {
    name: String,
    is_union: bool,
    fields: Vec<(String, String)>,
}

struct RdlFn {
    name: String,
    params: Vec<(String, String)>,
    ret: String,
}

struct RdlInterface {
    name: String,
    guid: Option<u128>,
    base: Option<String>,
    methods: Vec<RdlFn>,
}

// ---------------------------------------------------------------------------
// Collector
// ---------------------------------------------------------------------------

#[derive(Default)]
struct Collector {
    enums: Vec<RdlEnum>,
    typedefs: Vec<RdlTypedef>,
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
// AST collection
// ---------------------------------------------------------------------------

fn collect(entity: Entity, collector: &mut Collector) {
    for child in entity.get_children() {
        if child.is_in_system_header() {
            continue;
        }

        match child.get_kind() {
            EntityKind::StructDecl | EntityKind::UnionDecl => {
                // Skip forward declarations: they must not claim the name in
                // `seen` before the real definition (possibly a StructDecl
                // produced by the MIDL_INTERFACE macro) is processed.
                if !child.is_definition() {
                    continue;
                }
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
                        if let Some(structs) = collect_struct(&child, name, is_union) {
                            collector.structs.extend(structs);
                        }
                    }
                }
            }
            EntityKind::ClassDecl if collector.cpp => {
                // Skip forward declarations for the same reason as StructDecl above.
                if !child.is_definition() {
                    continue;
                }
                if let Some(name) = non_underscore_name(&child) {
                    if collector.seen.insert(name.clone()) {
                        if is_com_interface(&child) {
                            if let Some(iface) = collect_interface(&child, name) {
                                collector.interfaces.push(iface);
                            }
                        } else if let Some(s) = collect_struct(&child, name, false) {
                            collector.structs.extend(s);
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
                // Skip inline functions (functions with bodies) — they cannot
                // be represented in RDL.  This is important because macros like
                // DEFINE_ENUM_FLAG_OPERATORS expand to inline operator
                // overloads that must not be collected.
                if child.is_definition() {
                    continue;
                }
                if let Some(name) = child.get_name() {
                    if collector.seen.insert(name.clone()) {
                        if let Some(f) = collect_function(&child, name) {
                            collector.functions.push(f);
                        }
                    }
                }
            }
            // Recurse into `extern "C" { }` linkage-specification blocks so
            // that types defined inside them (as in the MIDL-generated style
            // used by WebView2.h) are collected.
            EntityKind::LinkageSpec => {
                collect(child, collector);
            }
            _ => {}
        }
    }
}

fn non_underscore_name(entity: &Entity) -> Option<String> {
    entity
        .get_name()
        .filter(|n| !n.is_empty() && !n.starts_with('_'))
}

fn is_com_interface(entity: &Entity) -> bool {
    // Follow forward declarations to the full definition.  When a COM interface
    // type is used as a parameter before its definition appears in the header,
    // `get_declaration()` may return the incomplete forward-declaration entity,
    // which has no children.  Resolving to the definition ensures we detect
    // pure-virtual methods regardless of declaration order or platform.
    let def = entity.get_definition();
    let target = def.as_ref().unwrap_or(entity);
    target
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
                if let Some(structs) = collect_struct(&child, name.clone(), is_union) {
                    if let Some(inner) = child.get_name() {
                        collector.seen.insert(inner);
                    }
                    collector.seen.insert(name);
                    collector.structs.extend(structs);
                }
                return;
            }
            EntityKind::ClassDecl => {
                // In C++ mode, `typedef interface X X;` creates a ClassDecl
                // child.  If it is a forward declaration (not yet defined),
                // return without inserting into `seen` so the actual definition
                // (which may arrive as a StructDecl from a MIDL_INTERFACE macro
                // expansion) can be collected later.
                if !child.is_definition() {
                    return;
                }
                // Inline class definition (`typedef class { … } X;`).
                if let Some(structs) = collect_struct(&child, name.clone(), false) {
                    if let Some(inner) = child.get_name() {
                        collector.seen.insert(inner);
                    }
                    collector.seen.insert(name);
                    collector.structs.extend(structs);
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

    // Reached if no StructDecl/ClassDecl/EnumDecl child was found in the TypedefDecl
    // children.  In C++ mode a forward-declaration alias such as
    // `typedef struct X X;` arrives here with an underlying type whose display
    // name equals the alias name itself.  Don't add the name to `seen` or emit a
    // typedef in that case — the real definition will be collected later.
    if let Some(underlying) = entity.get_typedef_underlying_type() {
        let value = map_type(&underlying);
        if value == name {
            // Self-referential forward-declaration alias: skip it.
            return;
        }

        // Also guard against the case where the underlying canonical type is an
        // *incomplete* record — this covers typedef aliases with a different name
        // for a not-yet-defined struct/class.
        let canonical = underlying.get_canonical_type();
        if canonical.get_kind() == TypeKind::Record {
            if let Some(decl) = canonical.get_declaration() {
                if !decl.is_definition() {
                    return;
                }
            }
        }

        collector.typedefs.push(RdlTypedef {
            name: name.clone(),
            value,
        });
    }

    collector.seen.insert(name);
}

fn collect_struct(entity: &Entity, name: String, is_union: bool) -> Option<Vec<RdlStruct>> {
    if !entity.is_definition() {
        return None;
    }

    let mut fields = vec![];
    let mut nested: Vec<RdlStruct> = vec![];
    // Track the 0-based index of anonymous nested types to match the numeric-
    // suffix naming scheme used by windows-bindgen and the windows-rdl writer:
    // OUTER_0, OUTER_1, OUTER_0_0, etc.
    let mut nested_index: usize = 0;

    for (i, child) in entity.get_children().iter().enumerate() {
        if child.get_kind() == EntityKind::FieldDecl {
            let field_name = child
                .get_name()
                .unwrap_or_else(|| format!("field_{}", i + 1));
            if let Some(ty) = child.get_type() {
                // Detect anonymous struct/union fields by checking whether the
                // (stripped) display name of the *original* type starts with '('.
                // e.g. Elaborated "struct (unnamed struct at ...)" → "(unnamed struct at ...)"
                let display = strip_elaboration(&ty.get_display_name());
                let canonical = ty.get_canonical_type();
                if display.starts_with('(') && canonical.get_kind() == TypeKind::Record {
                    if let Some(decl) = canonical.get_declaration() {
                        // Use numeric suffix matching windows-bindgen/windows-rdl convention:
                        // OUTER_0, OUTER_1, OUTER_0_0, ...
                        let nested_name = format!("{}_{}", name, nested_index);
                        let is_nested_union = decl.get_kind() == EntityKind::UnionDecl;
                        if let Some(nested_structs) =
                            collect_struct(&decl, nested_name.clone(), is_nested_union)
                        {
                            fields.push((field_name, nested_name));
                            nested.extend(nested_structs);
                            nested_index += 1;
                            continue;
                        }
                    }
                }
                fields.push((field_name, map_type(&ty)));
            }
        }
    }

    // Nested types must be emitted before the type that references them.
    nested.push(RdlStruct {
        name,
        is_union,
        fields,
    });
    Some(nested)
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
            let pname = child
                .get_name()
                .unwrap_or_else(|| format!("param_{}", i + 1));
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

    // Extract the GUID from an `UnexposedAttr` child whose source text contains
    // a `__declspec(uuid("..."))` or `MIDL_INTERFACE("...")` macro expansion.
    let guid = children
        .iter()
        .find(|c| c.get_kind() == EntityKind::UnexposedAttr)
        .and_then(|attr| attr.get_range())
        .and_then(|r| {
            let start = r.get_start().get_file_location();
            let end = r.get_end().get_file_location();
            let file = start.file?;
            let path = file.get_path();
            let source = std::fs::read_to_string(&path).ok()?;
            let text = source
                .get(start.offset as usize..end.offset as usize)
                .unwrap_or("");
            parse_guid_from_attr(text)
        });

    let base = children
        .iter()
        .find(|c| c.get_kind() == EntityKind::BaseSpecifier)
        .and_then(|c| c.get_name());

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
                let pname = p.get_name().unwrap_or_else(|| format!("param_{}", i + 1));
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
        guid,
        base,
        methods,
    })
}

/// Parse a GUID string from an attribute's source text.
///
/// Handles several common forms:
/// - `MIDL_INTERFACE("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX")`
/// - `__declspec(uuid("XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"))`
/// - Any quoted 36-character UUID string like `"XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"`
fn parse_guid_from_attr(text: &str) -> Option<u128> {
    // Find the first quoted string that looks like a GUID (36 chars: 8-4-4-4-12)
    for (i, ch) in text.char_indices() {
        if ch != '"' {
            continue;
        }
        // Collect the contents of this quoted string
        let start = i + 1;
        let mut end = start;
        for (j, c) in text[start..].char_indices() {
            if c == '"' {
                end = start + j;
                break;
            }
        }
        let candidate = &text[start..end];
        if let Some(guid) = parse_guid_str(candidate) {
            return Some(guid);
        }
    }
    None
}

/// Convert a `XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX` UUID string to u128.
fn parse_guid_str(s: &str) -> Option<u128> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 5 {
        return None;
    }
    // Verify each part has exactly the right number of hex characters.
    let lens = [8usize, 4, 4, 4, 12];
    if parts.iter().zip(lens.iter()).any(|(p, &l)| p.len() != l) {
        return None;
    }
    // Rebuild as a 32-char hex string and parse.
    u128::from_str_radix(&parts.concat(), 16).ok()
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
        TypeKind::WChar => "u16".into(),
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
            // Anonymous types are replaced with an opaque byte pointer.
            if name.starts_with('(') {
                "*mut u8".into()
            } else {
                name
            }
        }
        _ => {
            let canon = ty.get_canonical_type();
            if canon.get_kind() != ty.get_kind() {
                map_type(&canon)
            } else {
                "*mut u8".into()
            }
        }
    }
}

/// Returns `true` if `ty` is (or resolves to) a COM interface type —
/// i.e. a C++ class / struct that has at least one pure-virtual method.
fn is_com_interface_type(ty: &clang::Type) -> bool {
    let canonical = ty.get_canonical_type();
    if canonical.get_kind() != TypeKind::Record {
        return false;
    }
    if let Some(decl) = canonical.get_declaration() {
        is_com_interface(&decl)
    } else {
        false
    }
}

fn map_pointer(ty: &clang::Type) -> String {
    let inner = match ty.get_pointee_type() {
        Some(t) => t,
        None => return "*mut u8".into(),
    };
    let is_const = inner.is_const_qualified();

    // COM interfaces are reference types in RDL/metadata: one level of pointer
    // indirection is always implied.  Strip it here so that:
    //   IFoo*  → IFoo          (single pointer to interface → bare interface)
    //   IFoo** → *mut IFoo     (double pointer → one *mut remaining)
    if is_com_interface_type(&inner) {
        return map_type(&inner);
    }

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

fn map_non_void(ty: &clang::Type) -> String {
    let s = map_type(ty);
    if s == "()" {
        "u8".into()
    } else {
        s
    }
}

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

/// Escape a name that is a Rust keyword by prepending `r#`.
///
/// C allows identifiers like `type` that are reserved in Rust.  When we emit
/// RDL (which is parsed as Rust source), we must use the raw-identifier form
/// `r#type` so that the RDL reader can parse the output without errors.
fn escape_ident(name: &str) -> String {
    // Strict and reserved Rust keywords (edition 2021).
    const KEYWORDS: &[&str] = &[
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true",
        "type", "union", "unsafe", "use", "where", "while",
    ];
    if KEYWORDS.contains(&name) {
        format!("r#{name}")
    } else {
        name.to_string()
    }
}

fn emit(c: &Converter, collector: &Collector) -> String {
    let mut body = String::new();

    for e in &collector.enums {
        body.push_str(&emit_enum(e));
    }
    for t in &collector.typedefs {
        body.push_str(&emit_typedef(t));
    }
    for s in &collector.structs {
        body.push_str(&emit_struct(s));
    }
    for f in &collector.functions {
        body.push_str(&emit_fn(f, &c.library));
    }
    for iface in &collector.interfaces {
        body.push_str(&emit_interface(iface));
    }

    wrap_namespace(&c.namespace, &body)
}

fn emit_enum(e: &RdlEnum) -> String {
    let mut out = format!("#[repr({})] enum {} {{", e.repr, e.name);
    for (name, value) in &e.variants {
        out.push_str(&format!("{name} = {value},"));
    }
    out.push('}');
    out
}

fn emit_typedef(t: &RdlTypedef) -> String {
    format!("#[typedef] struct {} {{ value: {}, }}", t.name, t.value)
}

fn emit_struct(s: &RdlStruct) -> String {
    let kw = if s.is_union { "union" } else { "struct" };
    let mut out = format!("{kw} {} {{", s.name);
    for (name, ty) in &s.fields {
        out.push_str(&format!("{}: {ty},", escape_ident(name)));
    }
    out.push('}');
    out
}

fn emit_fn(f: &RdlFn, library: &str) -> String {
    let params = f
        .params
        .iter()
        .map(|(n, t)| format!("{}: {t}", escape_ident(n)))
        .collect::<Vec<_>>()
        .join(", ");

    let ret = if f.ret == "()" {
        String::new()
    } else {
        format!(" -> {}", f.ret)
    };

    let lib_attr = if library.is_empty() {
        String::new()
    } else {
        format!("#[library(\"{library}\")] ")
    };

    format!("{lib_attr}extern fn {}({params}){ret};", f.name)
}

fn emit_interface(iface: &RdlInterface) -> String {
    let base_part = iface
        .base
        .as_deref()
        .map(|b| format!(" : {b}"))
        .unwrap_or_default();

    let guid_attr = match iface.guid {
        Some(g) => format!("#[guid({g:#034x})] "),
        None => "#[no_guid] ".to_string(),
    };

    let mut out = format!("{guid_attr}interface {}{base_part} {{", iface.name);

    for m in &iface.methods {
        let mut param_str = "&self".to_string();
        for (n, t) in &m.params {
            param_str.push_str(&format!(", {}: {t}", escape_ident(n)));
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
