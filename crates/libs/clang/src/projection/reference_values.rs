use super::*;
use windows_metadata::reader::{HasAttributes, Index, TypeCategory, TypeDef};
use windows_metadata::{AsRow, FieldAttributes, MethodCallAttributes, TypeAttributes, Value};

impl Projector<'_, '_> {
    pub(super) fn validate_value_references(
        &self,
        rejected: &mut Option<&mut partial::Issues>,
    ) -> Result<(), Error> {
        for (&family, &metadata) in &self.external.records {
            if !matches!(
                metadata.category(),
                TypeCategory::Struct | TypeCategory::Enum
            ) {
                continue;
            }
            for &group in &self.plan.families[family] {
                let group = &self.resolution.groups[group];
                let result = if let Some(reason) = self.external.invalid_records.get(&family) {
                    Err(Error(reason.clone()))
                } else {
                    self.validate_reference_record(group, metadata)
                };
                if let Err(error) = result {
                    reject(
                        rejected,
                        group.records.iter().map(|record| {
                            partial::Key::of(
                                self.resolution.snapshots[record.snapshot],
                                DeclarationId::Record(record.record),
                            )
                        }),
                        error,
                    )?;
                }
            }
        }
        for (&id, &metadata) in &self.external.enums {
            let group = &self.resolution.enums[id];
            if let Err(error) = self.validate_reference_enum(id, metadata) {
                reject(
                    rejected,
                    group.enums.iter().map(|reference| {
                        partial::Key::of(
                            self.resolution.snapshots[reference.snapshot],
                            DeclarationId::Enum(reference.enumeration),
                        )
                    }),
                    error,
                )?;
            }
        }
        for alias in &self.resolution.aliases {
            let Some(&metadata) = self.external.aliases.get(&alias.name) else {
                continue;
            };
            let result = (|| {
                for reference in &alias.observations {
                    let snapshot = self.resolution.snapshots[reference.snapshot];
                    let observation =
                        &snapshot.aliases[reference.alias.0].observations[reference.observation];
                    if metadata.has_attribute("NativeTypedefAttribute") {
                        let expected = metadata.underlying_type().ok_or_else(|| {
                            Error(format!(
                                "external alias `{}` has no underlying type",
                                alias.name
                            ))
                        })?;
                        self.validate_alias_type(
                            reference.snapshot,
                            observation.canonical,
                            expected,
                            metadata.index(),
                        )?;
                        self.alias(
                            reference.snapshot,
                            observation,
                            &alias.name,
                            &observation.attributes,
                        )?;
                    } else if metadata.category() == TypeCategory::Delegate
                        && !metadata.flags().contains(TypeAttributes::WindowsRuntime)
                    {
                        self.validate_callback_type(reference.snapshot, observation, metadata)?;
                    } else if metadata.category() == TypeCategory::Struct
                        && let TypeKind::Record(record) =
                            snapshot.types[observation.canonical.0].kind
                    {
                        self.validate_reference_record(
                            self.resolution.group(RecordRef {
                                snapshot: reference.snapshot,
                                record,
                            }),
                            metadata,
                        )?;
                    } else if !metadata.flags().contains(TypeAttributes::WindowsRuntime) {
                        let native = checked_type(snapshot, observation.canonical)?;
                        // A typedef names the interface; it does not store an interface value.
                        let actual = if let TypeKind::Record(record) = native.kind
                            && self.interface_record(reference.snapshot, record)
                            && native.qualifiers == Qualifiers::default()
                            && observation.attributes.is_empty()
                        {
                            self.interface_type(reference.snapshot, record)
                        } else {
                            self.ty(reference.snapshot, observation.canonical)?
                        };
                        if actual != references::metadata_type(metadata) {
                            return Err(Error(format!(
                                "external alias `{}` nominal type differs from metadata",
                                alias.name,
                            )));
                        }
                    }
                }
                Ok(())
            })();
            if let Err(error) = result {
                reject(rejected, [partial::Key::Alias(alias.name.clone())], error)?;
            }
        }
        Ok(())
    }

