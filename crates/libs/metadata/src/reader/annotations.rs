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
}

impl std::fmt::Display for NativeContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Opaque(error) => error.fmt(f),
            Self::Annotation(error) => error.fmt(f),
            Self::Bitfield(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for NativeContractError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::Opaque(error) => error,
            Self::Annotation(error) => error,
            Self::Bitfield(error) => error,
        })
    }
}

#[derive(Default)]
struct Annotations {
    evidence: BTreeSet<(String, i32)>,
    bindings: Vec<(String, i32, String, i32)>,
}

impl Index {
    /// Checks opaque storage and raw native annotation/binding invariants.
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
        }
        self.validate_native_annotations()
            .map_err(NativeContractError::Annotation)?;
        self.validate_native_bitfields(reference)
            .map_err(NativeContractError::Bitfield)
    }

    /// Checks annotation identities, owners, and original positional bindings without lowering SAL.
    pub fn validate_native_annotations(&self) -> Result<(), NativeAnnotationError> {
        let mut owners: BTreeMap<HasAttribute<'_>, Annotations> = BTreeMap::new();
        for attr in self.all_attributes().filter(|attr| {
            attr.namespace() == "Windows.Win32.Metadata"
                && matches!(
                    attr.name(),
                    "NativeAnnotationAttribute" | "NativeAnnotationBindingAttribute"
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
