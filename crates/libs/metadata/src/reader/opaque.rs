use super::*;

/// An opaque record declaration or use that claims native storage.
#[derive(Debug)]
pub struct OpaqueTypeError(String);

impl std::fmt::Display for OpaqueTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for OpaqueTypeError {}

fn native_attribute<'a>(row: impl HasAttributes<'a>, name: &str) -> bool {
    row.attributes()
        .any(|attr| attr.namespace() == "Windows.Win32.Metadata" && attr.name() == name)
}

fn opaque_attribute(attr: Attribute<'_>) -> bool {
    attr.namespace() == "Windows.Win32.Metadata" && attr.name() == "NativeOpaqueAttribute"
}

fn type_name(ty: TypeDef<'_>) -> String {
    let name = ty.full_name();
    format!("{}.{}", name.namespace, name.name)
}

impl TypeDef<'_> {
    /// Returns whether this nominal record has no exposed native storage definition.
    pub fn is_opaque(&self) -> bool {
        native_attribute(*self, "NativeOpaqueAttribute")
    }
}

impl Index {
    /// Checks opaque declarations and storage-requiring uses, including typedef targets.
    ///
    /// References supply definitions absent from this index. Unresolved external types are
    /// outside this check; they are not inferred to be opaque or complete.
    pub fn validate_opaque_types(&self, reference: Option<&Self>) -> Result<(), OpaqueTypeError> {
        if let Some(reference) = reference {
            reference.validate_opaque_types(None)?;
        }
        let mut marked = false;
        for attr in self.all_attributes().filter(|attr| opaque_attribute(*attr)) {
            marked = true;
            if !matches!(attr.parent(), HasAttribute::TypeDef(_)) || !attr.value().is_empty() {
                return Err(OpaqueTypeError(
                    "NativeOpaqueAttribute requires a native record and no arguments".to_string(),
                ));
            }
        }
        if !marked && !reference.is_some_and(|index| index.all_attributes().any(opaque_attribute)) {
            return Ok(());
        }
        let mut types: Vec<_> = self.all_types().collect();
        types.sort_by_key(|ty| (type_name(*ty), ty.arches()));
        for ty in types {
            let name = type_name(ty);
            if ty.is_opaque()
                && self
                    .opaque_definitions(&ty.full_name(), None)
                    .iter()
                    .any(|other| {
                        let arches = effective_arches(ty, 0);
                        let other_arches = effective_arches(*other, 0);
                        !other.is_opaque()
                            && (arches == 0 || other_arches == 0 || arches & other_arches != 0)
                    })
            {
                return Err(OpaqueTypeError(format!(
                    "type `{name}` has conflicting opaque and complete definitions"
                )));
            }
            if ty.is_opaque()
                && (ty.category() != TypeCategory::Struct
                    || ty.flags().contains(TypeAttributes::WindowsRuntime)
                    || ty.fields().next().is_some()
                    || ty.methods().next().is_some()
                    || ty.generic_params().next().is_some()
                    || self.nested(ty).next().is_some()
                    || ty.class_layout().is_some()
                    || native_attribute(ty, "AlignmentAttribute")
                    || native_attribute(ty, "NativeTypedefAttribute"))
            {
                return Err(OpaqueTypeError(format!(
                    "opaque type `{name}` must be a native record without fields, methods, generics, nested definitions, or layout"
                )));
            }
            let alias = native_attribute(ty, "NativeTypedefAttribute");
            if alias && ty.fields().len() != 1 {
                return Err(OpaqueTypeError(format!(
                    "native typedef `{name}` must have one target field"
                )));
            }
            for field in ty.fields() {
                let context = format!("{name}.{}", field.name());
                self.check_opaque_type(
                    &field.ty(),
                    !alias,
                    effective_arches(ty, field.arches()),
                    reference,
                    &context,
                    &mut Vec::new(),
                )?;
            }
            let generics: Vec<_> = ty
                .generic_params()
                .map(|param| Type::Generic(param.name().to_string(), param.sequence()))
                .collect();
            if let Some(base) = ty.extends() {
                self.check_opaque_type(
                    &base.ty(&generics),
                    true,
                    effective_arches(ty, 0),
                    reference,
                    &format!("{name} base"),
                    &mut Vec::new(),
                )?;
            }
            for interface in ty.interface_impls() {
                self.check_opaque_type(
                    &interface.interface(&generics),
                    true,
                    effective_arches(ty, 0),
                    reference,
                    &format!("{name} base"),
                    &mut Vec::new(),
                )?;
            }
            for method in ty.methods() {
                let context = format!("{name}.{}", method.name());
                let arches = effective_arches(ty, method.arches());
                let signature = method.signature(&generics);
                self.check_opaque_type(
                    &signature.return_type,
                    true,
                    arches,
                    reference,
                    &format!("{context} return"),
                    &mut Vec::new(),
                )?;
                for (position, ty) in signature.types.iter().enumerate() {
                    self.check_opaque_type(
                        ty,
                        true,
                        arches,
                        reference,
                        &format!("{context} parameter {position}"),
                        &mut Vec::new(),
                    )?;
                }
            }
        }
        Ok(())
    }