    fn validate_callback_type(
        &self,
        snapshot_id: usize,
        observation: &AliasObservation,
        metadata: TypeDef<'_>,
    ) -> Result<(), Error> {
        let fail = |reason| Error(format!("external callback `{}` {reason}", metadata.name()));
        let snapshot = self.resolution.snapshots[snapshot_id];
        let function = callback(snapshot, observation.canonical)
            .ok_or_else(|| fail("has no native pointer prototype"))?;
        let written = written_function(snapshot, observation.underlying)?;
        let expected = callback_signature(metadata)?;
        let convention = match expected.flags {
            MethodCallAttributes::STDCALL => CallingConvention::Stdcall,
            MethodCallAttributes::C => CallingConvention::C,
            _ => unreachable!(),
        };
        let mut abi = callable_abi(written);
        abi.canonicalize(snapshot.target, convention);
        if abi.convention != convention {
            return Err(fail("calling convention differs from metadata"));
        }
        let actual = self.signature(snapshot_id, function, None)?;
        if actual.types.len() != expected.types.len() {
            return Err(fail("parameter count differs from metadata"));
        }
        for (actual, expected) in std::iter::once((&actual.return_type, &expected.return_type))
            .chain(actual.types.iter().zip(&expected.types))
        {
            let actual = normalize_type(
                actual.clone(),
                metadata.index(),
                snapshot.target,
                &mut HashSet::new(),
            )?;
            let expected = normalize_type(
                expected.clone(),
                metadata.index(),
                snapshot.target,
                &mut HashSet::new(),
            )?;
            if !reference_type_matches(&actual, &expected) {
                return Err(fail("signature types differ from metadata"));
            }
        }
        Ok(())
    }

