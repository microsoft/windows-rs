/// Walks the Clang AST and collects declarations into `IrItem`s.
use std::collections::{HashMap, HashSet};

use clang::{Entity, EntityKind};

use crate::ir::*;
use crate::types::{map_calling_convention, map_type, map_well_known};

/// Walk the top-level entities of a translation unit and collect all
/// declarations that originate from the header files under `source_paths`.
///
/// `source_paths` must be the set of absolute, canonical paths for the header
/// files that were passed on the command line.  Any declaration whose file is
/// not in this set (e.g. a system include) is silently skipped.
pub fn collect(
    tu_entity: Entity<'_>,
    source_paths: &HashSet<String>,
    default_library: &str,
) -> Vec<IrItem> {
    let mut collector = Collector {
        source_paths,
        default_library,
        known_names: HashSet::new(),
        typedef_to_struct: HashMap::new(),
        tag_to_typedef: HashMap::new(),
        items: Vec::new(),
    };

    // First pass: register names so type references can be resolved.
    collector.register_names(tu_entity);
    // Second pass: collect items.
    collector.collect_items(tu_entity);
    collector.items
}

struct Collector<'a> {
    source_paths: &'a HashSet<String>,
    default_library: &'a str,
    /// All type names seen in the TU (for `IrType::Named` resolution).
    known_names: HashSet<String>,
    /// Maps anonymous struct/union display strings to the typedef name that aliases them.
    typedef_to_struct: HashMap<String, String>,
    /// Maps a C tagged-struct name (e.g. `_MY_POINT`) to its typedef alias (`MY_POINT`),
    /// so the tagged name is skipped in favour of the cleaner typedef name.
    tag_to_typedef: HashMap<String, String>,
    items: Vec<IrItem>,
}

impl<'a> Collector<'a> {
    fn in_source(&self, entity: &Entity<'_>) -> bool {
        let Some(loc) = entity.get_location() else {
            return false;
        };
        let file = loc.get_file_location();
        let Some(f) = file.file else {
            return false;
        };
        let path = f.get_path().to_string_lossy().replace('\\', "/");
        self.source_paths.iter().any(|sp| path == *sp)
    }

