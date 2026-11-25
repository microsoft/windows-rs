use super::*;

/// Bottom-up dependency closure for precise filters; dependencies become name-only shells.
pub struct TypeClosure;

/// Inputs shared by every step of the dependency walk.
struct Walk<'a> {
    reader: &'a Reader,
    references: &'a References,
    filter: &'a Selection,
    style: Style,
}

impl TypeClosure {
    /// Build the dependency closure of the selected types and methods.
    #[track_caller]
    pub fn build(
        reader: &Reader,
        filter: &Selection,
        references: &References,
        implements: Option<&Implements>,
        style: Style,
    ) -> (TypeMap, HashSet<TypeName>) {
        let mut types = TypeMap::new();
        let mut implemented = HashSet::new();
        let cx = Walk {
            reader,
            references,
            filter,
            style,
        };

        for name in &filter.roots {
            for ty in reader.with_full_name(name.namespace(), name.name()) {
                if filter.excludes_type_name(ty.type_name()) {
                    continue;
                }
                if filter.methods.contains_key(name) {
                    types.insert(ty.clone());
                    ty.combine_definition_closure(&mut types, &cx, ALL_ARCHES);
                } else {
                    ty.combine_root(&mut types, &cx);
                    types.insert(ty.clone());
                }

                if let Type::Interface(iface) = &ty {
                    if filter.includes_full_hierarchy(name.namespace(), name.name()) {
                        for required in iface.required_interfaces(reader) {
                            Type::Interface(required).combine_root(&mut types, &cx);
                        }
                    }
                    for method in iface.def.methods() {
                        if filter.includes_method(ty.type_name(), method) {
                            let sig = method.method_signature(&iface.generics, reader);
                            for dep_ty in sig.types() {
                                dep_ty.combine_root(&mut types, &cx);
                            }
                        }
                    }
                } else if let Type::CppInterface(iface) = &ty {
                    for method in iface.def.methods() {
                        if filter.includes_method(ty.type_name(), method) {
                            for dependency in method.associated_enums(reader) {
                                dependency.combine_root(&mut types, &cx);
                            }
                            let sig = method.method_signature(&[], reader);
                            for dep_ty in sig.types() {
                                dep_ty.combine_root(&mut types, &cx);
                            }
                        }
                    }
                }
            }
        }

        // Implementation signatures can introduce more interfaces to implement.
        if let Some(implements) = implements {
            let mut visited = HashSet::new();
            loop {
                let pending: Vec<_> = types
                    .values()
                    .flatten()
                    .filter(|ty| {
                        matches!(ty, Type::Interface(_) | Type::CppInterface(_))
                            && (implements.matches(ty.type_name())
                                || implemented.contains(&ty.type_name()))
                            && visited.insert((*ty).clone())
                    })
                    .cloned()
                    .collect();
                if pending.is_empty() {
                    break;
                }
                for ty in pending {
                    implemented.insert(ty.type_name());
                    let (def, generics, required) = match &ty {
                        Type::Interface(interface) => (
                            interface.def,
                            interface.generics.as_slice(),
                            interface
                                .required_interfaces(reader)
                                .into_iter()
                                .map(Type::Interface)
                                .collect(),
                        ),
                        Type::CppInterface(interface) => {
                            (interface.def, &[][..], interface.base_interfaces(reader))
                        }
                        _ => unreachable!(),
                    };
                    for required in required {
                        if !matches!(required, Type::Interface(_) | Type::CppInterface(_)) {
                            continue;
                        }
                        required.combine_root(&mut types, &cx);
                        if types.contains_key(&required.type_name()) {
                            implemented.insert(required.type_name());
                        }
                    }
                    for method in def.methods() {
                        if matches!(ty, Type::CppInterface(_)) {
                            for dependency in method.associated_enums(reader) {
                                dependency.combine_root(&mut types, &cx);
                            }
                        }
                        let sig = method.method_signature(generics, reader);
                        for dep_ty in sig.types() {
                            dep_ty.combine_root(&mut types, &cx);
                        }
                    }
                }
            }
        }

        (types, implemented)
    }
}

/// Bottom-up dependency walk that avoids pulling full interface surfaces.
trait CombineClosure {
    /// Walks dependencies reached from a selected item on every architecture.
    fn combine_root(&self, types: &mut TypeMap, cx: &Walk) {
        self.combine_closure(types, cx, ALL_ARCHES);
    }

    fn combine_closure(&self, types: &mut TypeMap, cx: &Walk, arches: i32);

    fn combine_abi_closure(&self, types: &mut TypeMap, cx: &Walk, arches: i32);

    fn combine_definition_closure(&self, types: &mut TypeMap, cx: &Walk, arches: i32);
}

impl CombineClosure for Type {
    fn combine_abi_closure(&self, types: &mut TypeMap, cx: &Walk, arches: i32) {
        if self.is_interface_with_reader(cx.reader) {
            // Keep interface identities for callers constructing ABI outputs, but not erased aliases.
            self.projection_type(cx.reader)
                .combine_closure(types, cx, arches);
            return;
        }
        match self {
            Self::PtrMut(inner, _) | Self::PtrConst(inner, _) | Self::ArrayFixed(inner, _) => {
                inner.combine_abi_closure(types, cx, arches);
            }
            _ => self.combine_closure(types, cx, arches),
        }
    }