    fn validate_alias_type(
        &self,
        snapshot_id: usize,
        native: TypeId,
        expected: MetadataType,
        metadata: &Index,
    ) -> Result<(), Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let actual = normalize_type(
            self.ty(snapshot_id, native)?,
            metadata,
            snapshot.target,
            &mut HashSet::new(),
        )?;
        let expected = normalize_type(expected, metadata, snapshot.target, &mut HashSet::new())?;
        self.validate_alias_shape(snapshot_id, native, actual, expected, metadata)
    }

    fn validate_alias_shape(
        &self,
        snapshot_id: usize,
        mut native: TypeId,
        mut actual: MetadataType,
        mut expected: MetadataType,
        metadata: &Index,
    ) -> Result<(), Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        if reference_type_matches(&actual, &expected) {
            return Ok(());
        }
        loop {
            match (actual, expected) {
                (MetadataType::PtrMut(a, ac), MetadataType::PtrMut(b, bc))
                | (MetadataType::PtrConst(a, ac), MetadataType::PtrConst(b, bc))
                    if ac == bc =>
                {
                    for _ in 0..ac {
                        let TypeKind::Pointer(inner) = snapshot.types[native.0].kind else {
                            return Err(Error(
                                "external alias pointer shape differs from metadata".into(),
                            ));
                        };
                        native = inner;
                    }
                    actual = *a;
                    expected = *b;
                }
                (MetadataType::ValueName(_), MetadataType::ValueName(name))
                    if matches!(snapshot.types[native.0].kind, TypeKind::Record(_)) =>
                {
                    let TypeKind::Record(record) = snapshot.types[native.0].kind else {
                        unreachable!()
                    };
                    let bit = match snapshot.target {
                        Target::X86 => 1,
                        Target::X64 => 2,
                        Target::Arm64 => 4,
                    };
                    let mut definitions =
                        metadata
                            .get(&name.namespace, &name.name)
                            .filter(|definition| {
                                definition.arches() == 0 || definition.arches() & bit != 0
                            });
                    let definition = definitions
                        .next()
                        .ok_or_else(|| Error("missing external alias record".into()))?;
                    if definitions.next().is_some() {
                        return Err(Error("ambiguous external alias record".into()));
                    }
                    return self.validate_reference_record(
                        self.resolution.group(RecordRef {
                            snapshot: snapshot_id,
                            record,
                        }),
                        definition,
                    );
                }
                (actual, expected) => {
                    return Err(Error(format!(
                        "external alias underlying type differs from metadata: {actual:?} vs {expected:?}"
                    )));
                }
            }
        }
    }

    fn validate_reference_enum(&self, id: usize, metadata: TypeDef<'_>) -> Result<(), Error> {
        let name = &self.plan.enum_names[id];
        let fail = |reason| Error(format!("external enum `{name}` {reason}"));
        if metadata.category() != TypeCategory::Enum {
            return Err(fail("does not reference an enum"));
        }
        let group = &self.resolution.enums[id];
        let Some(reference) = group.definition else {
            return Err(fail("has no complete native definition for validation"));
        };
        let snapshot = self.resolution.snapshots[reference.snapshot];
        let observation =
            &snapshot.enums[reference.enumeration.0].observations[reference.observation];
        let expected = metadata
            .underlying_type()
            .ok_or_else(|| fail("has no metadata backing type"))?;
        if self.ty(reference.snapshot, observation.canonical)? != expected
            || observation.scoped != metadata.has_attribute("ScopedEnumAttribute")
            || flags(observation) != metadata.has_attribute("FlagsAttribute")
        {
            return Err(fail("backing type, scope, or flags differ from metadata"));
        }
        let members: BTreeMap<_, _> = metadata
            .fields()
            .filter_map(|field| {
                field
                    .constant()
                    .map(|constant| (field.name(), constant.value()))
            })
            .collect();
        for member in observation.members.as_ref().unwrap() {
            let Some(expected) = members.get(member.name.as_str()) else {
                if self.external.extended_enums.contains(&id) {
                    continue;
                }
                return Err(fail("adds members outside its metadata owner's namespace"));
            };
            let expected = native_enum_value(expected)
                .ok_or_else(|| fail("has an unsupported metadata member value"))?;
            if member.value != expected {
                return Err(Error(format!(
                    "external enum `{name}` member `{}` differs from metadata",
                    member.name,
                )));
            }
        }
        Ok(())
    }

    fn validate_reference_record(&self, group: &Group, metadata: TypeDef<'_>) -> Result<(), Error> {
        let native = group.records[0];
        let name = &self.plan.names[self.plan.family(native.snapshot, native.record)];
        let fail = |reason| Error(format!("external record `{name}` {reason}"));
        if metadata.category() != TypeCategory::Struct {
            return Err(fail("does not reference a record"));
        }
        let fields: Vec<_> = metadata
            .fields()
            .filter(|field| !field.flags().contains(FieldAttributes::Literal))
            .collect();
        if self.opaque_record(native.snapshot, native.record) {
            if !fields.is_empty()
                || metadata.class_layout().is_some()
                || metadata.flags().contains(TypeAttributes::ExplicitLayout)
                || metadata.has_attribute("NativeTypedefAttribute")
                || metadata_alignment(metadata)? != 0
            {
                return Err(fail(
                    "opaque native record cannot validate metadata storage",
                ));
            }
            let reference = group.definition.unwrap_or_else(|| {
                let snapshot = self.resolution.snapshots[native.snapshot];
                ObservationRef {
                    snapshot: native.snapshot,
                    observation: snapshot.records[native.record.0].observations[0],
                }
            });
            let observation = &self.resolution.snapshots[reference.snapshot].observations
                [reference.observation.0];
            self.record(
                reference.snapshot,
                observation,
                &ident(public_name(name))?,
                false,
            )?;
            if let Some(definition) = &observation.definition {
                self.validate_reference_record_constants(reference.snapshot, definition, metadata)?;
            }
            return Ok(());
        }
        let reference = group
            .definition
            .ok_or_else(|| fail("has no complete native definition for validation"))?;
        let snapshot = self.resolution.snapshots[reference.snapshot];
        let observation = &snapshot.observations[reference.observation.0];
        let definition = observation.definition.as_ref().unwrap();
        let native_union = snapshot.records[observation.record.0].kind == RecordKind::Union;
        if metadata.has_attribute("NativeTypedefAttribute")
            || native_union != metadata.flags().contains(TypeAttributes::ExplicitLayout)
            || !definition.bases.is_empty()
            || !definition.methods.is_empty()
            || definition
                .fields
                .iter()
                .any(|field| field.bit_width.is_some())
        {
            return Err(fail(
                "has an unsupported or incompatible referenced record shape",
            ));
        }
        self.validate_reference_record_constants(reference.snapshot, definition, metadata)?;
        if fields.len() != definition.fields.len() {
            return Err(fail("field count differs from metadata"));
        }
        let (packed, maximum) = resolve::packing(&observation.attributes);
        let native_packing = if packed { 1 } else { maximum.unwrap_or(0) / 8 };
        let metadata_packing = metadata
            .class_layout()
            .map_or(0, |layout| layout.packing_size());
        if native_packing != u32::from(metadata_packing) {
            return Err(fail("packing differs from metadata"));
        }
        let native_alignment = resolve::alignments(&observation.attributes)
            .into_iter()
            .map(|(bits, _)| u64::from(bits / 8))
            .max()
            .unwrap_or(0);
        let forced_alignment = metadata_alignment(metadata)?;
        if native_alignment != forced_alignment {
            return Err(fail("alignment request differs from metadata"));
        }
        let mut size = 0u64;
        let mut alignment = 1;
        let mut anonymous = 0;
        for (native, metadata_field) in definition.fields.iter().zip(fields) {
            checked_type(snapshot, native.canonical)?;
            let native_name = if native.name.is_empty() {
                anonymous += 1;
                if anonymous == 1 {
                    "Anonymous".into()
                } else {
                    format!("Anonymous{anonymous}")
                }
            } else {
                native.name.clone()
            };
            if native_name != metadata_field.name() {
                return Err(fail("field names or order differ from metadata"));
            }
            let expected = normalize_type(
                metadata_field.ty(),
                metadata.index(),
                snapshot.target,
                &mut HashSet::new(),
            )?;
            let actual = normalize_type(
                self.ty(reference.snapshot, native.canonical)?,
                metadata.index(),
                snapshot.target,
                &mut HashSet::new(),
            )?;
            if !reference_type_matches(&actual, &expected) {
                return Err(Error(format!(
                    "external record `{name}` field `{native_name}` type differs: {actual:?} vs {expected:?}"
                )));
            }
            // The reference owns field contracts; explicit alignment still affects storage.
            let native_alignment =
                u64::from(annotations::field_alignment(&native.attributes)?.unwrap_or(0));
            let expected_alignment = metadata_alignment(metadata_field)?;
            if native_alignment != expected_alignment {
                return Err(Error(format!(
                    "external record `{name}` field `{native_name}` alignment differs from metadata"
                )));
            }
            let layout = snapshot.types[native.canonical.0]
                .layout
                .ok_or_else(|| fail("field has no native layout"))?;
            let field_alignment = if metadata_packing == 0 {
                layout.alignment
            } else {
                layout.alignment.min(u64::from(metadata_packing))
            }
            .max(native_alignment)
            .max(self.required_alignment(
                reference.snapshot,
                native.canonical,
                &mut HashSet::new(),
            ));
            let offset = if native_union {
                0
            } else {
                size.div_ceil(field_alignment) * field_alignment
            };
            if native.offset_bits != offset * 8 {
                return Err(fail("field offsets differ from metadata layout"));
            }
            size = size.max(offset + layout.size);
            alignment = alignment.max(field_alignment);
        }
        alignment = alignment.max(forced_alignment);
        size = size.div_ceil(alignment) * alignment;
        if definition.layout != (Layout { size, alignment })
            || metadata.class_layout().is_some_and(|layout| {
                layout.class_size() != 0 && u64::from(layout.class_size()) != size
            })
        {
            return Err(fail("size or alignment differs from metadata layout"));
        }
        Ok(())
    }

    fn validate_reference_record_constants(
        &self,
        snapshot: usize,
        definition: &RecordDefinition,
        metadata: TypeDef<'_>,
    ) -> Result<(), Error> {
        for id in &definition.enums {
            let enumeration = &self.resolution.snapshots[snapshot].enums[id.0];
            let group = self.resolution.enum_groups[snapshot][id.0];
            if enumeration.access != FieldAccess::Public
                || !self.plan.enum_constants(self.resolution, group)
            {
                continue;
            }
            let reference = self.resolution.enums[group]
                .definition
                .ok_or_else(|| Error("incomplete referenced member enum".into()))?;
            let observation = &self.resolution.snapshots[reference.snapshot].enums
                [reference.enumeration.0]
                .observations[reference.observation];
            let backing = self.ty(reference.snapshot, observation.canonical)?;
            for member in observation.members.as_ref().unwrap() {
                let field = metadata.fields().find(|field| field.name() == member.name);
                if !field.is_some_and(|field| {
                    field
                        .flags()
                        .contains(FieldAttributes::Static | FieldAttributes::Literal)
                        && field.ty() == backing
                        && field
                            .constant()
                            .and_then(|constant| native_enum_value(&constant.value()))
                            == Some(member.value)
                }) {
                    return Err(Error(format!(
                        "external record `{}` constant `{}` differs from metadata",
                        metadata.name(),
                        member.name
                    )));
                }
            }
        }
        Ok(())
    }
}

