use super::*;

/// An invalid raw native annotation or original parameter binding.
#[derive(Debug)]
pub struct NativeAnnotationError(String);

impl std::fmt::Display for NativeAnnotationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for NativeAnnotationError {}

/// A failed native representation invariant checked before publishing metadata.
#[derive(Debug)]
pub enum NativeContractError {
    Opaque(OpaqueTypeError),
    Annotation(NativeAnnotationError),
    Bitfield(BitfieldError),
    DataImport(NativeDataImportError),
    Slot(NativeSlotError),
    Value(NativeValueError),
    Record(NativeRecordError),
}

impl std::fmt::Display for NativeContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opaque(error) => error.fmt(f),
            Self::Annotation(error) => error.fmt(f),
            Self::Bitfield(error) => error.fmt(f),
            Self::DataImport(error) => error.fmt(f),
            Self::Slot(error) => error.fmt(f),
            Self::Value(error) => error.fmt(f),
            Self::Record(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for NativeContractError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::Opaque(error) => error,
            Self::Annotation(error) => error,
            Self::Bitfield(error) => error,
            Self::DataImport(error) => error,
            Self::Slot(error) => error,
            Self::Value(error) => error,
            Self::Record(error) => error,
        })
    }
}

#[derive(Default)]
struct Annotations {
    evidence: BTreeSet<(String, i32)>,
    bindings: Vec<(String, i32, String, i32)>,
}

impl Index {
    /// Checks native storage, annotations, bit-fields, imported data, slots, and values.
    ///
    /// This does not calculate layout, interpret annotation expressions, or prove source closure.
    pub fn validate_native_contracts(
        &self,
        reference: Option<&Self>,
    ) -> Result<(), NativeContractError> {
        self.validate_opaque_types(reference)
            .map_err(NativeContractError::Opaque)?;
        if let Some(reference) = reference {
            reference
                .validate_native_annotations()
                .map_err(NativeContractError::Annotation)?;
            reference
                .validate_native_bitfields(None)
                .map_err(NativeContractError::Bitfield)?;
            reference
                .validate_native_data_imports(None)
                .map_err(NativeContractError::DataImport)?;
            reference
                .validate_native_slots(None)
                .map_err(NativeContractError::Slot)?;
            reference
                .validate_native_values(None)
                .map_err(NativeContractError::Value)?;
            reference
                .validate_native_records(None)
                .map_err(NativeContractError::Record)?;
        }
        self.validate_native_annotations_with_reference(reference)
            .map_err(NativeContractError::Annotation)?;
        self.validate_native_bitfields(reference)
            .map_err(NativeContractError::Bitfield)?;
        self.validate_native_data_imports(reference)
            .map_err(NativeContractError::DataImport)?;
        self.validate_native_slots(reference)
            .map_err(NativeContractError::Slot)?;
        self.validate_native_values(reference)
            .map_err(NativeContractError::Value)?;
        self.validate_native_records(reference)
            .map_err(NativeContractError::Record)
    }

    /// Checks annotation identities, owners, and original positional bindings without lowering SAL.
    pub fn validate_native_annotations(&self) -> Result<(), NativeAnnotationError> {
        self.validate_native_annotations_with_reference(None)
    }

