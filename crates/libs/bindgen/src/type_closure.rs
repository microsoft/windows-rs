use super::*;

/// Returns `true` if the given method should be considered "included" by the
/// method set for type-closure purposes.
fn method_included_by_set(method: MethodDef, method_set: &MethodSet) -> bool {
    if let Some(overload) = method_overload_name(method) {
        return method_set.includes(&overload);
    }
    method_set.includes(method.name())
}

/// Bottom-up dependency closure for precise filters; dependencies become name-only shells.
pub struct TypeClosure;

impl TypeClosure {
    /// Build the closure and add include rules for discovered types.
    #[track_caller]
    pub fn build(
        reader: &Reader,
        filter: &mut Filter,
        references: &References,
        implements: Option<&Implements>,
    ) -> TypeMap {
        let mut types = TypeMap::new();

        // Interface method seeds pull in only the requested signatures.
        for ((namespace, name), method_set) in &filter.requested_interfaces {
            for ty in reader.with_full_name(namespace, name) {
                types.insert(ty.clone());

                if let Type::Interface(iface) = &ty {
                    if filter.includes_full_hierarchy(namespace, name) {
                        for required in iface.required_interfaces(reader) {
                            Type::Interface(required.clone())
                                .combine_closure(&mut types, reader, references);
                        }
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

        for (namespace, name) in &filter.direct_types {
            for ty in reader.with_full_name(namespace, name) {
                ty.combine_closure(&mut types, reader, references);
                types.insert(ty.clone());

                if let Type::Interface(interface) = &ty
                    && implements
                        .is_some_and(|implements| implements.matches(interface.type_name()))
                {
                    for method in interface.def.methods() {
                        let sig = method.method_signature(&interface.generics, reader);
                        for dep_ty in sig.types() {
                            dep_ty.combine_closure(&mut types, reader, references);
                        }
                    }
                }

                // Unscoped enum variants are standalone constants; include the requested set
                // explicitly.
                if let Type::CppEnum(e) = &ty
                    && !e.def.has_attribute("ScopedEnumAttribute")
                    && let Some(variant_set) =
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

        // Add type-level rules for every discovered type.
        for type_name in types.keys() {
            if type_name.namespace().is_empty() {
                continue;
            }
            let rule = format!("{}.{}", type_name.namespace(), type_name.name());
            if !filter.rules.iter().any(|(r, _)| r == &rule) {
                filter.rules.push((rule, true));
            }
        }

        filter.rules.sort_unstable_by(|left, right| {
            let left = (left.0.len(), !left.1);
            let right = (right.0.len(), !right.1);
            left.cmp(&right).reverse()
        });

        types
    }
}

/// Bottom-up dependency walk that avoids pulling full interface surfaces.
trait CombineClosure {
    fn combine_closure(&self, types: &mut TypeMap, reader: &Reader, references: &References);
}

impl CombineClosure for Type {
    fn combine_closure(&self, types: &mut TypeMap, reader: &Reader, references: &References) {
        let ty = self.decay();

        if ty.is_intrinsic() {
            return;
        }

        // Referenced crates own the outer type, but generic arguments may be local.
        let tn = ty.type_name();
        if !tn.namespace().is_empty() && references.contains(tn).is_some() {
            let (_ty_inner, generics) = ty.split_generic(reader);
            for g in &generics {
                g.combine_closure(types, reader, references);
            }
            return;
        }

        let (ty_inner, generics) = ty.split_generic(reader);
        for g in &generics {
            g.combine_closure(types, reader, references);
        }

        // Insert generic definitions, not concrete specializations.
        let insert_ty = if generics.is_empty() {
            ty.clone()
        } else {
            ty_inner.clone()
        };

        // Core types have empty namespaces but are needed for standalone `--sys` output.
        let insert_tn = insert_ty.type_name();
        if (!insert_tn.namespace().is_empty() || insert_ty.is_core()) && !types.insert(insert_ty) {
            return;
        }

        // Pull every arch-split sibling so each target has a definition behind its cfg gate.
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
                // Dependency interfaces need identity and hierarchy, not full method surfaces.
                Self::Object.combine_closure(types, reader, references);
            }
            Self::CppInterface(iface) => {
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
                // Standalone unscoped enum constants still need their owning enum alias.
                let field_ty = c.field.field_type(None, reader);
                field_ty.combine_closure(types, reader, references);
            }
            Self::Class(c) => {
                // Classes pull in only the default interface unless other interfaces are
                // explicit seeds.
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