    fn combine_closure(&self, types: &mut TypeMap, cx: &Walk, arches: i32) {
        let ty = self.dependency_type();

        if let Self::Ptr64(inner, _) = ty {
            types.insert(Self::Pointer64Helper);
            inner.combine_closure(types, cx, arches);
            return;
        }
        if let Self::FnPtr(signature) = ty {
            for ty in signature.types() {
                ty.combine_closure(types, cx, arches);
            }
            return;
        }
        if ty.is_intrinsic() {
            return;
        }

        if let Some(owner) = ty.enclosing() {
            for owner in cx.reader.with_full_name(owner.namespace(), owner.name()) {
                owner.combine_closure(types, cx, arches);
            }
            return;
        }

        // Referenced crates own the outer type, but generic arguments may be local.
        let tn = ty.type_name();
        if !tn.namespace().is_empty() && cx.references.contains(tn).is_some() {
            let (_ty_inner, generics) = ty.split_generic(cx.reader);
            for g in &generics {
                g.combine_closure(types, cx, arches);
            }
            return;
        }

        let (ty_inner, generics) = ty.split_generic(cx.reader);
        for g in &generics {
            g.combine_closure(types, cx, arches);
        }

        // Insert generic definitions, not concrete specializations.
        let insert_ty = if generics.is_empty() {
            ty.clone()
        } else {
            ty_inner.clone()
        };

        // Pull arch-split siblings reached from `arches` so each target has a definition behind
        // its cfg gate.
        let siblings: Vec<Self> = match &ty_inner {
            Self::CppStruct(s) => cx
                .reader
                .with_full_name(s.def.namespace(), s.def.name())
                .collect(),
            Self::CppDelegate(d) => cx
                .reader
                .with_full_name(d.def.namespace(), d.def.name())
                .collect(),
            Self::CppEnum(e) => cx
                .reader
                .with_full_name(e.def.namespace(), e.def.name())
                .collect(),
            Self::CppFn(f) => cx
                .reader
                .with_full_name(f.namespace, f.method.name())
                .collect(),
            _ => Vec::new(),
        };
        let candidates = std::iter::once((insert_ty, ty_inner.clone())).chain(
            siblings
                .into_iter()
                .filter(|sibling| *sibling != ty_inner)
                .map(|sibling| (sibling.clone(), sibling)),
        );
        for (insert_ty, definition) in candidates {
            let own = insert_ty.arches();
            let reached = arches & arch_mask(own);
            if reached == 0 {
                continue;
            }
            // Core types have empty namespaces but are needed for standalone `--sys` output.
            let insert_tn = insert_ty.type_name();
            // Excluded dependencies are left for rustc to report rather than generated. Enum
            // members follow their enum; their own exclusions filter variants when written.
            let excluded = match &insert_ty {
                Self::CppConst(c) if c.is_enum_member => c
                    .value_type(cx.reader)
                    .is_some_and(|ty| cx.filter.excludes_type_name(ty.type_name())),
                _ => cx.filter.excludes_type_name(insert_tn),
            };
            if excluded {
                continue;
            }
            if (!insert_tn.namespace().is_empty() || insert_ty.is_core())
                && !types.insert_reached(insert_ty, reached)
            {
                continue;
            }
            definition.combine_definition_closure(types, cx, reached);
        }
    }