fn native_enum_value(value: &Value) -> Option<EnumValue> {
    Some(match value {
        Value::I8(value) => EnumValue::Signed((*value).into()),
        Value::I16(value) => EnumValue::Signed((*value).into()),
        Value::I32(value) => EnumValue::Signed((*value).into()),
        Value::I64(value) => EnumValue::Signed(*value),
        Value::U8(value) => EnumValue::Unsigned((*value).into()),
        Value::U16(value) => EnumValue::Unsigned((*value).into()),
        Value::U32(value) => EnumValue::Unsigned((*value).into()),
        Value::U64(value) => EnumValue::Unsigned(*value),
        _ => return None,
    })
}

// Reference-owned opaque pointers may describe more specific native data pointers.
// This is directional publication compatibility, not native type equality.
fn reference_type_matches(actual: &MetadataType, expected: &MetadataType) -> bool {
    if actual == expected {
        return true;
    }
    match (actual, expected) {
        (MetadataType::PtrMut(actual, a), MetadataType::PtrMut(expected, b))
        | (MetadataType::PtrConst(actual, a), MetadataType::PtrConst(expected, b))
            if a == b =>
        {
            reference_pointee_matches(actual, expected)
        }
        (MetadataType::Ptr64(actual, a), MetadataType::Ptr64(expected, b)) if a == b => {
            reference_pointee_matches(actual, expected)
        }
        (MetadataType::ArrayFixed(actual, a), MetadataType::ArrayFixed(expected, b)) => {
            a == b && reference_type_matches(actual, expected)
        }
        (MetadataType::FnPtr(actual), MetadataType::FnPtr(expected)) => {
            actual.flags == expected.flags
                && actual.types.len() == expected.types.len()
                && reference_type_matches(&actual.return_type, &expected.return_type)
                && actual
                    .types
                    .iter()
                    .zip(&expected.types)
                    .all(|(actual, expected)| reference_type_matches(actual, expected))
        }
        _ => false,
    }
}

