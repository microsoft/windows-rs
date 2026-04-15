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

// clang-sys exports C-style names (e.g. CXType_Void, CXCursor_StructDecl) that
// Rust's nonstandard_style lint flags as non-upper-case when used in patterns.
#![allow(non_upper_case_globals)]

use clang_sys::*;
use std::collections::BTreeSet;
use std::ffi::{CStr, CString};
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
    clang_sys::load().is_ok()
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
// RAII wrappers for raw libclang handles
// ---------------------------------------------------------------------------

struct IndexGuard(CXIndex);

impl Drop for IndexGuard {
    fn drop(&mut self) {
        unsafe { clang_disposeIndex(self.0) };
    }
}

struct TuGuard(CXTranslationUnit);

impl Drop for TuGuard {
    fn drop(&mut self) {
        unsafe { clang_disposeTranslationUnit(self.0) };
    }
}

// ---------------------------------------------------------------------------
// libclang helper functions
// ---------------------------------------------------------------------------

/// Convert a `CXString` to a Rust `String`, disposing the CXString afterwards.
fn cx_string(s: CXString) -> String {
    unsafe {
        let ptr = clang_getCString(s);
        let result = if ptr.is_null() {
            String::new()
        } else {
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        };
        clang_disposeString(s);
        result
    }
}

/// Like `cx_string` but returns `None` for empty strings.
fn cx_string_option(s: CXString) -> Option<String> {
    let result = cx_string(s);
    if result.is_empty() { None } else { Some(result) }
}

/// Collect the direct children of `parent` into a `Vec`.
fn get_children(parent: CXCursor) -> Vec<CXCursor> {
    extern "C" fn visitor(
        cursor: CXCursor,
        _parent: CXCursor,
        data: CXClientData,
    ) -> CXChildVisitResult {
        let children = unsafe { &mut *(data as *mut Vec<CXCursor>) };
        children.push(cursor);
        CXChildVisit_Continue
    }
    let mut children: Vec<CXCursor> = Vec::new();
    unsafe {
        clang_visitChildren(
            parent,
            visitor,
            &mut children as *mut Vec<CXCursor> as CXClientData,
        );
    }
    children
}

fn cursor_name(cursor: CXCursor) -> Option<String> {
    cx_string_option(unsafe { clang_getCursorSpelling(cursor) })
}

fn cursor_kind(cursor: CXCursor) -> CXCursorKind {
    unsafe { clang_getCursorKind(cursor) }
}

fn is_definition(cursor: CXCursor) -> bool {
    unsafe { clang_isCursorDefinition(cursor) != 0 }
}

fn is_in_system_header(cursor: CXCursor) -> bool {
    let loc = unsafe { clang_getCursorLocation(cursor) };
    unsafe { clang_Location_isInSystemHeader(loc) != 0 }
}

fn is_pure_virtual(cursor: CXCursor) -> bool {
    unsafe { clang_CXXMethod_isPureVirtual(cursor) != 0 }
}

fn cursor_type(cursor: CXCursor) -> Option<CXType> {
    let ty = unsafe { clang_getCursorType(cursor) };
    if ty.kind == CXType_Invalid { None } else { Some(ty) }
}

fn cursor_definition(cursor: CXCursor) -> Option<CXCursor> {
    let def = unsafe { clang_getCursorDefinition(cursor) };
    if unsafe { clang_Cursor_isNull(def) } != 0 { None } else { Some(def) }
}

fn typedef_underlying_type(cursor: CXCursor) -> Option<CXType> {
    let ty = unsafe { clang_getTypedefDeclUnderlyingType(cursor) };
    if ty.kind == CXType_Invalid { None } else { Some(ty) }
}

fn enum_integer_type(cursor: CXCursor) -> Option<CXType> {
    let ty = unsafe { clang_getEnumDeclIntegerType(cursor) };
    if ty.kind == CXType_Invalid { None } else { Some(ty) }
}

fn type_spelling(ty: CXType) -> String {
    cx_string(unsafe { clang_getTypeSpelling(ty) })
}

fn canonical_type(ty: CXType) -> CXType {
    unsafe { clang_getCanonicalType(ty) }
}