    fn combine_definition_closure(&self, types: &mut TypeMap, cx: &Walk, arches: i32) {
        match self {
            Self::Struct(s) => {
                for field in s.def.fields() {
                    let field_ty = field.field_type(None, cx.reader);
                    field_ty.combine_closure(types, cx, arches);
                }
            }
            Self::CppStruct(s) => {
                for ty in s.definition_types(cx.reader) {
                    ty.combine_closure(types, cx, arches);
                }
            }
            Self::Enum(_) | Self::CppEnum(_) => {}
            Self::Delegate(d) => {
                for method in d.def.methods() {
                    if method.name() == "Invoke" {
                        let sig = method.method_signature(&d.generics, cx.reader);
                        for dep_ty in sig.types() {
                            dep_ty.combine_closure(types, cx, arches);
                        }
                    }
                }
            }
            Self::CppDelegate(d) => {
                for method in d.def.methods() {
                    if method.name() == "Invoke" {
                        for dependency in method.associated_enums(cx.reader) {
                            dependency.combine_closure(types, cx, arches);
                        }
                        let sig = method.method_signature(&[], cx.reader);
                        for dep_ty in sig.types() {
                            dep_ty.combine_closure(types, cx, arches);
                        }
                    }
                }
            }
            Self::Interface(_iface) => {
                // Dependency interfaces need identity and hierarchy, not full method surfaces.
                Self::Object.combine_closure(types, cx, arches);
            }
            Self::CppInterface(iface) => {
                for base in iface.base_interfaces(cx.reader) {
                    base.combine_closure(types, cx, arches);
                }
            }
            Self::CppFn(f) => {
                for dependency in f.method.associated_enums(cx.reader) {
                    dependency.combine_closure(types, cx, arches);
                }
                let sig = f.method.method_signature(&[], cx.reader);
                sig.return_type.combine_closure(types, cx, arches);
                for param in &sig.params {
                    if cx.style.is_minimal() {
                        param.ty.combine_abi_closure(types, cx, arches);
                    } else {
                        param.ty.combine_closure(types, cx, arches);
                    }
                }
                if let Some(dependency) = f.window_long_dependency(cx.reader) {
                    cx.reader
                        .unwrap_full_name(f.namespace, dependency)
                        .combine_closure(types, cx, arches);
                }
            }
            Self::CppConst(c) => {
                if let Some(ty) = c.value_type(cx.reader) {
                    ty.combine_closure(types, cx, arches);
                }
            }
            Self::Class(c) => {
                for iface in c.required_interfaces(cx.reader) {
                    if matches!(
                        iface.kind,
                        InterfaceKind::Static | InterfaceKind::Composable
                    ) {
                        continue;
                    }
                    if iface.kind != InterfaceKind::Default
                        && !cx
                            .filter
                            .includes_hierarchy(c.def.namespace(), c.def.name(), &iface)
                    {
                        continue;
                    }
                    Self::Interface(iface).combine_closure(types, cx, arches);
                }

                let mut def = c.def;
                loop {
                    let extends = def.extends().unwrap();
                    if extends == (TypeName::Object.0, TypeName::Object.1) {
                        break;
                    }
                    let base = cx
                        .reader
                        .unwrap_type_name(extends.namespace(), extends.name());
                    base.combine_closure(types, cx, arches);
                    if let Self::Class(base_class) = &base {
                        def = base_class.def;
                    } else {
                        break;
                    }
                }
            }
            Self::IUnknown => {
                Self::GUID.combine_closure(types, cx, arches);
                Self::HRESULT.combine_closure(types, cx, arches);
            }
            Self::Object => {
                Self::IUnknown.combine_closure(types, cx, arches);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precise_type_exclusions() {
        let reader = default_reader(false);
        let cases: &[(&[&str], TypeName)] = &[
            (
                &["GetTickCount64", "!GetTickCount64"],
                TypeName("Windows.Win32", "GetTickCount64"),
            ),
            (
                &[
                    "!Windows.Win32.GetTickCount64",
                    "Windows.Win32.GetTickCount64",
                ],
                TypeName("Windows.Win32", "GetTickCount64"),
            ),
            (
                &[
                    "Windows.Win32::{GetTickCount,GetTickCount64}",
                    "!Windows.Win32::{GetTickCount64}",
                ],
                TypeName("Windows.Win32", "GetTickCount64"),
            ),
            (
                &["DateTime", "!Windows.Foundation.DateTime"],
                TypeName("Windows.Foundation", "DateTime"),
            ),
            (
                &["Uri", "!Windows.Foundation.Uri"],
                TypeName("Windows.Foundation", "Uri"),
            ),
            (
                &["IUriRuntimeClass", "!IUriRuntimeClass"],
                TypeName("Windows.Foundation", "IUriRuntimeClass"),
            ),
            (
                &[
                    "Windows.Foundation.IUriRuntimeClass::AbsoluteUri",
                    "!Windows.Foundation.IUriRuntimeClass",
                ],
                TypeName("Windows.Foundation", "IUriRuntimeClass"),
            ),
            (
                &[
                    "Windows.Foundation.IUriRuntimeClass::{}",
                    "!Windows.Foundation.IUriRuntimeClass",
                ],
                TypeName("Windows.Foundation", "IUriRuntimeClass"),
            ),
        ];

        for (entries, excluded) in cases {
            let parsed: Vec<_> = ["GetTickCount"]
                .into_iter()
                .chain(entries.iter().copied())
                .flat_map(filter_parser::parse_filter_entry)
                .collect();
            let resolved = filter_parser::resolve_entries(reader, &parsed);
            let filter = Selection::resolve(reader, &resolved, &References::default());
            let (types, _) =
                TypeClosure::build(reader, &filter, &References::default(), None, Style::Full);

            assert!(!types.contains_key(excluded), "{entries:?}");
            assert!(types.contains_key(&TypeName("Windows.Win32", "GetTickCount")));
        }
    }

    #[test]
    fn precise_type_selection_overrides_namespace_exclusion() {
        let reader = default_reader(false);
        for entries in [
            ["Windows.Foundation.DateTime", "!Windows.Foundation"],
            ["!Windows.Foundation", "Windows.Foundation.DateTime"],
        ] {
            let parsed: Vec<_> = entries
                .into_iter()
                .flat_map(filter_parser::parse_filter_entry)
                .collect();
            let resolved = filter_parser::resolve_entries(reader, &parsed);
            let filter = Selection::resolve(reader, &resolved, &References::default());
            let (types, _) =
                TypeClosure::build(reader, &filter, &References::default(), None, Style::Full);

            assert!(types.contains_key(&TypeName("Windows.Foundation", "DateTime")));
        }
    }
}