fn reference_pointee_matches(actual: &MetadataType, expected: &MetadataType) -> bool {
    if *expected == MetadataType::Void {
        matches!(
            actual,
            MetadataType::Void
                | MetadataType::ValueName(_)
                | MetadataType::Bool
                | MetadataType::Char
                | MetadataType::I8
                | MetadataType::U8
                | MetadataType::I16
                | MetadataType::U16
                | MetadataType::I32
                | MetadataType::U32
                | MetadataType::I64
                | MetadataType::U64
                | MetadataType::F32
                | MetadataType::F64
                | MetadataType::ArrayFixed(..)
        )
    } else {
        reference_type_matches(actual, expected)
    }
}

pub(super) fn callback_signature(
    metadata: TypeDef<'_>,
) -> Result<windows_metadata::Signature, Error> {
    let fail = |reason| Error(format!("external callback `{}` {reason}", metadata.name()));
    let convention = match metadata.find_attribute("UnmanagedFunctionPointerAttribute") {
        None => MethodCallAttributes::STDCALL,
        Some(attribute) => match attribute.value().as_slice() {
            [(_, Value::EnumValue(_, value))] => match value.as_ref() {
                Value::I32(1) => MethodCallAttributes::STDCALL,
                Value::I32(2) => MethodCallAttributes::C,
                _ => return Err(fail("has an unsupported metadata calling convention")),
            },
            _ => return Err(fail("has malformed calling convention metadata")),
        },
    };
    let method = metadata
        .methods()
        .find(|method| method.name() == "Invoke")
        .ok_or_else(|| fail("has no Invoke method"))?;
    let mut signature = method.signature(&[]);
    if signature.flags.0 & 0x0f != 0 || metadata.generic_params().next().is_some() {
        return Err(fail("has an unsupported metadata prototype"));
    }
    signature.flags = convention;
    Ok(signature)
}