    fn validate_native_annotations_with_reference(
        &self,
        reference: Option<&Self>,
    ) -> Result<(), NativeAnnotationError> {
        let mut owners: BTreeMap<HasAttribute<'_>, Annotations> = BTreeMap::new();
        let mut aliases = BTreeSet::new();
        let mut extents = BTreeSet::new();
        for attr in self.all_attributes().filter(|attr| {
            attr.namespace() == "Windows.Win32.Metadata"
                && matches!(
                    attr.name(),
                    "NativeAnnotationAttribute"
                        | "NativeAnnotationBindingAttribute"
                        | "NativeMacroAliasAttribute"
                        | "NativeBufferExtentAttribute"
                )
        }) {
            let owner = attr.parent();
            let declaration = match owner {
                HasAttribute::TypeDef(ty) => ty,
                HasAttribute::Field(field) => field.parent_row(4),
                HasAttribute::MethodDef(method) => method_type(method)?,
                HasAttribute::MethodParam(param) => method_type(param.parent())?,
                _ => return invalid("native annotation requires a declaration owner"),
            };
            if declaration.flags().contains(TypeAttributes::WindowsRuntime) {
                return invalid("native annotation is not valid on a WinRT declaration");
            }
            let values = attr.value();
            if values.iter().any(|(name, _)| !name.is_empty()) {
                return invalid("native annotation arguments must be positional");
            }
            let values: Vec<_> = values.into_iter().map(|(_, value)| value).collect();
            if attr.name() == "NativeMacroAliasAttribute" {
                let [Value::Utf8(name), Value::Utf8(kind), Value::Utf8(profile)] =
                    values.as_slice()
                else {
                    return invalid("invalid native macro alias arguments");
                };
                let valid_owner = match owner {
                    HasAttribute::MethodDef(method) => {
                        kind == "FunctionDecl" && method.impl_map().is_some()
                    }
                    HasAttribute::TypeDef(_) => matches!(
                        kind.as_str(),
                        "TypedefDecl"
                            | "TypeAliasDecl"
                            | "StructDecl"
                            | "ClassDecl"
                            | "UnionDecl"
                            | "EnumDecl"
                    ),
                    _ => false,
                };
                let mut bytes = name.bytes();
                if !valid_owner
                    || !matches!(bytes.next(), Some(b'a'..=b'z' | b'A'..=b'Z' | b'_'))
                    || !bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
                    || !native_alias_profile(profile)
                {
                    return invalid("invalid native macro alias owner, name, kind, or profile");
                }
                if !aliases.insert((name.clone(), profile.clone())) {
                    return invalid("duplicate active native macro alias in one compiler profile");
                }
                continue;
            }
            if attr.name() == "NativeBufferExtentAttribute" {
                let HasAttribute::MethodParam(parameter) = owner else {
                    return invalid("native buffer extent requires a parameter owner");
                };
                let [
                    Value::Utf8(phase),
                    Value::Utf8(units),
                    Value::I32(position),
                    Value::Bool(dereference),
                ] = values.as_slice()
                else {
                    return invalid("invalid native buffer extent arguments");
                };
                let method = parameter.parent();
                let generics: Vec<_> = declaration
                    .generic_params()
                    .map(|param| Type::Generic(param.name().to_string(), param.sequence()))
                    .collect();
                let signature = method.signature(&generics);
                method
                    .params_by_sequence(signature.types.len())
                    .map_err(|error| NativeAnnotationError(error.to_string()))?;
                if parameter.sequence() == 0
                    || !matches!(phase.as_str(), "capacity" | "valid")
                    || !matches!(units.as_str(), "bytes" | "elements")
                    || *position < 0
                    || *position as usize >= signature.types.len()
                    || !extents.insert((owner, phase.clone(), units.clone()))
                {
                    return invalid("invalid or duplicate native buffer extent");
                }
                let owned = &signature.types[usize::from(parameter.sequence() - 1)];
                if !matches!(owned.unqualified(), Type::PtrMut(..) | Type::PtrConst(..)) {
                    return invalid("native buffer extent owner is not a pointer parameter");
                }
                let value = &signature.types[*position as usize];
                let value = if *dereference {
                    match value.unqualified() {
                        Type::PtrMut(value, 1) | Type::PtrConst(value, 1) => value.as_ref(),
                        _ => {
                            return invalid(
                                "dereferenced native extent is not a one-level pointer",
                            );
                        }
                    }
                } else {
                    value
                };
                if !self.native_extent_integer(
                    value,
                    reference,
                    method.arches(),
                    &mut BTreeSet::new(),
                ) {
                    return invalid("native buffer extent is not an integer value");
                }
                continue;
            }
            let group = owners.entry(owner).or_default();
            match (attr.name(), values.as_slice()) {
                (
                    "NativeAnnotationAttribute",
                    [
                        Value::Utf8(source),
                        Value::I32(ordinal),
                        Value::Utf8(payload),
                    ],
                ) if !source.is_empty() && *ordinal >= 0 && !payload.is_empty() => {
                    if !group.evidence.insert((source.clone(), *ordinal)) {
                        return invalid("duplicate native annotation source/ordinal on one owner");
                    }
                }
                (
                    "NativeAnnotationBindingAttribute",
                    [
                        Value::Utf8(source),
                        Value::I32(ordinal),
                        Value::Utf8(identifier),
                        Value::I32(position),
                    ],
                ) if !source.is_empty() && *ordinal >= 0 && !identifier.is_empty() => {
                    group
                        .bindings
                        .push((source.clone(), *ordinal, identifier.clone(), *position));
                }
                _ => return invalid("invalid native annotation arguments"),
            }
        }
        for (owner, group) in owners {
            let mut bound = BTreeSet::new();
            for (source, ordinal, identifier, position) in group.bindings {
                if !group.evidence.contains(&(source.clone(), ordinal)) {
                    return invalid(
                        "native annotation binding has no matching evidence on its owner",
                    );
                }
                if !bound.insert((source, ordinal, identifier)) {
                    return invalid("duplicate native annotation identifier binding");
                }
                let method = match owner {
                    HasAttribute::MethodDef(method) => method,
                    HasAttribute::MethodParam(param) => param.parent(),
                    _ => return invalid("native positional binding requires a callable owner"),
                };
                let ty = method_type(method)?;
                let generics: Vec<_> = ty
                    .generic_params()
                    .map(|param| Type::Generic(param.name().to_string(), param.sequence()))
                    .collect();
                let count = method.signature(&generics).types.len();
                method
                    .params_by_sequence(count)
                    .map_err(|error| NativeAnnotationError(error.to_string()))?;
                if position < 0 || position as usize >= count {
                    return invalid("native annotation parameter position is out of range");
                }
            }
        }
        Ok(())
    }

