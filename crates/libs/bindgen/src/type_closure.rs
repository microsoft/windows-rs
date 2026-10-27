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
        style: Style,
    ) -> TypeMap {
        let mut types = TypeMap::new();

        // Interface method seeds pull in only the requested signatures.
        for ((namespace, name), method_set) in &filter.requested_interfaces {
            for ty in reader.with_full_name(namespace, name) {
                if filter.excludes_type_name(ty.type_name()) {
                    continue;
                }
                types.insert(ty.clone());

                if let Type::Interface(iface) = &ty {
                    if filter.includes_full_hierarchy(namespace, name) {
                        for required in iface.required_interfaces(reader) {
                            Type::Interface(required)
                                .combine_closure(&mut types, reader, references, style, 7);
                        }
                    }
                    for method in iface.def.methods() {
                        if method_included_by_set(method, method_set) {
                            let sig = method.method_signature(&iface.generics, reader);
                            for dep_ty in sig.types() {
                                dep_ty.combine_closure(&mut types, reader, references, style, 7);
                            }
                        }
                    }
                } else if let Type::CppInterface(iface) = &ty {
                    for base in iface.base_interfaces(reader) {
                        base.combine_closure(&mut types, reader, references, style, 7);
                    }
                    for method in iface.def.methods() {
                        if method_included_by_set(method, method_set) {
                            let sig = method.method_signature(&[], reader);
                            for dep_ty in sig.types() {
                                dep_ty.combine_closure(&mut types, reader, references, style, 7);
                            }
                        }
                    }
                }
            }
        }

        for (namespace, name) in &filter.direct_types {
            for ty in reader.with_full_name(namespace, name) {
                if filter.excludes_type_name(ty.type_name()) {
                    continue;
                }
                ty.combine_closure(&mut types, reader, references, style, 7);
                types.insert(ty.clone());

                if let Type::Interface(interface) = &ty
                    && implements
                        .is_some_and(|implements| implements.matches(interface.type_name()))
                {
                    for method in interface.def.methods() {
                        let sig = method.method_signature(&interface.generics, reader);
                        for dep_ty in sig.types() {
                            dep_ty.combine_closure(&mut types, reader, references, style, 7);
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
                    for field in e.def.fields() {
                        if field.flags().contains(FieldAttributes::Literal)
                            && variant_set.includes(field.name())
                        {
                            for constant in reader.with_full_name(e.def.namespace(), field.name()) {
                                if matches!(constant, Type::CppConst(_)) {
                                    constant
                                        .combine_closure(&mut types, reader, references, style, 7);
                                }
                            }
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
    fn combine_closure(
        &self,
        types: &mut TypeMap,
        reader: &Reader,
        references: &References,
        style: Style,
        arches: i32,
    );

    fn combine_abi_closure(
        &self,
        types: &mut TypeMap,
        reader: &Reader,
        references: &References,
        style: Style,
        arches: i32,
    );

    fn combine_definition_closure(
        &self,
        types: &mut TypeMap,
        reader: &Reader,
        references: &References,
        style: Style,
        arches: i32,
    );
}

impl CombineClosure for Type {
    fn combine_abi_closure(
        &self,
        types: &mut TypeMap,
        reader: &Reader,
        references: &References,
        style: Style,
        arches: i32,
    ) {
        if self.is_interface_with_reader(reader) {
            // Keep interface identities for callers constructing ABI outputs, but not erased aliases.
            self.projection_type(reader)
                .combine_closure(types, reader, references, style, arches);
            return;
        }
        match self {
            Self::PtrMut(inner, _) | Self::PtrConst(inner, _) | Self::ArrayFixed(inner, _) => {
                inner.combine_abi_closure(types, reader, references, style, arches);
            }
            _ => self.combine_closure(types, reader, references, style, arches),
        }
    }

    fn combine_closure(
        &self,
        types: &mut TypeMap,
        reader: &Reader,
        references: &References,
        style: Style,
        arches: i32,
    ) {
        let ty = self.dependency_type();

        if let Self::Ptr64(inner, _) = ty {
            types.insert(Self::Pointer64Helper);
            inner.combine_closure(types, reader, references, style, arches);
            return;
        }
        if let Self::FnPtr(signature) = ty {
            for ty in signature.types() {
                ty.combine_closure(types, reader, references, style, arches);
            }
            return;
        }
        if ty.is_intrinsic() {
            return;
        }

        if let Some(owner) = ty.enclosing() {
            for owner in reader.with_full_name(owner.namespace(), owner.name()) {
                owner.combine_closure(types, reader, references, style, arches);
            }
            return;
        }

        // Referenced crates own the outer type, but generic arguments may be local.
        let tn = ty.type_name();
        if !tn.namespace().is_empty() && references.contains(tn).is_some() {
            let (_ty_inner, generics) = ty.split_generic(reader);
            for g in &generics {
                g.combine_closure(types, reader, references, style, arches);
            }
            return;
        }

        let (ty_inner, generics) = ty.split_generic(reader);
        for g in &generics {
            g.combine_closure(types, reader, references, style, arches);
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
        let candidates = std::iter::once((insert_ty, ty_inner.clone())).chain(
            siblings
                .into_iter()
                .filter(|sibling| *sibling != ty_inner)
                .map(|sibling| (sibling.clone(), sibling)),
        );
        for (insert_ty, definition) in candidates {
            let own = insert_ty.arches();
            let reached = arches & if own == 0 { 7 } else { own };
            if reached == 0 {
                continue;
            }
            // Core types have empty namespaces but are needed for standalone `--sys` output.
            let insert_tn = insert_ty.type_name();
            if (!insert_tn.namespace().is_empty() || insert_ty.is_core())
                && !types.insert_reached(insert_ty, reached)
            {
                continue;
            }
            definition.combine_definition_closure(types, reader, references, style, reached);
        }
    }

    fn combine_definition_closure(
        &self,
        types: &mut TypeMap,
        reader: &Reader,
        references: &References,
        style: Style,
        arches: i32,
    ) {
        match self {
            Self::Struct(s) => {
                for field in s.def.fields() {
                    let field_ty = field.field_type(None, reader);
                    field_ty.combine_closure(types, reader, references, style, arches);
                }
            }
            Self::CppStruct(s) => {
                for ty in s.definition_types(reader) {
                    ty.combine_closure(types, reader, references, style, arches);
                }
            }
            Self::Enum(_) | Self::CppEnum(_) => {}
            Self::Delegate(d) => {
                for method in d.def.methods() {
                    if method.name() == "Invoke" {
                        let sig = method.method_signature(&d.generics, reader);
                        for dep_ty in sig.types() {
                            dep_ty.combine_closure(types, reader, references, style, arches);
                        }
                    }
                }
            }
            Self::CppDelegate(d) => {
                for method in d.def.methods() {
                    if method.name() == "Invoke" {
                        let sig = method.method_signature(&[], reader);
                        for dep_ty in sig.types() {
                            dep_ty.combine_closure(types, reader, references, style, arches);
                        }
                    }
                }
            }
            Self::Interface(_iface) => {
                // Dependency interfaces need identity and hierarchy, not full method surfaces.
                Self::Object.combine_closure(types, reader, references, style, arches);
            }
            Self::CppInterface(iface) => {
                for base in iface.base_interfaces(reader) {
                    base.combine_closure(types, reader, references, style, arches);
                }
                Self::IUnknown.combine_closure(types, reader, references, style, arches);
            }
            Self::CppFn(f) => {
                let sig = f.method.method_signature(&[], reader);
                sig.return_type
                    .combine_closure(types, reader, references, style, arches);
                for param in &sig.params {
                    if style.is_minimal() {
                        param
                            .ty
                            .combine_abi_closure(types, reader, references, style, arches);
                    } else {
                        param
                            .ty
                            .combine_closure(types, reader, references, style, arches);
                    }
                }
                if let Some(dependency) = f.window_long_dependency(reader) {
                    reader
                        .unwrap_full_name(f.namespace, dependency)
                        .combine_closure(types, reader, references, style, arches);
                }
            }
            Self::CppConst(c) => {
                // Standalone unscoped enum constants still need their owning enum alias.
                let field_ty = c.field.field_type(None, reader);
                field_ty.combine_closure(types, reader, references, style, arches);
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
                            g.combine_closure(types, reader, references, style, arches);
                        }
                        continue;
                    }
                    iface_ty.combine_closure(types, reader, references, style, arches);
                }

                let mut def = c.def;
                loop {
                    let extends = def.extends().unwrap();
                    if extends == (TypeName::Object.0, TypeName::Object.1) {
                        break;
                    }
                    let base = reader.unwrap_type_name(extends.namespace(), extends.name());
                    base.combine_closure(types, reader, references, style, arches);
                    if let Self::Class(base_class) = &base {
                        def = base_class.def;
                    } else {
                        break;
                    }
                }
            }
            Self::IUnknown => {
                Self::GUID.combine_closure(types, reader, references, style, arches);
                Self::HRESULT.combine_closure(types, reader, references, style, arches);
            }
            Self::Object => {
                Self::IUnknown.combine_closure(types, reader, references, style, arches);
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
            let mut filter = Filter::from_resolved(reader, &resolved);
            assert!(!filter.has_broad_filter);
            filter.uses_closure = true;
            let types = TypeClosure::build(
                reader,
                &mut filter,
                &References::default(),
                None,
                Style::Default,
            );

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
            let mut filter = Filter::from_resolved(reader, &resolved);
            assert!(!filter.has_broad_filter);
            filter.uses_closure = true;
            let types = TypeClosure::build(
                reader,
                &mut filter,
                &References::default(),
                None,
                Style::Default,
            );

            assert!(types.contains_key(&TypeName("Windows.Foundation", "DateTime")));
        }
    }
}