fn metadata_alignment<'a>(owner: impl HasAttributes<'a>) -> Result<u64, Error> {
    owner
        .find_attribute("AlignmentAttribute")
        .map(|attribute| match attribute.value().as_slice() {
            [(_, Value::I32(value))] if *value > 0 => Ok(*value as u64),
            [(_, Value::U32(value))] if *value > 0 => Ok(u64::from(*value)),
            _ => Err(Error("unsupported metadata alignment".into())),
        })
        .transpose()
        .map(|value| value.unwrap_or(0))
}

pub(super) fn declaration_key(tokens: TokenStream) -> (BTreeSet<String>, String) {
    let mut tokens = tokens.into_iter().peekable();
    let mut attributes = BTreeSet::new();
    while matches!(tokens.peek(), Some(proc_macro2::TokenTree::Punct(value)) if value.as_char() == '#')
    {
        tokens.next();
        let Some(proc_macro2::TokenTree::Group(group)) = tokens.next() else {
            unreachable!();
        };
        let value = group.stream().to_string();
        if !value.starts_with("arch ") {
            attributes.insert(value);
        }
    }
    (attributes, tokens.collect::<TokenStream>().to_string())
}

pub(super) fn normalize_type(
    ty: MetadataType,
    metadata: &Index,
    target: Target,
    seen: &mut HashSet<(String, String)>,
) -> Result<MetadataType, Error> {
    expand_type(ty, metadata, target, seen, true)
}

