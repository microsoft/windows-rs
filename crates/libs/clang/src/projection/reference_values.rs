use super::*;
use windows_metadata::reader::{HasAttributes, Index, TypeCategory, TypeDef};
use windows_metadata::{AsRow, TypeAttributes, Value};

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
                        let actual = self.ty(reference.snapshot, observation.canonical)?;
                        let actual = normalize_type(
                            actual,
                            metadata.index(),
                            snapshot.target,
                            &mut HashSet::new(),
                        )?;
                        let expected = normalize_type(
                            expected,
                            metadata.index(),
                            snapshot.target,
                            &mut HashSet::new(),
                        )?;
                        if actual != expected {
                            return Err(Error(format!(
                                "external alias `{}` underlying type differs from metadata: {actual:?} vs {expected:?}",
                                alias.name
                            )));
                        }
                    } else if metadata.category() == TypeCategory::Delegate
                        && !metadata.flags().contains(TypeAttributes::WindowsRuntime)
                    {
                        let projector = Projector {
                            namespace: metadata.namespace(),
                            resolution: self.resolution,
                            plan: self.plan,
                            external: self.external,
                        };
                        let Body {
                            attributes,
                            declaration,
                        } = projector.alias(
                            reference.snapshot,
                            observation,
                            &alias.name,
                            &observation.attributes,
                        )?;
                        let rendered = windows_rdl::emit::write_item(
                            self.namespace,
                            windows_metadata::reader::Item::Type(metadata),
                        )
                        .map_err(|error| Error(error.to_string()))?;
                        let [(_, expected)] = rendered.as_slice() else {
                            return Err(Error(format!(
                                "external callback `{}` has no unique declaration",
                                alias.name
                            )));
                        };
                        if declaration_key(quote! { #attributes #declaration })
                            != declaration_key(expected.clone())
                        {
                            return Err(Error(format!(
                                "external callback `{}` declaration differs from metadata",
                                alias.name
                            )));
                        }
                    } else if !metadata.flags().contains(TypeAttributes::WindowsRuntime)
                        && self.ty(reference.snapshot, observation.canonical)?
                            != references::metadata_type(metadata)
                    {
                        return Err(Error(format!(
                            "external alias `{}` nominal type differs from metadata",
                            alias.name,
                        )));
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
            || observation
                .attributes
                .iter()
                .any(|attribute| attribute.kind == AttributeKind::FlagEnum)
                != metadata.has_attribute("FlagsAttribute")
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
            let expected = match expected {
                Value::I8(value) => EnumValue::Signed((*value).into()),
                Value::I16(value) => EnumValue::Signed((*value).into()),
                Value::I32(value) => EnumValue::Signed((*value).into()),
                Value::I64(value) => EnumValue::Signed(*value),
                Value::U8(value) => EnumValue::Unsigned((*value).into()),
                Value::U16(value) => EnumValue::Unsigned((*value).into()),
                Value::U32(value) => EnumValue::Unsigned((*value).into()),
                Value::U64(value) => EnumValue::Unsigned(*value),
                _ => return Err(fail("has an unsupported metadata member value")),
            };
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
        let fields: Vec<_> = metadata.fields().collect();
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
        let metadata_alignment = metadata
            .attributes()
            .find(|attribute| attribute.name() == "AlignmentAttribute")
            .map(|attribute| match attribute.value().as_slice() {
                [(_, Value::I32(value))] if *value > 0 => Ok(*value as u64),
                [(_, Value::U32(value))] if *value > 0 => Ok(u64::from(*value)),
                _ => Err(fail("has unsupported metadata alignment")),
            })
            .transpose()?
            .unwrap_or(0);
        if native_alignment != metadata_alignment {
            return Err(fail("alignment request differs from metadata"));
        }
        let mut size = 0u64;
        let mut alignment = 1;
        let mut anonymous = 0;
        for (native, metadata_field) in definition.fields.iter().zip(fields) {
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
            if actual != expected {
                return Err(Error(format!(
                    "external record `{name}` field `{native_name}` type differs: {actual:?} vs {expected:?}"
                )));
            }
            let actual = annotations::field(snapshot, native, &definition.fields, false)?;
            let expected =
                windows_rdl::emit::write_field_attributes(metadata_field, self.namespace)
                    .map_err(|error| Error(error.to_string()))?;
            if attribute_set(actual) != attribute_set(expected) {
                return Err(Error(format!(
                    "external record `{name}` field `{native_name}` contracts differ from metadata"
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
            .max(u64::from(
                annotations::field_alignment(&native.attributes)?.unwrap_or(0),
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
        alignment = alignment.max(metadata_alignment);
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
}

fn attribute_set(tokens: TokenStream) -> BTreeSet<String> {
    tokens
        .into_iter()
        .filter_map(|token| match token {
            proc_macro2::TokenTree::Group(group) => Some(group.stream().to_string()),
            _ => None,
        })
        .collect()
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

fn normalize_type(
    ty: MetadataType,
    metadata: &Index,
    target: Target,
    seen: &mut HashSet<(String, String)>,
) -> Result<MetadataType, Error> {
    Ok(match ty {
        MetadataType::ValueName(ref name) => {
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
                && definition.has_attribute("NativeTypedefAttribute")
            {
                let key = (name.namespace.clone(), name.name.clone());
                if !seen.insert(key.clone()) {
                    return Err(Error(format!("cyclic referenced alias `{name:?}`")));
                }
                let underlying = definition
                    .underlying_type()
                    .ok_or_else(|| Error(format!("referenced alias `{name:?}` has no type")))?;
                let result = normalize_type(underlying, metadata, target, seen)?;
                seen.remove(&key);
                result
            } else {
                ty
            }
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
        MetadataType::PtrMut(inner, count) => match normalize_type(*inner, metadata, target, seen)?
        {
            MetadataType::PtrMut(inner, nested) => MetadataType::PtrMut(inner, count + nested),
            inner => MetadataType::PtrMut(Box::new(inner), count),
        },
        MetadataType::PtrConst(inner, count) => {
            match normalize_type(*inner, metadata, target, seen)? {
                MetadataType::PtrConst(inner, nested) => {
                    MetadataType::PtrConst(inner, count + nested)
                }
                inner => MetadataType::PtrConst(Box::new(inner), count),
            }
        }
        MetadataType::Ptr64(inner, constant) => MetadataType::Ptr64(
            Box::new(normalize_type(*inner, metadata, target, seen)?),
            constant,
        ),
        MetadataType::ArrayFixed(inner, count) => MetadataType::ArrayFixed(
            Box::new(normalize_type(*inner, metadata, target, seen)?),
            count,
        ),
        _ => ty,
    })
}
