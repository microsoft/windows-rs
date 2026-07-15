use super::*;

/// Returns `true` if the given method should be considered "included" by the
/// method set for type-closure purposes.
fn method_included_by_set(method: MethodDef, method_set: &MethodSet) -> bool {
    if let Some(overload) = method_overload_name(method) {
        return method_set.includes(&overload);
    }
    method_set.includes(method.name())
}

/// Computes the bottom-up type closure for a precise (non-broad, non-package)
/// filter — the projection whose seeds are the exact types and methods named in
/// the [`Filter`].
///
/// Starting from those seeds, this walks method signatures recursively to
/// discover only the types actually needed by the requested API surface; types
/// reached only as dependencies are pulled in as name-only shells. This is the
/// seeding path for every style whose filter names things precisely (`--sys`,
/// `--minimal`, and the default), as opposed to the top-down [`TypeMap::filter`]
/// namespace scan used for broad filters and `--package`.
pub struct TypeClosure;

impl TypeClosure {
    /// Build a `TypeMap` containing only the types required by the methods and
    /// types listed in `filter`. Also adds type-level include rules to the
    /// filter for all discovered types.
    #[track_caller]
    pub fn build(reader: &Reader, filter: &mut Filter, references: &References) -> TypeMap {
        let mut types = TypeMap::new();

        // 1. Process interface method entries — for each requested interface,
        //    include the interface itself and walk the signatures of requested
        //    methods to pull in their dependent types.
        for ((namespace, name), method_set) in &filter.requested_interfaces {
            for ty in reader.with_full_name(namespace, name) {
                types.insert(ty.clone());

                if let Type::Interface(iface) = &ty {
                    for required in iface.required_interfaces(reader) {
                        let req_tn = Type::Interface(required.clone()).type_name();
                        if references.contains(req_tn).is_some() {
                            for g in &required.generics {
                                g.combine_closure(&mut types, reader, references);
                            }
                            continue;
                        }
                        types.insert(Type::Interface(required.clone()));
                        Type::Object.combine_closure(&mut types, reader, references);
                    }

                    for method in iface.def.methods() {
                        if method_included_by_set(method, method_set) {
                            let sig = method.method_signature(&iface.generics, reader);
                            for dep_ty in sig.types() {
                                dep_ty.combine_closure(&mut types, reader, references);
                            }
                        }
                    }
                } else if let Type::CppInterface(iface) = &ty {
                    for base in iface.base_interfaces(reader) {
                        base.combine_closure(&mut types, reader, references);
                    }
                    for method in iface.def.methods() {
                        if method_included_by_set(method, method_set) {
                            let sig = method.method_signature(&[], reader);
                            for dep_ty in sig.types() {
                                dep_ty.combine_closure(&mut types, reader, references);
                            }
                        }
                    }
                }
            }
        }

        // 2. Process directly-included types (functions, structs, enums, etc.)
        for (namespace, name) in &filter.direct_types {
            for ty in reader.with_full_name(namespace, name) {
                ty.combine_closure(&mut types, reader, references);
                types.insert(ty.clone());

                // An unscoped (C-style) enum named directly in the filter projects
                // its variants. Unlike scoped/WinRT enums (whose variants are
                // associated consts of the type), these variants are surfaced as
                // standalone `Enum_Member` constants, so they must be pulled into the
                // closure explicitly — otherwise the enum projects as a bare
                // `pub type Enum = i32;` with no values. The recorded variant set
                // decides which come along: a bare enum records `All`; `Enum::{A, B}`
                // records just those; an explicit shell `Enum::{}` records an empty
                // set (alias only). Enums reached only as a dependency record no set
                // and stay name-only too. Member constants requested individually
                // (`Enum_Member`) are independent direct types, so they are unaffected
                // by this set.
                if let Type::CppEnum(e) = &ty {
                    if !e.def.has_attribute("ScopedEnumAttribute") {
                        if let Some(variant_set) =
                            filter.enum_variant_filter(e.def.namespace(), e.def.name())
                        {
                            let enum_arches = e.def.arches();
                            for field in e.def.fields() {
                                if field.flags().contains(FieldAttributes::Literal)
                                    && variant_set.includes(field.name())
                                {
                                    Type::CppConst(CppConst {
                                        namespace: e.def.namespace(),
                                        field,
                                        enum_arches,
                                        is_enum_member: true,
                                    })
                                    .combine_closure(&mut types, reader, references);
                                }
                            }
                        }
                    }
                }
            }
        }

        // 3. Add type-level include rules for every discovered type so the
        //    codegen pipeline's includes_type_name / includes_namespace work.
        for type_name in types.keys() {
            if type_name.namespace().is_empty() {
                continue;
            }
            let rule = format!("{}.{}", type_name.namespace(), type_name.name());
            if !filter.rules.iter().any(|(r, _)| r == &rule) {
                filter.rules.push((rule, true));
            }
        }

        // Re-sort rules so precedence (longer/more-specific first) is maintained.
        filter.rules.sort_unstable_by(|left, right| {
            let left = (left.0.len(), !left.1);
            let right = (right.0.len(), !right.1);
            left.cmp(&right).reverse()
        });

        types
    }
}