pub(super) fn expand_type(
    ty: MetadataType,
    metadata: &Index,
    target: Target,
    seen: &mut HashSet<(String, String)>,
    normalize_abi: bool,
) -> Result<MetadataType, Error> {
    Ok(match ty {
        MetadataType::ValueName(ref name) | MetadataType::ClassName(ref name) => {
            let bit = match target {
                Target::X86 => 1,
                Target::X64 => 2,
                Target::Arm64 => 4,
            };
            let mut candidates = metadata
                .get(&name.namespace, &name.name)
                .filter(|definition| definition.arches() == 0 || definition.arches() & bit != 0);
            let definition = candidates.next();
            if candidates.next().is_some() {
                return Err(Error(format!("ambiguous referenced type `{name:?}`")));
            }
            if let Some(definition) = definition
                && (definition.has_attribute("NativeTypedefAttribute")
                    || definition.category() == TypeCategory::Delegate
                        && !definition.flags().contains(TypeAttributes::WindowsRuntime))
            {
                let key = (name.namespace.clone(), name.name.clone());
                if !seen.insert(key.clone()) {
                    return Err(Error(format!("cyclic referenced alias `{name:?}`")));
                }
                let underlying = if definition.has_attribute("NativeTypedefAttribute") {
                    definition
                        .underlying_type()
                        .ok_or_else(|| Error(format!("referenced alias `{name:?}` has no type")))?
                } else {
                    MetadataType::FnPtr(Box::new(callback_signature(definition)?))
                };
                let result = expand_type(underlying, metadata, target, seen, normalize_abi)?;
                seen.remove(&key);
                result
            } else {
                ty
            }
        }
        MetadataType::FnPtr(signature) => {
            let mut signature = *signature;
            // Windows x64 and ARM64 have one ABI for C and stdcall prototypes.
            if normalize_abi
                && target != Target::X86
                && signature.flags == MethodCallAttributes::STDCALL
            {
                signature.flags = MethodCallAttributes::C;
            }
            signature.return_type =
                expand_type(signature.return_type, metadata, target, seen, normalize_abi)?;
            signature.types = signature
                .types
                .into_iter()
                .map(|ty| expand_type(ty, metadata, target, seen, normalize_abi))
                .collect::<Result<_, _>>()?;
            MetadataType::FnPtr(Box::new(signature))
        }
        MetadataType::ISize => {
            if target == Target::X86 {
                MetadataType::I32
            } else {
                MetadataType::I64
            }
        }
        MetadataType::USize => {
            if target == Target::X86 {
                MetadataType::U32
            } else {
                MetadataType::U64
            }
        }
        MetadataType::PtrMut(inner, count) => {
            match expand_type(*inner, metadata, target, seen, normalize_abi)? {
                MetadataType::PtrMut(inner, nested) => MetadataType::PtrMut(inner, count + nested),
                inner => MetadataType::PtrMut(Box::new(inner), count),
            }
        }
        MetadataType::PtrConst(inner, count) => {
            match expand_type(*inner, metadata, target, seen, normalize_abi)? {
                MetadataType::PtrConst(inner, nested) => {
                    MetadataType::PtrConst(inner, count + nested)
                }
                inner => MetadataType::PtrConst(Box::new(inner), count),
            }
        }
        MetadataType::Ptr64(inner, constant) if target != Target::X86 => expand_type(
            if constant {
                MetadataType::PtrConst(inner, 1)
            } else {
                MetadataType::PtrMut(inner, 1)
            },
            metadata,
            target,
            seen,
            normalize_abi,
        )?,
        MetadataType::Ptr64(inner, constant) => MetadataType::Ptr64(
            Box::new(expand_type(*inner, metadata, target, seen, normalize_abi)?),
            constant,
        ),
        MetadataType::ArrayFixed(inner, count) => MetadataType::ArrayFixed(
            Box::new(expand_type(*inner, metadata, target, seen, normalize_abi)?),
            count,
        ),
        MetadataType::RefMut(inner) => MetadataType::RefMut(Box::new(expand_type(
            *inner,
            metadata,
            target,
            seen,
            normalize_abi,
        )?)),
        MetadataType::RefConst(inner) => MetadataType::RefConst(Box::new(expand_type(
            *inner,
            metadata,
            target,
            seen,
            normalize_abi,
        )?)),
        _ => ty,
    })
}