fn type_declaration(ty: CXType) -> Option<CXCursor> {
    let cursor = unsafe { clang_getTypeDeclaration(ty) };
    if unsafe { clang_Cursor_isNull(cursor) } != 0
        || cursor_kind(cursor) == CXCursor_NoDeclFound
    {
        None
    } else {
        Some(cursor)
    }
}

fn pointee_type(ty: CXType) -> Option<CXType> {
    let inner = unsafe { clang_getPointeeType(ty) };
    if inner.kind == CXType_Invalid { None } else { Some(inner) }
}

fn element_type(ty: CXType) -> Option<CXType> {
    let elem = unsafe { clang_getArrayElementType(ty) };
    if elem.kind == CXType_Invalid { None } else { Some(elem) }
}

fn array_size(ty: CXType) -> usize {
    let s = unsafe { clang_getArraySize(ty) };
    if s < 0 { 0 } else { s as usize }
}

fn result_type(ty: CXType) -> Option<CXType> {
    let ret = unsafe { clang_getResultType(ty) };
    if ret.kind == CXType_Invalid { None } else { Some(ret) }
}

fn is_const_qualified(ty: CXType) -> bool {
    unsafe { clang_isConstQualifiedType(ty) != 0 }
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
    // Load libclang at runtime; the returned handle keeps the library alive.
    // It must be declared first so it is dropped last (after all libclang
    // objects derived from the index are already disposed).
    let _lib = clang_sys::load()
        .map_err(|e| format!("failed to initialize libclang: {e}"))?;

    let raw_index = unsafe { clang_createIndex(0, 0) };
    if raw_index.is_null() {
        return Err("failed to create clang index".into());
    }
    let _index = IndexGuard(raw_index);

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

    // Build the CString argument list once; keep it alive for the whole loop.
    let args_cstrings: Vec<CString> = clang_args
        .iter()
        .map(|s| CString::new(s.as_str()).expect("clang arg contains null byte"))
        .collect();
    let args_ptrs: Vec<*const std::os::raw::c_char> =
        args_cstrings.iter().map(|s| s.as_ptr()).collect();

    let mut collector = Collector::new(c.cpp);

    for input in &c.files {
        let input_str = input.to_string_lossy();
        let input_cstr = CString::new(input_str.as_bytes())
            .map_err(|_| format!("invalid filename: {}", input.display()))?;

        let mut tu_raw: CXTranslationUnit = std::ptr::null_mut();
        let err = unsafe {
            clang_parseTranslationUnit2(
                raw_index,
                input_cstr.as_ptr(),
                args_ptrs.as_ptr(),
                args_ptrs.len() as std::os::raw::c_int,
                std::ptr::null_mut(), // unsaved_files
                0,                    // num_unsaved_files
                CXTranslationUnit_None,
                &mut tu_raw,
            )
        };
        if err != CXError_Success || tu_raw.is_null() {
            return Err(format!("failed to parse `{}`", input.display()));
        }
        let tu = TuGuard(tu_raw);

        // Collect error/fatal diagnostics.
        let num_diags = unsafe { clang_getNumDiagnostics(tu.0) };
        let mut errors: Vec<String> = Vec::new();
        for i in 0..num_diags {
            let diag = unsafe { clang_getDiagnostic(tu.0, i) };
            let severity = unsafe { clang_getDiagnosticSeverity(diag) };
            if severity >= CXDiagnostic_Error {
                let text = cx_string(unsafe { clang_getDiagnosticSpelling(diag) });
                let loc = unsafe { clang_getDiagnosticLocation(diag) };
                let mut file: CXFile = std::ptr::null_mut();
                let mut line: std::os::raw::c_uint = 0;
                let mut col: std::os::raw::c_uint = 0;
                let mut offset: std::os::raw::c_uint = 0;
                unsafe {
                    clang_getFileLocation(loc, &mut file, &mut line, &mut col, &mut offset);
                }
                let file_str = if file.is_null() {
                    String::new()
                } else {
                    // Normalize the path: collect components to remove any redundant
                    // separators that libclang sometimes produces on Windows
                    // (e.g. `C:\Windows Kits\10\\include\...`), then convert to
                    // forward slashes so that VS Code terminal link detection works
                    // correctly even for paths containing spaces
                    // (e.g. `C:\Program Files (x86)\...`).
                    let name = cx_string(unsafe { clang_getFileName(file) });
                    let normalized: PathBuf = PathBuf::from(&name).components().collect();
                    normalized.to_string_lossy().replace('\\', "/")
                };
                errors.push(format!("error: {text}\n --> {file_str}:{line}:{col}"));
            }
            unsafe { clang_disposeDiagnostic(diag) };
        }
        if !errors.is_empty() {
            return Err(errors.join("\n"));
        }

        let root = unsafe { clang_getTranslationUnitCursor(tu.0) };
        collect(root, &mut collector);
        // `tu` is dropped here, which calls `clang_disposeTranslationUnit`.
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

fn collect(cursor: CXCursor, collector: &mut Collector) {
    for child in get_children(cursor) {
        if is_in_system_header(child) {
            continue;
        }

        match cursor_kind(child) {
            CXCursor_StructDecl | CXCursor_UnionDecl => {
                // Skip forward declarations: they must not claim the name in
                // `seen` before the real definition (possibly a StructDecl
                // produced by the MIDL_INTERFACE macro) is processed.
                if !is_definition(child) {
                    continue;
                }
                let is_union = cursor_kind(child) == CXCursor_UnionDecl;
                if collector.cpp && is_com_interface(child) {
                    if let Some(name) = non_underscore_name(child) {
                        if collector.seen.insert(name.clone()) {
                            if let Some(iface) = collect_interface(child, name) {
                                collector.interfaces.push(iface);
                            }
                        }
                    }
                } else if let Some(name) = non_underscore_name(child) {
                    if collector.seen.insert(name.clone()) {
                        if let Some(structs) = collect_struct(child, name, is_union) {
                            collector.structs.extend(structs);
                        }
                    }
                }
            }
            CXCursor_ClassDecl if collector.cpp => {
                // Skip forward declarations for the same reason as StructDecl above.
                if !is_definition(child) {
                    continue;
                }
                if let Some(name) = non_underscore_name(child) {
                    if collector.seen.insert(name.clone()) {
                        if is_com_interface(child) {
                            if let Some(iface) = collect_interface(child, name) {
                                collector.interfaces.push(iface);
                            }
                        } else if let Some(s) = collect_struct(child, name, false) {
                            collector.structs.extend(s);
                        }
                    }
                }
            }
            CXCursor_EnumDecl => {
                if let Some(name) = non_underscore_name(child) {
                    if collector.seen.insert(name.clone()) {
                        if let Some(e) = collect_enum(child, name) {
                            collector.enums.push(e);
                        }
                    }
                }
            }
            CXCursor_TypedefDecl => {
                collect_typedef(child, collector);
            }
            CXCursor_FunctionDecl => {
                // Skip inline functions (functions with bodies) — they cannot
                // be represented in RDL.  This is important because macros like
                // DEFINE_ENUM_FLAG_OPERATORS expand to inline operator
                // overloads that must not be collected.
                if is_definition(child) {
                    continue;
                }
                if let Some(name) = cursor_name(child) {
                    if collector.seen.insert(name.clone()) {
                        if let Some(f) = collect_function(child, name) {
                            collector.functions.push(f);
                        }
                    }
                }
            }
            // Recurse into `extern "C" { }` linkage-specification blocks so
            // that types defined inside them (as in the MIDL-generated style
            // used by WebView2.h) are collected.
            CXCursor_LinkageSpec => {
                collect(child, collector);
            }
            _ => {}
        }
    }
}

fn non_underscore_name(cursor: CXCursor) -> Option<String> {
    cursor_name(cursor).filter(|n| !n.is_empty() && !n.starts_with('_'))
}

fn is_com_interface(cursor: CXCursor) -> bool {
    // Follow forward declarations to the full definition.  When a COM interface
    // type is used as a parameter before its definition appears in the header,
    // `get_declaration()` may return the incomplete forward-declaration entity,
    // which has no children.  Resolving to the definition ensures we detect
    // pure-virtual methods regardless of declaration order or platform.
    let target = cursor_definition(cursor).unwrap_or(cursor);
    get_children(target)
        .iter()
        .any(|c| cursor_kind(*c) == CXCursor_CXXMethod && is_pure_virtual(*c))
}

fn collect_typedef(cursor: CXCursor, collector: &mut Collector) {
    let name = match cursor_name(cursor) {
        Some(n) if !n.is_empty() && !n.starts_with('_') => n,
        _ => return,
    };

    if collector.seen.contains(&name) {
        return;
    }

    for child in get_children(cursor) {
        match cursor_kind(child) {
            CXCursor_StructDecl | CXCursor_UnionDecl => {
                let is_union = cursor_kind(child) == CXCursor_UnionDecl;
                if let Some(structs) = collect_struct(child, name.clone(), is_union) {
                    if let Some(inner) = cursor_name(child) {
                        collector.seen.insert(inner);
                    }
                    collector.seen.insert(name);
                    collector.structs.extend(structs);
                }
                return;
            }
            CXCursor_ClassDecl => {
                // In C++ mode, `typedef interface X X;` creates a ClassDecl
                // child.  If it is a forward declaration (not yet defined),
                // return without inserting into `seen` so the actual definition
                // (which may arrive as a StructDecl from a MIDL_INTERFACE macro
                // expansion) can be collected later.
                if !is_definition(child) {
                    return;
                }
                // Inline class definition (`typedef class { … } X;`).
                if let Some(structs) = collect_struct(child, name.clone(), false) {
                    if let Some(inner) = cursor_name(child) {
                        collector.seen.insert(inner);
                    }
                    collector.seen.insert(name);
                    collector.structs.extend(structs);
                }
                return;
            }
            CXCursor_EnumDecl => {
                if let Some(e) = collect_enum(child, name.clone()) {
                    if let Some(inner) = cursor_name(child) {
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
    if let Some(underlying) = typedef_underlying_type(cursor) {
        let value = map_type(underlying);
        if value == name {
            // Self-referential forward-declaration alias: skip it.
            return;
        }

        // Also guard against the case where the underlying canonical type is an
        // *incomplete* record — this covers typedef aliases with a different name
        // for a not-yet-defined struct/class.
        let canonical = canonical_type(underlying);
        if canonical.kind == CXType_Record {
            if let Some(decl) = type_declaration(canonical) {
                if !is_definition(decl) {
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

fn collect_struct(cursor: CXCursor, name: String, is_union: bool) -> Option<Vec<RdlStruct>> {
    if !is_definition(cursor) {
        return None;
    }

    let mut fields = vec![];
    let mut nested: Vec<RdlStruct> = vec![];
    // Track the 0-based index of anonymous nested types to match the numeric-
    // suffix naming scheme used by windows-bindgen and the windows-rdl writer:
    // OUTER_0, OUTER_1, OUTER_0_0, etc.
    let mut nested_index: usize = 0;

    for (i, child) in get_children(cursor).iter().enumerate() {
        if cursor_kind(*child) == CXCursor_FieldDecl {
            let field_name = cursor_name(*child).unwrap_or_else(|| format!("field_{}", i + 1));
            if let Some(ty) = cursor_type(*child) {
                // Detect anonymous struct/union fields by checking whether the
                // (stripped) display name of the *original* type starts with '('.
                // e.g. Elaborated "struct (unnamed struct at ...)" → "(unnamed struct at ...)"
                let display = strip_elaboration(&type_spelling(ty));
                let canonical = canonical_type(ty);
                if display.starts_with('(') && canonical.kind == CXType_Record {
                    if let Some(decl) = type_declaration(canonical) {
                        // Use numeric suffix matching windows-bindgen/windows-rdl convention:
                        // OUTER_0, OUTER_1, OUTER_0_0, ...
                        let nested_name = format!("{}_{}", name, nested_index);
                        let is_nested_union = cursor_kind(decl) == CXCursor_UnionDecl;
                        if let Some(nested_structs) =
                            collect_struct(decl, nested_name.clone(), is_nested_union)
                        {
                            fields.push((field_name, nested_name));
                            nested.extend(nested_structs);
                            nested_index += 1;
                            continue;
                        }
                    }
                }
                fields.push((field_name, map_type(ty)));
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

fn collect_enum(cursor: CXCursor, name: String) -> Option<RdlEnum> {
    if !is_definition(cursor) {
        return None;
    }

    let repr = match enum_integer_type(cursor) {
        Some(ty) => match ty.kind {
            CXType_Int | CXType_Long => "i32",
            CXType_UInt | CXType_ULong => "u32",
            CXType_Short => "i16",
            CXType_UShort => "u16",
            CXType_Char_S | CXType_SChar => "i8",
            CXType_Char_U | CXType_UChar => "u8",
            CXType_LongLong => "i64",
            CXType_ULongLong => "u64",
            _ => "i32",
        },
        None => "i32",
    };

    let mut variants = vec![];
    for child in get_children(cursor) {
        if cursor_kind(child) == CXCursor_EnumConstantDecl {
            if let Some(vname) = cursor_name(child) {
                let signed = unsafe { clang_getEnumConstantDeclValue(child) };
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

fn collect_function(cursor: CXCursor, name: String) -> Option<RdlFn> {
    let ty = cursor_type(cursor)?;
    let ret_ty = result_type(ty)?;
    let ret = map_type(ret_ty);

    let mut params = vec![];
    for (i, child) in get_children(cursor).iter().enumerate() {
        if cursor_kind(*child) == CXCursor_ParmDecl {
            let pname = cursor_name(*child).unwrap_or_else(|| format!("param_{}", i + 1));
            if let Some(pty) = cursor_type(*child) {
                params.push((pname, map_type(pty)));
            }
        }
    }

    Some(RdlFn { name, params, ret })
}

fn collect_interface(cursor: CXCursor, name: String) -> Option<RdlInterface> {
    if !is_definition(cursor) {
        return None;
    }

    let children = get_children(cursor);

    // Extract the GUID from an `UnexposedAttr` child whose source text contains
    // a `__declspec(uuid("..."))` or `MIDL_INTERFACE("...")` macro expansion.
    let guid = children
        .iter()
        .find(|c| cursor_kind(**c) == CXCursor_UnexposedAttr)
        .and_then(|attr| {
            let range = unsafe { clang_getCursorExtent(*attr) };
            let start_loc = unsafe { clang_getRangeStart(range) };
            let end_loc = unsafe { clang_getRangeEnd(range) };

            let mut start_file: CXFile = std::ptr::null_mut();
            let mut start_line: std::os::raw::c_uint = 0;
            let mut start_col: std::os::raw::c_uint = 0;
            let mut start_offset: std::os::raw::c_uint = 0;
            unsafe {
                clang_getFileLocation(
                    start_loc,
                    &mut start_file,
                    &mut start_line,
                    &mut start_col,
                    &mut start_offset,
                );
            }
            if start_file.is_null() {
                return None;
            }

            let mut end_file: CXFile = std::ptr::null_mut();
            let mut end_line: std::os::raw::c_uint = 0;
            let mut end_col: std::os::raw::c_uint = 0;
            let mut end_offset: std::os::raw::c_uint = 0;
            unsafe {
                clang_getFileLocation(
                    end_loc,
                    &mut end_file,
                    &mut end_line,
                    &mut end_col,
                    &mut end_offset,
                );
            }

            let path = PathBuf::from(cx_string(unsafe { clang_getFileName(start_file) }));
            let source = std::fs::read_to_string(&path).ok()?;
            let text = source
                .get(start_offset as usize..end_offset as usize)
                .unwrap_or("");
            parse_guid_from_attr(text)
        });

    let base = children
        .iter()
        .find(|c| cursor_kind(**c) == CXCursor_CXXBaseSpecifier)
        .and_then(|c| cursor_name(*c));

    const IUNKNOWN: [&str; 3] = ["QueryInterface", "AddRef", "Release"];

    let mut methods = vec![];
    for child in &children {
        if cursor_kind(*child) != CXCursor_CXXMethod || !is_pure_virtual(*child) {
            continue;
        }
        let mname = match cursor_name(*child) {
            Some(n) if !IUNKNOWN.contains(&n.as_str()) => n,
            _ => continue,
        };
        let fn_type = cursor_type(*child)?;
        let ret_ty = result_type(fn_type)?;
        let ret = map_type(ret_ty);

        let mut params = vec![];
        for (i, p) in get_children(*child).iter().enumerate() {
            if cursor_kind(*p) == CXCursor_ParmDecl {
                let pname = cursor_name(*p).unwrap_or_else(|| format!("param_{}", i + 1));
                if let Some(pty) = cursor_type(*p) {
                    params.push((pname, map_type(pty)));
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

fn map_type(ty: CXType) -> String {
    match ty.kind {
        CXType_Void => "()".into(),
        CXType_Bool => "bool".into(),
        CXType_Char_S | CXType_SChar => "i8".into(),
        CXType_Char_U | CXType_UChar => "u8".into(),
        CXType_Short => "i16".into(),
        CXType_UShort => "u16".into(),
        CXType_Int => "i32".into(),
        CXType_UInt => "u32".into(),
        // Windows uses the LLP64 model: `long` is always 32-bit on all targets.
        CXType_Long => "i32".into(),
        CXType_ULong => "u32".into(),
        CXType_LongLong => "i64".into(),
        CXType_ULongLong => "u64".into(),
        CXType_Float => "f32".into(),
        CXType_Double => "f64".into(),
        CXType_WChar => "u16".into(),
        CXType_Pointer => map_pointer(ty),
        CXType_ConstantArray => {
            let size = array_size(ty);
            let elem = element_type(ty)
                .map(map_non_void)
                .unwrap_or_else(|| "u8".into());
            format!("[{elem}; {size}]")
        }
        // Preserve typedef / elaborated type names (e.g. DWORD, HANDLE, POINT)
        // so that cross-references resolve via windows-rdl's metadata search.
        CXType_Elaborated | CXType_Typedef => {
            let name = strip_elaboration(&type_spelling(ty));
            if name.starts_with('(') {
                "*mut u8".into()
            } else {
                name
            }
        }
        CXType_Record | CXType_Enum => {
            let name = strip_elaboration(&type_spelling(ty));
            // Anonymous types are replaced with an opaque byte pointer.
            if name.starts_with('(') {
                "*mut u8".into()
            } else {
                name
            }
        }
        _ => {
            let canon = canonical_type(ty);
            if canon.kind != ty.kind {
                map_type(canon)
            } else {
                "*mut u8".into()
            }
        }
    }
}

/// Returns `true` if `ty` is (or resolves to) a COM interface type —
/// i.e. a C++ class / struct that has at least one pure-virtual method.
fn is_com_interface_type(ty: CXType) -> bool {
    let canonical = canonical_type(ty);
    if canonical.kind != CXType_Record {
        return false;
    }
    if let Some(decl) = type_declaration(canonical) {
        is_com_interface(decl)
    } else {
        false
    }
}

fn map_pointer(ty: CXType) -> String {
    let inner = match pointee_type(ty) {
        Some(t) => t,
        None => return "*mut u8".into(),
    };
    let is_const = is_const_qualified(inner);

    // COM interfaces are reference types in RDL/metadata: one level of pointer
    // indirection is always implied.  Strip it here so that:
    //   IFoo*  → IFoo          (single pointer to interface → bare interface)
    //   IFoo** → *mut IFoo     (double pointer → one *mut remaining)
    if is_com_interface_type(inner) {
        return map_type(inner);
    }

    match inner.kind {
        CXType_Void => {
            if is_const {
                "*const u8".into()
            } else {
                "*mut u8".into()
            }
        }
        CXType_FunctionProto | CXType_FunctionNoProto => "*mut u8".into(),
        _ => {
            let inner_str = map_non_void(inner);
            if is_const {
                format!("*const {inner_str}")
            } else {
                format!("*mut {inner_str}")
            }
        }
    }
}

fn map_non_void(ty: CXType) -> String {
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
