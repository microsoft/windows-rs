use super::*;

/// Computes the minimal type closure for `--minimal` mode.
///
/// Starting from the methods and types listed in a [`MinimalFilter`], this
/// walks method signatures recursively to discover only the types that are
/// actually needed by the requested API surface.
pub struct MinimalTypeMap;

impl MinimalTypeMap {
    /// Build a `TypeMap` containing only the types required by the methods and
    /// types listed in `filter`. Also returns a `Filter` compatible with the
    /// existing codegen pipeline (for method demotion queries).
    #[track_caller]
    pub fn build(
        reader: &Reader,
        minimal_filter: &MinimalFilter,
        references: &References,
    ) -> (TypeMap, Filter) {
        let mut types = TypeMap::new();

        // 1. Process interface method entries — for each requested interface,
        //    include the interface itself and walk the signatures of requested
        //    methods to pull in their dependent types.
        for ((namespace, name), method_set) in &minimal_filter.interfaces {
            // Include the interface type itself.
            for ty in reader.with_full_name(namespace, name) {
                types.insert(ty.clone());

                // Walk the interface hierarchy so QI / casts work.
                if let Type::Interface(iface) = &ty {
                    for required in iface.required_interfaces(reader) {
                        let req_tn = Type::Interface(required.clone()).type_name();
                        if references.contains(req_tn).is_some() {
                            // Still recurse into generic args of reference interfaces.
                            for g in &required.generics {
                                g.combine_minimal(&mut types, reader, references);
                            }
                            continue;
                        }
                        types.insert(Type::Interface(required.clone()));
                        // Also pull in the required interface's type dependencies
                        // (IUnknown, Object, etc.) minimally.
                        Type::Object.combine_minimal(&mut types, reader, references);
                    }

                    // Walk method signatures for the requested methods.
                    // Use the standard `combine` (not `combine_minimal`) so the
                    // type closure exactly matches what `Method::dependencies`
                    // computes at codegen time.
                    for method in iface.def.methods() {
                        if method_set.includes(method.name()) {
                            let sig = method.method_signature(
                                iface.def.namespace(),
                                &iface.generics,
                                reader,
                            );
                            let mut sig_deps = TypeMap::new();
                            sig.combine(&mut sig_deps, reader);
                            types.combine_references(&sig_deps, references);
                        }
                    }
                } else if let Type::CppInterface(iface) = &ty {
                    // Win32 COM interface — walk requested method signatures.
                    for method in iface.def.methods() {
                        if method_set.includes(method.name()) {
                            let sig = method.method_signature(iface.def.namespace(), &[], reader);
                            let mut sig_deps = TypeMap::new();
                            sig.combine(&mut sig_deps, reader);
                            types.combine_references(&sig_deps, references);
                        }
                    }
                }
            }
        }

        // 2. Process directly-included types (functions, structs, enums, etc.)
        for (namespace, name) in &minimal_filter.types {
            for ty in reader.with_full_name(namespace, name) {
                ty.combine_minimal(&mut types, reader, references);
            }
        }

        // 3. Build a Filter that the existing codegen can query for method
        //    demotion. We construct include rules for every type in `types`,
        //    and configure method filters based on the MinimalFilter.
        let filter = Self::build_filter(reader, minimal_filter, &types);

        (types, filter)
    }

    /// Build a `Filter` compatible with the existing codegen.
    ///
    /// For minimal mode, every type in the type map is "included", and method
    /// filters demote all methods not explicitly requested.
    fn build_filter(reader: &Reader, _minimal_filter: &MinimalFilter, types: &TypeMap) -> Filter {
        // In minimal mode the Filter only needs type-level includes.
        // Method-level emission is handled directly by Config::includes_method
        // querying the MinimalFilter with raw names.
        let mut include: Vec<&str> = Vec::new();

        for type_name in types.keys() {
            if type_name.namespace().is_empty() {
                continue;
            }
            let full = format!("{}.{}", type_name.namespace(), type_name.name());
            include.push(full.leak());
        }

        let exclude: Vec<&str> = Vec::new();
        Filter::new(reader, &include, &exclude)
    }
}

/// Extension trait providing a minimal-mode dependency walk.
///
/// Unlike the full `Dependencies::combine`, this only pulls in types that are
/// directly referenced (struct fields, enum bases, delegate signatures) without
/// greedily pulling entire interface hierarchies for every interface encountered.
trait CombineMinimal {
    fn combine_minimal(&self, types: &mut TypeMap, reader: &Reader, references: &References);
}