    fn register_names(&mut self, tu_entity: Entity<'_>) {
        for child in tu_entity.get_children() {
            if !self.in_source(&child) {
                continue;
            }
            match child.get_kind() {
                EntityKind::StructDecl | EntityKind::UnionDecl => {
                    if let Some(name) = child.get_name() {
                        self.known_names.insert(name);
                    }
                }
                EntityKind::EnumDecl => {
                    if let Some(name) = child.get_name() {
                        self.known_names.insert(name);
                    }
                }
                EntityKind::TypedefDecl => {
                    if let Some(name) = child.get_name() {
                        self.known_names.insert(name.clone());
                        if let Some(underlying) = child.get_typedef_underlying_type() {
                            if let Some(decl) = underlying.get_canonical_type().get_declaration() {
                                let decl_name = decl.get_name().unwrap_or_default();
                                if decl_name.is_empty() {
                                    // Anonymous underlying type — map display string to typedef name.
                                    let display = format!("{:?}", decl.get_display_name());
                                    self.typedef_to_struct.insert(display, name);
                                } else if decl_name != name {
                                    // Named struct/enum with a different typedef alias.
                                    // Prefer the typedef name (e.g. MY_POINT) over the tag (e.g. _MY_POINT).
                                    self.tag_to_typedef.insert(decl_name, name);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_items(&mut self, tu_entity: Entity<'_>) {
        // Track which struct/union canonical types have already been emitted via
        // a typedef so we don't emit them twice.
        let mut emitted: HashSet<String> = HashSet::new();

        for child in tu_entity.get_children() {
            if !self.in_source(&child) {
                continue;
            }
            match child.get_kind() {
                EntityKind::StructDecl | EntityKind::UnionDecl => {
                    let is_union = child.get_kind() == EntityKind::UnionDecl;
                    let name = match child.get_name() {
                        Some(n) if !n.is_empty() => n,
                        _ => continue, // anonymous at top level — skip
                    };
                    if emitted.contains(&name) {
                        continue;
                    }
                    // Skip forward declarations (no body).
                    if !child.is_definition() {
                        continue;
                    }
                    // If this tag name has a typedef alias, skip it here — the typedef
                    // branch will emit the struct under the cleaner typedef name.
                    if self.tag_to_typedef.contains_key(&name) {
                        emitted.insert(name);
                        continue;
                    }
                    emitted.insert(name.clone());
                    // In C++ mode, an abstract struct is a COM-style interface.
                    if !is_union && child.is_abstract_record() {
                        if let Some(iface) = self.collect_interface(&child, &name) {
                            self.items.push(IrItem::Interface(iface));
                        }
                    } else if let Some(s) = self.collect_struct(&child, &name, is_union) {
                        self.items.push(IrItem::Struct(s));
                    }
                }

                EntityKind::EnumDecl => {
                    let name = match child.get_name() {
                        Some(n) if !n.is_empty() => n,
                        _ => continue,
                    };
                    if emitted.contains(&name) {
                        continue;
                    }
                    if !child.is_definition() {
                        continue;
                    }
                    // Same deduplication: skip tag names with typedef aliases.
                    if self.tag_to_typedef.contains_key(&name) {
                        emitted.insert(name);
                        continue;
                    }
                    emitted.insert(name.clone());
                    if let Some(e) = self.collect_enum(&child, &name) {
                        self.items.push(IrItem::Enum(e));
                    }
                }

                EntityKind::TypedefDecl => {
                    let typedef_name = match child.get_name() {
                        Some(n) if !n.is_empty() => n,
                        _ => continue,
                    };
                    let Some(underlying) = child.get_typedef_underlying_type() else {
                        continue;
                    };
                    let canonical = underlying.get_canonical_type();

                    match canonical.get_kind() {
                        clang::TypeKind::FunctionPrototype
                        | clang::TypeKind::FunctionNoPrototype => {
                            // typedef to a function pointer → callback
                            if emitted.contains(&typedef_name) {
                                continue;
                            }
                            emitted.insert(typedef_name.clone());
                            if let Some(cb) =
                                self.collect_callback_from_type(&typedef_name, canonical)
                            {
                                self.items.push(IrItem::Callback(cb));
                            }
                        }
                        clang::TypeKind::Pointer => {
                            // Might be a pointer-to-function typedef.
                            let Some(inner) = canonical.get_pointee_type() else {
                                continue;
                            };
                            let inner_canon = inner.get_canonical_type();
                            if matches!(
                                inner_canon.get_kind(),
                                clang::TypeKind::FunctionPrototype
                                    | clang::TypeKind::FunctionNoPrototype
                            ) {
                                if emitted.contains(&typedef_name) {
                                    continue;
                                }
                                emitted.insert(typedef_name.clone());
                                if let Some(cb) =
                                    self.collect_callback_from_type(&typedef_name, inner_canon)
                                {
                                    self.items.push(IrItem::Callback(cb));
                                }
                            } else {
                                // Typedef to an opaque pointer (e.g. `typedef void* HANDLE;`) –
                                // we don't need to emit anything special; the type mapping
                                // will resolve it via well-known names.
                            }
                        }
                        clang::TypeKind::Record => {
                            // typedef struct { ... } Foo; — emit the struct under the typedef name.
                            let Some(decl) = canonical.get_declaration() else {
                                continue;
                            };
                            let struct_name = decl.get_name().unwrap_or_default();
                            // If the struct already has the same name as the typedef, it was
                            // already (or will be) emitted in the StructDecl branch.
                            if struct_name == typedef_name {
                                continue;
                            }
                            if !decl.is_definition() {
                                continue;
                            }
                            if emitted.contains(&typedef_name) {
                                continue;
                            }
                            emitted.insert(typedef_name.clone());
                            let is_union = decl.get_kind() == EntityKind::UnionDecl;
                            if let Some(s) = self.collect_struct(&decl, &typedef_name, is_union) {
                                self.items.push(IrItem::Struct(s));
                            }
                        }
                        clang::TypeKind::Enum => {
                            let Some(decl) = canonical.get_declaration() else {
                                continue;
                            };
                            let enum_name = decl.get_name().unwrap_or_default();
                            if enum_name == typedef_name {
                                continue;
                            }
                            if !decl.is_definition() {
                                continue;
                            }
                            if emitted.contains(&typedef_name) {
                                continue;
                            }
                            emitted.insert(typedef_name.clone());
                            if let Some(e) = self.collect_enum(&decl, &typedef_name) {
                                self.items.push(IrItem::Enum(e));
                            }
                        }
                        _ => {}
                    }
                }

                EntityKind::FunctionDecl => {
                    let name = match child.get_name() {
                        Some(n) if !n.is_empty() => n,
                        _ => continue,
                    };
                    // Skip static/inline functions.
                    if child.is_inline_function() {
                        continue;
                    }
                    if emitted.contains(&name) {
                        continue;
                    }
                    emitted.insert(name.clone());
                    if let Some(f) = self.collect_function(&child, &name) {
                        self.items.push(IrItem::Function(f));
                    }
                }

                // C++ class declarations: detect COM interfaces (abstract classes where all
                // methods are pure virtual and have a `__declspec(uuid(...))` attribute).
                EntityKind::ClassDecl => {
                    let name = match child.get_name() {
                        Some(n) if !n.is_empty() => n,
                        _ => continue,
                    };
                    if emitted.contains(&name) {
                        continue;
                    }
                    if !child.is_definition() {
                        continue;
                    }
                    // Only treat abstract classes (all methods pure virtual) as COM interfaces.
                    if !child.is_abstract_record() {
                        continue;
                    }
                    emitted.insert(name.clone());
                    if let Some(iface) = self.collect_interface(&child, &name) {
                        self.items.push(IrItem::Interface(iface));
                    }
                }

                EntityKind::VarDecl => {
                    // const variable declarations — attempt to extract as IrConst.
                    let name = match child.get_name() {
                        Some(n) if !n.is_empty() => n,
                        _ => continue,
                    };
                    if emitted.contains(&name) {
                        continue;
                    }
                    emitted.insert(name.clone());
                    if let Some(c) = self.collect_const(&child, &name) {
                        self.items.push(IrItem::Const(c));
                    }
                }

                _ => {}
            }
        }
    }

    fn collect_struct(&self, entity: &Entity<'_>, name: &str, is_union: bool) -> Option<IrStruct> {
        let mut fields = Vec::new();
        for child in entity.get_children() {
            if child.get_kind() != EntityKind::FieldDecl {
                continue;
            }
            let field_name = child
                .get_name()
                .unwrap_or_else(|| format!("_field{}", fields.len()));
            let field_type = child.get_type().unwrap();

            // Check for fixed-size array.
            let ir_type = if field_type.get_kind() == clang::TypeKind::ConstantArray {
                let elem = field_type.get_element_type().unwrap();
                let size = field_type.get_size().unwrap_or(0) as u64;
                IrType::Array {
                    elem: Box::new(self.resolve_type(elem)),
                    size,
                }
            } else {
                self.resolve_type(field_type)
            };

            fields.push(IrField {
                name: field_name,
                ty: ir_type,
            });
        }
        Some(IrStruct {
            name: name.to_string(),
            fields,
            is_union,
            pack: None,
        })
    }

    fn collect_enum(&self, entity: &Entity<'_>, name: &str) -> Option<IrEnum> {
        // Determine the underlying integer type.
        let underlying = if let Some(int_type) = entity.get_enum_underlying_type() {
            map_type(int_type, &self.known_names)
        } else {
            IrType::Primitive(IrPrimitive::I32)
        };
        let underlying_prim = match underlying {
            IrType::Primitive(p) => p,
            _ => IrPrimitive::I32,
        };

        let mut variants = Vec::new();
        for child in entity.get_children() {
            if child.get_kind() != EntityKind::EnumConstantDecl {
                continue;
            }
            let vname = child.get_name().unwrap_or_default();
            let value = child
                .get_enum_constant_value()
                .map(|(_, u)| u as i64)
                .unwrap_or(0);
            variants.push(IrVariant { name: vname, value });
        }

        // Heuristic: treat as flags if all non-zero values are powers of two.
        let is_flags = !variants.is_empty()
            && variants
                .iter()
                .all(|v| v.value == 0 || v.value.count_ones() == 1);

        Some(IrEnum {
            name: name.to_string(),
            underlying: underlying_prim,
            is_flags,
            variants,
        })
    }

    fn collect_function(&self, entity: &Entity<'_>, name: &str) -> Option<IrFunction> {
        let func_type = entity.get_type()?;
        let cc = func_type
            .get_calling_convention()
            .unwrap_or(clang::CallingConvention::Cdecl);
        let abi = map_calling_convention(cc);

        let mut params = Vec::new();
        for child in entity.get_children() {
            if child.get_kind() != EntityKind::ParmDecl {
                continue;
            }
            let param_name = child
                .get_name()
                .unwrap_or_else(|| format!("param{}", params.len()));
            let param_type = child.get_type().unwrap();
            let ir_type = self.resolve_type(param_type);
            let modifiers = extract_sal_modifiers(&child);
            params.push(IrParam {
                name: param_name,
                ty: ir_type,
                modifiers,
            });
        }

        let ret = if let Some(ret_type) = func_type.get_result_type() {
            self.resolve_type(ret_type)
        } else {
            IrType::Void
        };

        Some(IrFunction {
            name: name.to_string(),
            library: self.default_library.to_string(),
            abi,
            params,
            ret,
            last_error: false,
        })
    }

    fn collect_callback_from_type(
        &self,
        name: &str,
        func_type: clang::Type<'_>,
    ) -> Option<IrCallback> {
        let cc = func_type
            .get_calling_convention()
            .unwrap_or(clang::CallingConvention::Cdecl);
        let abi = map_calling_convention(cc);

        let mut params = Vec::new();
        for (i, arg_type) in func_type
            .get_argument_types()
            .unwrap_or_default()
            .into_iter()
            .enumerate()
        {
            let ir_type = self.resolve_type(arg_type);
            params.push(IrParam {
                name: format!("param{i}"),
                ty: ir_type,
                modifiers: vec![],
            });
        }

        let ret = func_type
            .get_result_type()
            .map(|t| self.resolve_type(t))
            .unwrap_or(IrType::Void);

        Some(IrCallback {
            name: name.to_string(),
            abi,
            params,
            ret,
        })
    }

    fn collect_const(&self, entity: &Entity<'_>, name: &str) -> Option<IrConst> {
        let ty = entity.get_type()?;
        // Only emit if the type is const-qualified.
        if !ty.is_const_qualified() {
            return None;
        }
        // Try to evaluate the constant value.
        let eval = entity.evaluate()?;
        match eval {
            clang::EvaluationResult::SignedInteger(v) => {
                let prim = match map_type(ty, &self.known_names) {
                    IrType::Primitive(p) => p,
                    _ => IrPrimitive::I32,
                };
                Some(IrConst {
                    name: name.to_string(),
                    ty: prim,
                    value: ConstValue::Int(v),
                })
            }
            clang::EvaluationResult::UnsignedInteger(v) => {
                let prim = match map_type(ty, &self.known_names) {
                    IrType::Primitive(p) => p,
                    _ => IrPrimitive::U32,
                };
                Some(IrConst {
                    name: name.to_string(),
                    ty: prim,
                    value: ConstValue::Uint(v),
                })
            }
            clang::EvaluationResult::Float(v) => Some(IrConst {
                name: name.to_string(),
                ty: IrPrimitive::F64,
                value: ConstValue::Float(v),
            }),
            _ => None,
        }
    }

    /// Resolve a Clang type, first consulting well-known Windows type names,
    /// then falling back to canonical type mapping.
    fn resolve_type(&self, ty: clang::Type<'_>) -> IrType {
        // Try well-known name first (works on the non-canonical typedef name).
        let display = ty.get_display_name();
        if let Some(known) = map_well_known(&display) {
            return known;
        }

        // If the display name is itself a known type (e.g. a typedef like HANDLE),
        // return it as a Named type before we strip away the typedef.
        if !display.is_empty() {
            let preferred = self
                .tag_to_typedef
                .get(&display)
                .cloned()
                .unwrap_or_else(|| display.clone());
            if self.known_names.contains(&preferred) {
                return IrType::Named(preferred);
            }
        }

        let canonical = ty.get_canonical_type();

        // Handle pointer types explicitly so the pointee is resolved through our
        // context-aware resolver (which knows about tag→typedef mappings).
        if canonical.get_kind() == clang::TypeKind::Pointer {
            if let Some(pointee) = canonical.get_pointee_type() {
                let is_const = pointee.is_const_qualified();
                let inner = self.resolve_type(pointee);
                return IrType::Ptr {
                    is_const,
                    pointee: Box::new(inner),
                };
            }
        }

        // Handle fixed-size arrays explicitly for the same reason.
        if canonical.get_kind() == clang::TypeKind::ConstantArray {
            if let Some(elem) = canonical.get_element_type() {
                let size = canonical.get_size().unwrap_or(0) as u64;
                return IrType::Array {
                    elem: Box::new(self.resolve_type(elem)),
                    size,
                };
            }
        }

        // If this is a record or enum, resolve via its declaration name.
        if let Some(decl) = canonical.get_declaration() {
            if let Some(dname) = decl.get_name() {
                if !dname.is_empty() {
                    // If this tag name has a typedef alias, prefer the typedef name.
                    let preferred = self.tag_to_typedef.get(&dname).cloned().unwrap_or(dname);
                    if self.known_names.contains(&preferred) {
                        return IrType::Named(preferred);
                    }
                }
            }
        }

        map_type(ty, &self.known_names)
    }

    /// Collect a COM interface from an abstract C++ class declaration.
    ///
    /// Detection heuristic: the class must be abstract (`is_abstract_record()`) and
    /// all non-inherited methods must be pure virtual.  The GUID is extracted from
    /// a `__declspec(uuid("..."))` attribute if present on the class cursor.
    fn collect_interface(&self, entity: &Entity<'_>, name: &str) -> Option<IrInterface> {
        let mut base: Option<String> = None;
        let mut methods = Vec::new();

        // Walk the first base specifier to find the base interface name.
        for child in entity.get_children() {
            match child.get_kind() {
                EntityKind::BaseSpecifier => {
                    if base.is_none() {
                        if let Some(base_type) = child.get_type() {
                            let base_name = base_type.get_display_name();
                            // Strip template parameters and qualifiers if present.
                            let clean = base_name.split('<').next().unwrap_or(&base_name).trim();
                            if !clean.is_empty() && clean != name {
                                base = Some(clean.to_string());
                            }
                        }
                    }
                }
                EntityKind::Method => {
                    // Only include pure virtual methods (the COM vtable).
                    if !child.is_pure_virtual_method() {
                        continue;
                    }
                    let method_name = child.get_name().unwrap_or_default();
                    if method_name.is_empty() {
                        continue;
                    }
                    let func_type = child.get_type().unwrap();
                    let mut params = Vec::new();
                    for param in child.get_children() {
                        if param.get_kind() != EntityKind::ParmDecl {
                            continue;
                        }
                        let pname = param
                            .get_name()
                            .unwrap_or_else(|| format!("p{}", params.len()));
                        let pty = param.get_type().unwrap();
                        let modifiers = extract_sal_modifiers(&param);
                        params.push(IrParam {
                            name: pname,
                            ty: self.resolve_type(pty),
                            modifiers,
                        });
                    }
                    let ret = func_type
                        .get_result_type()
                        .map(|t| self.resolve_type(t))
                        .unwrap_or(IrType::Void);
                    methods.push(IrMethod {
                        name: method_name,
                        params,
                        ret,
                    });
                }
                _ => {}
            }
        }

        // Try to extract the GUID from the `__declspec(uuid(...))` attribute.
        // Clang exposes it through the display name of the UnexposedAttr child.
        let guid = extract_declspec_uuid(entity);

        Some(IrInterface {
            name: name.to_string(),
            guid,
            base,
            methods,
        })
    }
}

/// Attempt to extract a GUID string from a `__declspec(uuid("..."))` attribute on
/// a class entity.  Clang exposes this as an `UnexposedAttr` child whose display
/// name contains the UUID string.
fn extract_declspec_uuid(entity: &Entity<'_>) -> Option<String> {
    for child in entity.get_children() {
        // `__declspec(uuid(...))` appears as an UnexposedAttr in the AST.
        if matches!(child.get_kind(), EntityKind::UnexposedAttr) {
            if let Some(text) = child.get_display_name() {
                // The display name may be something like `uuid("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")`
                // or just the bare GUID string.
                if let Some(guid) = extract_guid_from_text(&text) {
                    return Some(guid);
                }
            }
        }
    }
    None
}

/// Extract a canonical GUID string from free-form text that may include the
/// `uuid("...")` wrapper or be a bare GUID.
fn extract_guid_from_text(text: &str) -> Option<String> {
    // Look for 8-4-4-4-12 hex pattern.
    let pattern = regex_guid(text)?;
    Some(pattern)
}

/// Rudimentary GUID extractor: scan for the typical xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx pattern.
fn regex_guid(s: &str) -> Option<String> {
    // Find a '-' separated sequence of hex groups with the right lengths.
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut i = 0;
    while i + 35 <= n {
        // Try to match an 8-4-4-4-12 pattern starting at i.
        if let Some(guid) = try_match_guid(&chars[i..]) {
            return Some(guid);
        }
        i += 1;
    }
    None
}

fn try_match_guid(chars: &[char]) -> Option<String> {
    const GROUPS: &[usize] = &[8, 4, 4, 4, 12];
    let mut pos = 0;
    let mut parts = Vec::new();
    for (gi, &len) in GROUPS.iter().enumerate() {
        if pos + len > chars.len() {
            return None;
        }
        let group: String = chars[pos..pos + len].iter().collect();
        if !group.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        parts.push(group);
        pos += len;
        if gi < GROUPS.len() - 1 {
            if pos >= chars.len() || chars[pos] != '-' {
                return None;
            }
            pos += 1;
        }
    }
    Some(parts.join("-"))
}

/// Attempt to extract SAL annotation modifiers from a parameter entity.
/// Clang exposes SAL annotations through attribute tokens or annotation attributes.
fn extract_sal_modifiers(param: &Entity<'_>) -> Vec<ParamModifier> {
    let mut mods = Vec::new();

    // Inspect child nodes for annotation attributes that carry SAL text.
    for child in param.get_children() {
        if child.get_kind() == EntityKind::AnnotateAttr {
            if let Some(text) = child.get_display_name() {
                let text = text.to_uppercase();
                if text.contains("_IN_OPT") || text.contains("_INOUT_OPT") {
                    mods.push(ParamModifier::Opt);
                }
                if text.contains("_OUT_OPT") {
                    mods.push(ParamModifier::Opt);
                }
                if text.contains("_IN") || text.contains("_INOUT") {
                    if !mods.contains(&ParamModifier::In) {
                        mods.push(ParamModifier::In);
                    }
                }
                if text.contains("_OUT") || text.contains("_INOUT") {
                    if !mods.contains(&ParamModifier::Out) {
                        mods.push(ParamModifier::Out);
                    }
                }
            }
        }
    }

    mods
}