    /// Rejects storage calculations for an opaque value, including aliases and array elements.
    pub fn validate_opaque_storage(&self, ty: &Type) -> Result<(), OpaqueTypeError> {
        self.validate_opaque_types(None)?;
        self.check_opaque_type(ty, true, 0, None, "type", &mut Vec::new())
    }

    fn check_opaque_type(
        &self,
        ty: &Type,
        storage: bool,
        arches: i32,
        reference: Option<&Self>,
        context: &str,
        visiting: &mut Vec<TypeName>,
    ) -> Result<(), OpaqueTypeError> {
        match ty {
            Type::Const(inner) | Type::Volatile(inner) => {
                self.check_opaque_type(inner, storage, arches, reference, context, visiting)
            }
            Type::PtrMut(inner, _)
            | Type::PtrConst(inner, _)
            | Type::RefMut(inner)
            | Type::RefConst(inner) => {
                self.check_opaque_type(inner, false, arches, reference, context, visiting)
            }
            Type::Array(inner) | Type::ArrayFixed(inner, _) => {
                self.check_opaque_type(inner, true, arches, reference, context, visiting)
            }
            Type::ValueName(name) | Type::ClassName(name) => {
                for def in self.opaque_definitions(name, reference) {
                    let def_arches = effective_arches(def, 0);
                    if arches != 0 && def_arches != 0 && arches & def_arches == 0 {
                        continue;
                    }
                    if storage && def.is_opaque() {
                        return Err(OpaqueTypeError(format!(
                            "{context} requires storage for opaque type `{}`",
                            type_name(def)
                        )));
                    }
                    if native_attribute(def, "NativeTypedefAttribute") {
                        if visiting.contains(name) {
                            return Err(OpaqueTypeError(format!(
                                "{context} has a cyclic native typedef `{}`",
                                type_name(def)
                            )));
                        }
                        visiting.push(name.clone());
                        let fields: Vec<_> = def.fields().collect();
                        if fields.len() != 1 {
                            return Err(OpaqueTypeError(format!(
                                "native typedef `{}` must have one target field",
                                type_name(def)
                            )));
                        }
                        self.check_opaque_type(
                            &fields[0].ty(),
                            storage,
                            arches,
                            reference,
                            context,
                            visiting,
                        )?;
                        visiting.pop();
                    }
                }
                for argument in &name.generics {
                    self.check_opaque_type(argument, true, arches, reference, context, visiting)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn opaque_definitions<'a>(
        &'a self,
        name: &TypeName,
        reference: Option<&'a Self>,
    ) -> Vec<TypeDef<'a>> {
        let mut path = name.name.split('/');
        let root = path.next().unwrap();
        let roots = |index: &'a Self| -> Vec<_> {
            if name.namespace.is_empty() {
                index
                    .all_types()
                    .filter(|def| {
                        let full = def.full_name();
                        full.namespace.is_empty() && full.name == root
                    })
                    .collect()
            } else {
                index.get(&name.namespace, root).collect()
            }
        };
        let mut types = roots(self);
        if types.is_empty()
            && let Some(reference) = reference
        {
            types.extend(roots(reference));
        }
        for part in path {
            types = types
                .into_iter()
                .flat_map(|parent| {
                    parent
                        .index()
                        .nested(parent)
                        .filter(move |child| child.name() == part)
                })
                .collect();
        }
        types
    }
}

fn effective_arches(ty: TypeDef<'_>, member: i32) -> i32 {
    if member != 0 {
        return member;
    }
    if ty.arches() != 0 {
        return ty.arches();
    }
    ty.equal_range::<NestedClass>(0, ty.pos() + 1)
        .next()
        .map_or(0, |nested| effective_arches(nested.outer(), 0))
}