impl CombineMinimal for Type {
    fn combine_minimal(&self, types: &mut TypeMap, reader: &Reader, references: &References) {
        let ty = self.decay();

        if ty.is_intrinsic() {
            return;
        }

        // Skip types owned by references (they come from external crates).
        let tn = ty.type_name();
        if !tn.namespace().is_empty() && references.contains(tn).is_some() {
            return;
        }

        // Split off generic args and recurse into them.
        let (ty_inner, generics) = ty.split_generic(reader);
        for g in &generics {
            g.combine_minimal(types, reader, references);
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

        // Avoid infinite recursion: if already in the map, stop.
        let insert_tn = insert_ty.type_name();
        if !insert_tn.namespace().is_empty() && !types.insert(insert_ty) {
            return;
        }

        match &ty_inner {
            Type::Struct(s) => {
                // Pull in field types.
                for field in s.def.fields() {
                    let field_ty = field.field_type(None, reader);
                    field_ty.combine_minimal(types, reader, references);
                }
            }
            Type::CppStruct(s) => {
                // Pull in field types.
                for field in s.def.fields() {
                    let field_ty = field.field_type(Some(s), reader);
                    field_ty.combine_minimal(types, reader, references);
                }
            }
            Type::Enum(_) | Type::CppEnum(_) => {
                // Enums are self-contained, no further deps needed.
            }
            Type::Delegate(d) => {
                // Pull in the delegate's invoke signature types.
                for method in d.def.methods() {
                    if method.name() == "Invoke" {
                        let sig = method.method_signature(d.def.namespace(), &d.generics, reader);
                        for dep_ty in sig.types() {
                            dep_ty.combine_minimal(types, reader, references);
                        }
                    }
                }
            }
            Type::CppDelegate(d) => {
                for method in d.def.methods() {
                    if method.name() == "Invoke" {
                        let sig = method.method_signature(d.def.namespace(), &[], reader);
                        for dep_ty in sig.types() {
                            dep_ty.combine_minimal(types, reader, references);
                        }
                    }
                }
            }
            Type::Interface(_iface) => {
                // For interfaces pulled in as dependencies (not explicitly
                // requested), we only need the struct/IID/hierarchy — no need
                // to recursively pull in all their method signature types.
                // The hierarchy is handled by the caller.
                Type::Object.combine_minimal(types, reader, references);
            }
            Type::CppInterface(_) => {
                Type::IUnknown.combine_minimal(types, reader, references);
            }
            Type::CppFn(f) => {
                // Pull in the function's signature types.
                let sig = f.method.method_signature(f.namespace, &[], reader);
                for dep_ty in sig.types() {
                    dep_ty.combine_minimal(types, reader, references);
                }
            }
            Type::CppConst(_) => {
                // Constants are self-contained.
            }
            Type::Class(c) => {
                // Pull in all required interfaces (instance, factory, statics,
                // composable, base) so class codegen can reference them.
                for iface in c.required_interfaces(reader) {
                    let iface_ty = Type::Interface(iface.clone());
                    let iface_tn = iface_ty.type_name();
                    if references.contains(iface_tn).is_some() {
                        // Even though the interface itself is from a reference crate,
                        // its generic arguments might be local types that need inclusion.
                        for g in &iface.generics {
                            g.combine_minimal(types, reader, references);
                        }
                        continue;
                    }
                    // Use combine_minimal so that the interface's own dependencies
                    // (e.g. method signature types for factories) are walked.
                    iface_ty.combine_minimal(types, reader, references);
                }
                // Pull in base classes for the required_hierarchy! macro.
                let mut def = c.def;
                loop {
                    let extends = def.extends().unwrap();
                    if extends == (TypeName::Object.0, TypeName::Object.1) {
                        break;
                    }
                    let base = reader.unwrap_full_name(extends.namespace(), extends.name());
                    base.combine_minimal(types, reader, references);
                    if let Type::Class(base_class) = &base {
                        def = base_class.def;
                    } else {
                        break;
                    }
                }
            }
            Type::IUnknown => {
                Type::GUID.combine_minimal(types, reader, references);
                Type::HRESULT.combine_minimal(types, reader, references);
            }
            Type::Object => {
                Type::IUnknown.combine_minimal(types, reader, references);
            }
            _ => {}
        }
    }
}