/// Extension trait providing the bottom-up closure dependency walk.
///
/// Unlike the full `Dependencies::combine`, this only pulls in types that are
/// directly referenced (struct fields, enum bases, delegate signatures) without
/// greedily pulling entire interface hierarchies for every interface encountered.
trait CombineClosure {
    fn combine_closure(&self, types: &mut TypeMap, reader: &Reader, references: &References);
}

impl CombineClosure for Type {
    fn combine_closure(&self, types: &mut TypeMap, reader: &Reader, references: &References) {
        let ty = self.decay();

        if ty.is_intrinsic() {
            return;
        }

        // Skip types owned by references (they come from external crates),
        // but still recurse into generic arguments which may be local types.
        let tn = ty.type_name();
        if !tn.namespace().is_empty() && references.contains(tn).is_some() {
            // Still need to process generic args (e.g., IVector<LocalType>).
            let (_ty_inner, generics) = ty.split_generic(reader);
            for g in &generics {
                g.combine_closure(types, reader, references);
            }
            return;
        }

        // Split off generic args and recurse into them.
        let (ty_inner, generics) = ty.split_generic(reader);
        for g in &generics {
            g.combine_closure(types, reader, references);
        }

        // Insert the base (non-specialized) type into the map.
        // For generic types like TypedEventHandler<Foo, Bar>, we insert the
        // unspecialized TypedEventHandler so codegen emits the generic struct
        // definition rather than a concrete specialization.
        let insert_ty = if generics.is_empty() {
            ty.clone()
        } else {
            ty_inner.clone()
        };

        // Insert the type and stop on repeats to avoid infinite recursion. Core
        // types (`GUID`, `HRESULT`, `BOOL`, `PCWSTR`, `IUnknown`, …) carry an empty
        // namespace; they are still inserted so a standalone `--sys` crate emits
        // their local definitions (`write_no_deps`). Non-sys crates carry them in
        // the map harmlessly because `write_no_deps` only emits when
        // `uses_inline_core_types()`.
        let insert_tn = insert_ty.type_name();
        if (!insert_tn.namespace().is_empty() || insert_ty.is_core()) && !types.insert(insert_ty) {
            return;
        }

        // Fan out to architecture-split siblings. An arch-divergent Win32 type
        // (e.g. `FARPROC` = `isize` on 64-bit / `i32` on x86, or a per-arch
        // `CONTEXT`) is emitted as one TypeDef per architecture sharing a name.
        // A referencing signature only resolves to a single variant, so the
        // others must be pulled in explicitly or the target architectures that
        // use them would be left with an undefined type. This mirrors the
        // arch-sibling walk in the full `Dependencies::combine`, extended to
        // cover delegates as well. Each sibling carries its own
        // `#[cfg(target_arch = ...)]` gate in codegen.
        let siblings: Vec<Self> = match &ty_inner {
            Self::CppStruct(s) => reader
                .with_full_name(s.def.namespace(), s.def.name())
                .collect(),
            Self::CppDelegate(d) => reader
                .with_full_name(d.def.namespace(), d.def.name())
                .collect(),
            Self::CppEnum(e) => reader
                .with_full_name(e.def.namespace(), e.def.name())
                .collect(),
            Self::CppFn(f) => reader
                .with_full_name(f.namespace, f.method.name())
                .collect(),
            _ => Vec::new(),
        };
        for sibling in siblings {
            if sibling != ty_inner {
                sibling.combine_closure(types, reader, references);
            }
        }

        match &ty_inner {
            Self::Struct(s) => {
                for field in s.def.fields() {
                    let field_ty = field.field_type(None, reader);
                    field_ty.combine_closure(types, reader, references);
                }
            }
            Self::CppStruct(s) => {
                for field in s.def.fields() {
                    let field_ty = field.field_type(Some(s), reader);
                    field_ty.combine_closure(types, reader, references);
                }
            }
            Self::Enum(_) | Self::CppEnum(_) => {}
            Self::Delegate(d) => {
                for method in d.def.methods() {
                    if method.name() == "Invoke" {
                        let sig = method.method_signature(&d.generics, reader);
                        for dep_ty in sig.types() {
                            dep_ty.combine_closure(types, reader, references);
                        }
                    }
                }
            }
            Self::CppDelegate(d) => {
                for method in d.def.methods() {
                    if method.name() == "Invoke" {
                        let sig = method.method_signature(&[], reader);
                        for dep_ty in sig.types() {
                            dep_ty.combine_closure(types, reader, references);
                        }
                    }
                }
            }
            Self::Interface(_iface) => {
                // For interfaces pulled in as dependencies (not explicitly
                // requested), we only need the struct/IID/hierarchy — no need
                // to recursively pull in all their method signature types.
                // The hierarchy is handled by the caller.
                Self::Object.combine_closure(types, reader, references);
            }
            Self::CppInterface(iface) => {
                // Pull in base interfaces so vtable/Deref/hierarchy work.
                for base in iface.base_interfaces(reader) {
                    base.combine_closure(types, reader, references);
                }
                Self::IUnknown.combine_closure(types, reader, references);
            }
            Self::CppFn(f) => {
                let sig = f.method.method_signature(&[], reader);
                for dep_ty in sig.types() {
                    dep_ty.combine_closure(types, reader, references);
                }
                if let Some(dependency) = f.window_long_dependency() {
                    reader
                        .unwrap_full_name(f.namespace, dependency)
                        .combine_closure(types, reader, references);
                }
            }
            Self::CppConst(c) => {
                // Pull in the constant's declared type. Non-scoped enum variants
                // are surfaced as standalone constants whose type is the owning
                // enum; without this the enum's type alias (e.g. `pub type CLSCTX
                // = u32;`) is never emitted, leaving the constant dangling.
                let field_ty = c.field.field_type(None, reader);
                field_ty.combine_closure(types, reader, references);
            }
            Self::Class(c) => {
                // In the closure, only pull in the default interface (needed for
                // Deref and class identity). All other instance/base interfaces are
                // only included if they appear explicitly in the filter's interfaces
                // map (which is handled by step 1 of build()).
                for iface in c.required_interfaces(reader) {
                    if matches!(
                        iface.kind,
                        InterfaceKind::Static | InterfaceKind::Composable
                    ) {
                        continue;
                    }
                    if iface.kind != InterfaceKind::Default {
                        continue;
                    }
                    let iface_ty = Self::Interface(iface.clone());
                    let iface_tn = iface_ty.type_name();
                    if references.contains(iface_tn).is_some() {
                        for g in &iface.generics {
                            g.combine_closure(types, reader, references);
                        }
                        continue;
                    }
                    iface_ty.combine_closure(types, reader, references);
                }
                // Pull in base classes for the required_hierarchy! macro.
                let mut def = c.def;
                loop {
                    let extends = def.extends().unwrap();
                    if extends == (TypeName::Object.0, TypeName::Object.1) {
                        break;
                    }
                    let base = reader.unwrap_full_name(extends.namespace(), extends.name());
                    base.combine_closure(types, reader, references);
                    if let Self::Class(base_class) = &base {
                        def = base_class.def;
                    } else {
                        break;
                    }
                }
            }
            Self::IUnknown => {
                Self::GUID.combine_closure(types, reader, references);
                Self::HRESULT.combine_closure(types, reader, references);
            }
            Self::Object => {
                Self::IUnknown.combine_closure(types, reader, references);
            }
            _ => {}
        }
    }
}