    fn native_extent_integer(
        &self,
        ty: &Type,
        reference: Option<&Self>,
        arches: i32,
        visiting: &mut BTreeSet<(String, String)>,
    ) -> bool {
        match ty.unqualified() {
            Type::I8
            | Type::U8
            | Type::I16
            | Type::U16
            | Type::I32
            | Type::U32
            | Type::I64
            | Type::U64
            | Type::ISize
            | Type::USize => true,
            Type::ValueName(name)
                if name.generics.is_empty()
                    && visiting.len() < 64
                    && visiting.insert((name.namespace.clone(), name.name.clone())) =>
            {
                let definitions: Vec<_> = self
                    .get(&name.namespace, &name.name)
                    .chain(
                        reference
                            .into_iter()
                            .flat_map(|index| index.get(&name.namespace, &name.name)),
                    )
                    .filter(|definition| {
                        arches == 0 || definition.arches() == 0 || definition.arches() & arches != 0
                    })
                    .collect();
                if definitions.is_empty() {
                    return false;
                }
                definitions.into_iter().all(|definition| {
                    if !definition.attributes().any(|attribute| {
                        attribute.namespace() == "Windows.Win32.Metadata"
                            && attribute.name() == "NativeTypedefAttribute"
                    }) {
                        return false;
                    }
                    definition.underlying_type().is_some_and(|ty| {
                        self.native_extent_integer(&ty, reference, arches, &mut visiting.clone())
                    })
                })
            }
            _ => false,
        }
    }
}

fn native_alias_profile(profile: &str) -> bool {
    let mut lines = profile.lines();
    let Some(hash) = lines
        .next()
        .and_then(|line| line.strip_prefix("source-sha256="))
    else {
        return false;
    };
    hash.len() == 64
        && hash.bytes().all(|byte| byte.is_ascii_hexdigit())
        && lines.clone().next().is_some()
        && lines.all(|line| !line.is_empty())
}

fn method_type(method: MethodDef<'_>) -> Result<TypeDef<'_>, NativeAnnotationError> {
    match method.parent() {
        MemberRefParent::TypeDef(ty) => Ok(ty),
        _ => invalid("native annotation method has no declaring type"),
    }
}

fn invalid<T>(message: &str) -> Result<T, NativeAnnotationError> {
    Err(NativeAnnotationError(message.to_string()))
}
