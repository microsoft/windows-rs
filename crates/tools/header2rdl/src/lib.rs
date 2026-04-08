//! Parses C/C++ header files and emits Windows RDL (`.rdl`) source.
//!
//! # Example
//!
//! ```no_run
//! tool_header2rdl::converter()
//!     .input("widget.h")
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
//! `LIBCLANG_PATH=/usr/lib/llvm-18/lib` (or the installed LLVM version).

use clang::{diagnostic::Severity, Clang, Entity, EntityKind, Index, TypeKind};
use std::collections::BTreeSet;

// ---------------------------------------------------------------------------
// Public builder
// ---------------------------------------------------------------------------

/// Returns a new [`Converter`] builder.
pub fn converter() -> Converter {
    Converter::new()
}

/// Builder for converting C/C++ header files to Windows RDL source.
///
/// Call builder methods to configure the conversion, then call
/// [`convert`](Converter::convert) to obtain the RDL string or
/// [`write`](Converter::write) to write directly to a file.
#[derive(Default)]
pub struct Converter {
    inputs: Vec<String>,
    namespace: String,
    library: String,
    cpp: bool,
    includes: Vec<String>,
    defines: Vec<String>,
    arch: String,
    output: String,
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
    pub fn input(&mut self, input: &str) -> &mut Self {
        self.inputs.push(input.to_string());
        self
    }

    /// Add multiple input header files.
    pub fn inputs<I, S>(&mut self, inputs: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        for input in inputs {
            self.inputs.push(input.as_ref().to_string());
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
    pub fn include(&mut self, include: &str) -> &mut Self {
        self.includes.push(include.to_string());
        self
    }

    /// Add one preprocessor definition passed to clang as `-D<def>`.
    /// The value may include `=value` (e.g. `"MY_DEFINE=1"`).
    pub fn define(&mut self, define: &str) -> &mut Self {
        self.defines.push(define.to_string());
        self
    }

    /// Set the target architecture: `"x86"`, `"x64"` (default), or `"arm64"`.
    pub fn arch(&mut self, arch: &str) -> &mut Self {
        self.arch = arch.to_string();
        self
    }

    /// Set the output file path (used by [`write`](Converter::write)).
    /// When [`split`](Converter::split) is `true` this is treated as a directory.
    pub fn output(&mut self, output: &str) -> &mut Self {
        self.output = output.to_string();
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
        if self.inputs.is_empty() {
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
                .map_err(|e| format!("failed to create directory `{dir}`: {e}"))?;
            let filename = format!("{}.rdl", self.namespace);
            let path = std::path::Path::new(dir).join(&filename);
            std::fs::write(&path, &rdl)
                .map_err(|e| format!("failed to write `{}`: {e}", path.display()))?;
        } else {
            if let Some(parent) = std::path::Path::new(&self.output).parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("failed to create parent directory: {e}"))?;
                }
            }
            std::fs::write(&self.output, &rdl)
                .map_err(|e| format!("failed to write `{}`: {e}", self.output))?;
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
        clang_args.push(format!("-I{inc}"));
    }
    for def in &c.defines {
        clang_args.push(format!("-D{def}"));
    }

    if c.cpp {
        clang_args.extend(["-x".into(), "c++".into(), "-std=c++17".into()]);
    } else {
        clang_args.extend(["-x".into(), "c".into()]);
    }

    let clang_arg_refs: Vec<&str> = clang_args.iter().map(String::as_str).collect();
    let mut collector = Collector::new(c.cpp);

    for input in &c.inputs {
        let tu = index
            .parser(input.as_str())
            .arguments(&clang_arg_refs)
            .parse()
            .map_err(|_| format!("failed to parse `{input}`"))?;

        let errors: Vec<String> = tu
            .get_diagnostics()
            .into_iter()
            .filter(|d| matches!(d.get_severity(), Severity::Error | Severity::Fatal))
            .map(|d| d.get_text())
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

fn non_underscore_name(entity: &Entity) -> Option<String> {
    entity
        .get_name()
        .filter(|n| !n.is_empty() && !n.starts_with('_'))
}

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
                if let Some(structs) = collect_struct(&child, name.clone(), is_union) {
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

    collector.seen.insert(name.clone());

    if let Some(underlying) = entity.get_typedef_underlying_type() {
        let value = map_type(&underlying);
        collector.typedefs.push(RdlTypedef { name, value });
    }
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
