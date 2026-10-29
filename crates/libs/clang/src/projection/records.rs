use super::*;

impl Projector<'_, '_> {
    pub(super) fn native_object_record(&self, snapshot: usize, record: RecordId) -> bool {
        let group = self.resolution.group(RecordRef { snapshot, record });
        group.definition.is_some_and(|reference| {
            let definition = self.resolution.snapshots[reference.snapshot].observations
                [reference.observation.0]
                .definition
                .as_ref()
                .unwrap();
            definition.bases.len() > 1
                && definition
                    .bases
                    .iter()
                    .all(|base| self.interface_record(reference.snapshot, base.record))
        })
    }

    pub(super) fn opaque_record(&self, snapshot: usize, record: RecordId) -> bool {
        if self.native_object_record(snapshot, record) {
            return true;
        }
        if self.interface_record(snapshot, record) {
            return false;
        }
        let Some(reference) = self
            .resolution
            .group(RecordRef { snapshot, record })
            .definition
        else {
            return true;
        };
        let definition = self.resolution.snapshots[reference.snapshot].observations
            [reference.observation.0]
            .definition
            .as_ref()
            .unwrap();
        definition.semantics.empty
            || definition.semantics.non_trivial_destructor
            || definition
                .fields
                .iter()
                .any(|field| field.access != FieldAccess::Public)
            || definition
                .bases
                .iter()
                .any(|base| self.opaque_record(reference.snapshot, base.record))
    }

    pub(super) fn check_opaque_storage(&self, snapshot: usize, ty: TypeId) -> Result<(), Error> {
        self.check_opaque_storage_inner(snapshot, ty, &mut HashSet::new())
    }

    fn check_opaque_storage_inner(
        &self,
        snapshot: usize,
        ty: TypeId,
        seen: &mut HashSet<TypeId>,
    ) -> Result<(), Error> {
        if !seen.insert(ty) {
            return Err(Error("cyclic value typedef in record storage".into()));
        }
        let source = self.resolution.snapshots[snapshot];
        let result = match source.types[ty.0].kind {
            TypeKind::Record(record) if self.opaque_record(snapshot, record) => {
                Err(Error(format!(
                    "opaque record `{}` cannot be used as value storage",
                    source.records[record.0].name
                )))
            }
            TypeKind::Alias(alias) => {
                for observation in &source.aliases[alias.0].observations {
                    self.check_opaque_storage_inner(snapshot, observation.canonical, seen)?;
                }
                Ok(())
            }
            TypeKind::Attributed { modified, .. } => {
                self.check_opaque_storage_inner(snapshot, modified, seen)
            }
            TypeKind::Array { element, .. } => {
                self.check_opaque_storage_inner(snapshot, element, seen)
            }
            _ => Ok(()),
        };
        seen.remove(&ty);
        result
    }

    pub(super) fn record(
        &self,
        snapshot_id: usize,
        observation: &RecordObservation,
        name: &TokenStream,
        inline_record: bool,
    ) -> Result<Body, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let attributes = &observation.attributes;
        let kind = snapshot.records[observation.record.0].kind;
        let anonymous_attribute = (!inline_record)
            .then(|| {
                snapshot.records[observation.record.0]
                    .owner
                    .and_then(|owner| owner.field)
            })
            .flatten()
            .map(|position| {
                let position = proc_macro2::Literal::usize_unsuffixed(position);
                quote! { #[anonymous(#position)] }
            });
        let Some(definition) = &observation.definition else {
            if inline_record
                || kind == RecordKind::Union
                || attributes.iter().any(|attribute| {
                    !matches!(
                        attribute.kind,
                        AttributeKind::Sal(_) | AttributeKind::Deprecated(_)
                    )
                })
            {
                return Err(Error(format!("unsupported incomplete record `{name}`")));
            }
            // A nominal pointee marker carries no native storage or layout claim.
            return Ok(Body {
                abi: None,
                attributes: annotations::write(attributes)?,
                declaration: quote! { struct #name {} },
            });
        };
        if self.native_object_record(snapshot_id, observation.record) {
            return self.native_object(snapshot_id, observation, name, inline_record);
        }
        if definition
            .bases
            .iter()
            .any(|base| self.native_object_record(snapshot_id, base.record))
        {
            return Err(Error(format!(
                "inheritance from native object `{name}` is unsupported"
            )));
        }
        if !definition.methods.is_empty() || self.interface_record(snapshot_id, observation.record)
        {
            if !definition.enums.is_empty() {
                return Err(Error(format!(
                    "interface-owned enums on `{name}` are unsupported"
                )));
            }

            return self.interface(snapshot_id, observation, name);
        }
        if attributes.iter().any(|attribute| {
            matches!(
                attribute.kind,
                AttributeKind::Uuid(_)
                    | AttributeKind::NoVTable
                    | AttributeKind::DllImport
                    | AttributeKind::DllExport
            )
        }) {
            return Err(Error(format!(
                "unsupported ordinary record attributes on `{name}`"
            )));
        }
        if attributes
            .iter()
            .any(|attribute| matches!(attribute.kind, AttributeKind::Unresolved(_)))
        {
            return Err(Error(format!("unresolved attributes on `{name}`")));
        }
        let constants = self.record_constants(snapshot_id, definition)?;
        if self.opaque_record(snapshot_id, observation.record) {
            let nested = self.nested_records(snapshot_id, observation.record, name)?;
            let nested = nested
                .iter()
                .map(|(leaf, (_, reference))| self.nested_declaration(leaf, *reference))
                .collect::<Result<Vec<_>, Error>>()?;
            let native_attributes = annotations::write(attributes)?;
            return Ok(Body {
                abi: None,
                attributes: quote! { #native_attributes #anonymous_attribute },
                declaration: quote! { struct #name { #(#nested)* #constants } },
            });
        }
        if definition.fields.is_empty() && definition.bases.is_empty() {
            return Err(Error(format!(
                "empty record `{name}` projection is not implemented"
            )));
        }
        let (packed, maximum) = resolve::packing(attributes);
        let packing = if packed {
            Some(1)
        } else {
            maximum.map(|bits| u64::from(bits / 8))
        };
        let requested = resolve::alignments(attributes)
            .into_iter()
            .map(|(bits, _)| bits / 8)
            .max();
        let forced = requested.filter(|bytes| *bytes != 0);
        if forced.is_some_and(|bytes| bytes > u32::from(u16::MAX)) {
            return Err(Error(format!(
                "record `{name}` alignment exceeds metadata limits"
            )));
        }
        let mut size = 0;
        let mut alignment = 1;
        let mut fields = Vec::new();
        let mut names = self.validate_record_fields(snapshot_id, name, &definition.fields)?;
        if !definition.bases.is_empty() {
            let (layout, field) =
                self.value_base(snapshot_id, &definition.bases, name, packing, &mut names)?;
            size = layout.size;
            alignment = packing.map_or(layout.alignment, |packing| layout.alignment.min(packing));
            fields.push(field);
        }
        let groups = bitfields::groups(snapshot, &definition.fields, kind)?;
        let nested = self.nested_records(snapshot_id, observation.record, name)?;
        let mut direct_uses: HashMap<RecordId, Vec<usize>> = HashMap::new();
        for (position, group) in groups.iter().enumerate() {
            let field = &group[0];
            if field.bit_width.is_none()
                && let TypeKind::Record(record) = snapshot.types[field.canonical.0].kind
                && snapshot.records[record.0]
                    .owner
                    .is_some_and(|owner| owner.record == observation.record)
                && self.written_ty(snapshot_id, field.written, field.canonical)?
                    == self.ty(snapshot_id, field.canonical)?
            {
                direct_uses.entry(record).or_default().push(position);
            }
        }
        let inline: HashSet<_> = nested
            .iter()
            .filter_map(|(leaf, (record, _))| {
                if let Some(positions) = direct_uses.get(record)
                    && let [position] = positions.as_slice()
                    && snapshot.records[record.0]
                        .owner
                        .is_some_and(|owner| owner.field == Some(*position))
                    && *leaf
                        == format!(
                            "{}_{position}",
                            public_name(
                                self.plan.names[self.plan.family(snapshot_id, observation.record)]
                                    .rsplit('/')
                                    .next()
                                    .unwrap()
                            )
                        )
                {
                    Some(*record)
                } else {
                    None
                }
            })
            .collect();
        let declarations = nested
            .iter()
            .filter(|(_, (record, _))| !inline.contains(record))
            .map(|(leaf, (_, reference))| self.nested_declaration(leaf, *reference))
            .collect::<Result<Vec<_>, Error>>()?;
        let bitfield_count = groups
            .iter()
            .filter(|group| group[0].bit_width.is_some())
            .count();
        let mut bitfield_index = 0;
        let mut anonymous = 0;
        for (position, group) in groups.into_iter().enumerate() {
            let field = &group[0];
            let (field_name, ty, members) = if field.bit_width.is_some() {
                bitfield_index += 1;
                let backing = if bitfield_count == 1 {
                    "_bitfield".to_string()
                } else {
                    format!("_bitfield{bitfield_index}")
                };
                if !names.insert(backing.clone()) {
                    return Err(Error(format!(
                        "bitfield backing `{backing}` collides in `{name}`"
                    )));
                }
                self.bitfield(snapshot_id, group, &backing)?
            } else {
                let field_name = if field.name.is_empty() {
                    anonymous += 1;
                    let field_name = if anonymous == 1 {
                        "Anonymous".to_string()
                    } else {
                        format!("Anonymous{anonymous}")
                    };
                    if !names.insert(field_name.clone()) {
                        return Err(Error(format!(
                            "anonymous field name collision in `{name}`: {field_name}"
                        )));
                    }
                    ident(&field_name)?
                } else {
                    ident(&field.name)?
                };
                let ty = if let TypeKind::Record(record) = snapshot.types[field.canonical.0].kind
                    && inline.contains(&record)
                    && direct_uses[&record][0] == position
                {
                    let (_, reference) = nested.values().find(|(id, _)| *id == record).unwrap();
                    let child = &self.resolution.snapshots[reference.snapshot].observations
                        [reference.observation.0];
                    let full_name = &self.plan.names[self.plan.family(snapshot_id, record)];
                    let leaf = ident(full_name.rsplit('/').next().unwrap())?;
                    let body = self.record(reference.snapshot, child, &leaf, true)?;
                    body.tokens()
                } else {
                    write_type(
                        self.namespace,
                        &self.written_ty(snapshot_id, field.written, field.canonical)?,
                    )
                };
                (field_name, ty, TokenStream::new())
            };
            let layout = snapshot.types[field.canonical.0]
                .layout
                .ok_or_else(|| Error(format!("field `{name}.{field_name}` has no layout")))?;
            let field_alignment = packing
                .map_or(layout.alignment, |packing| layout.alignment.min(packing))
                .max(self.required_alignment(snapshot_id, field.canonical, &mut HashSet::new()))
                .max(u64::from(
                    annotations::field_alignment(&field.attributes)?.unwrap_or(1),
                ));
            let offset = if kind == RecordKind::Union {
                0
            } else {
                round_up(size, field_alignment)
            };
            if field.offset_bits != offset * 8 {
                return Err(Error(format!(
                    "field `{name}.{field_name}` cannot use metadata layout"
                )));
            }
            size = size.max(offset + layout.size);
            alignment = alignment.max(field_alignment);
            let interface = matches!(snapshot.types[field.canonical.0].kind, TypeKind::Pointer(inner)
                if matches!(snapshot.types[inner.0].kind, TypeKind::Record(record)
                    if self.interface_record(snapshot_id, record)));
            let attributes = annotations::field(snapshot, field, &definition.fields, interface)?;
            fields.push(quote! { #attributes #field_name: #ty #members, });
        }
        alignment = alignment.max(u64::from(forced.unwrap_or(1)));
        if definition.layout
            != (Layout {
                size: round_up(size, alignment),
                alignment,
            })
        {
            return Err(Error(format!("record `{name}` cannot use metadata layout")));
        }
        let keyword = match kind {
            RecordKind::Struct | RecordKind::Class => quote! { struct },
            RecordKind::Union => quote! { union },
        };
        let declaration_name = (!inline_record).then_some(name);
        let native_attributes = annotations::write(attributes)?;
        let packing_attribute = packing.map(|packing| {
            let packing = proc_macro2::Literal::u64_unsuffixed(packing);
            quote! { #[packed(#packing)] }
        });
        let alignment_attribute = forced.map(|forced| {
            let forced = proc_macro2::Literal::u32_unsuffixed(forced);
            quote! { #[align(#forced)] }
        });
        Ok(Body {
            abi: None,
            attributes: quote! { #native_attributes #packing_attribute #alignment_attribute #anonymous_attribute },
            declaration: quote! { #keyword #declaration_name { #(#declarations)* #(#fields)* #constants } },
        })
    }

    fn record_constants(
        &self,
        snapshot: usize,
        record: &RecordDefinition,
    ) -> Result<TokenStream, Error> {
        let mut constants = TokenStream::new();
        for id in &record.enums {
            let enumeration = &self.resolution.snapshots[snapshot].enums[id.0];
            if enumeration.access != FieldAccess::Public {
                continue;
            }
            let group = self.resolution.enum_groups[snapshot][id.0];
            if !self.plan.enum_constants(self.resolution, group) {
                return Err(Error(format!(
                    "nested enum `{}` is not supported",
                    enumeration.name
                )));
            }
            let reference = self.resolution.enums[group].definition.ok_or_else(|| {
                Error(format!("member enum `{}` is incomplete", enumeration.name))
            })?;
            let observation = &self.resolution.snapshots[reference.snapshot].enums
                [reference.enumeration.0]
                .observations[reference.observation];
            if !resolve::sal(&observation.attributes).is_empty()
                || !resolve::deprecations(&observation.attributes).is_empty()
            {
                return Err(Error(format!(
                    "anonymous member enum `{}` has no annotation owner",
                    enumeration.name
                )));
            }
            let repr = self.enum_backing(reference.snapshot, observation, &enumeration.name)?;
            for (attributes, name, value) in self.enum_fields(observation, &enumeration.name)? {
                constants.extend(quote! { #attributes const #name: #repr = #value; });
            }
        }
        Ok(constants)
    }

    pub(super) fn required_alignment(
        &self,
        snapshot_id: usize,
        id: TypeId,
        seen: &mut HashSet<(usize, RecordId)>,
    ) -> u64 {
        let snapshot = self.resolution.snapshots[snapshot_id];
        match snapshot.types[id.0].kind {
            TypeKind::Array { element, .. } => self.required_alignment(snapshot_id, element, seen),
            TypeKind::Record(id) => self.record_required_alignment(snapshot_id, id, seen),
            _ => 0,
        }
    }

    fn record_required_alignment(
        &self,
        snapshot_id: usize,
        id: RecordId,
        seen: &mut HashSet<(usize, RecordId)>,
    ) -> u64 {
        if !seen.insert((snapshot_id, id)) {
            return 0;
        }
        let Some(reference) = self
            .resolution
            .group(RecordRef {
                snapshot: snapshot_id,
                record: id,
            })
            .definition
        else {
            return 0;
        };
        let snapshot_id = reference.snapshot;
        let observation =
            &self.resolution.snapshots[snapshot_id].observations[reference.observation.0];
        let definition = observation.definition.as_ref().unwrap();
        let mut required = resolve::alignments(&observation.attributes)
            .into_iter()
            .map(|(bits, _)| u64::from(bits / 8))
            .max()
            .unwrap_or(0);
        for field in &definition.fields {
            required = required
                .max(
                    resolve::alignments(&field.attributes)
                        .into_iter()
                        .map(|(bits, _)| u64::from(bits / 8))
                        .max()
                        .unwrap_or(0),
                )
                .max(self.required_alignment(snapshot_id, field.canonical, seen));
        }
        for base in &definition.bases {
            required = required.max(self.record_required_alignment(snapshot_id, base.record, seen));
        }
        required
    }

    fn validate_record_fields(
        &self,
        snapshot_id: usize,
        name: &TokenStream,
        fields: &[Field],
    ) -> Result<HashSet<String>, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let mut names = HashSet::new();
        for field in fields {
            self.check_opaque_storage(snapshot_id, field.canonical)?;
            self.check_opaque_storage(snapshot_id, field.written)?;
            if !field.name.is_empty() {
                let projected = if field.bit_width.is_some() {
                    field.name.clone()
                } else {
                    ident(&field.name)?.to_string()
                };
                if !names.insert(projected) {
                    return Err(Error(format!(
                        "duplicate projected field `{}` in `{name}`",
                        field.name
                    )));
                }
            }
            let native = checked_type(snapshot, field.canonical)?;
            if !field.attributes.iter().all(|attribute| {
                matches!(
                    attribute.kind,
                    AttributeKind::Sal(_)
                        | AttributeKind::Deprecated(_)
                        | AttributeKind::Aligned { .. }
                )
            }) || native.qualifiers.constant
                || field.bit_width.is_some() && !field.attributes.is_empty()
            {
                return Err(Error(format!(
                    "attributed or const field `{name}.{}` is unsupported",
                    field.name
                )));
            }
        }
        Ok(names)
    }

    fn value_base(
        &self,
        snapshot_id: usize,
        bases: &[Base],
        name: &TokenStream,
        packing: Option<u64>,
        names: &mut HashSet<String>,
    ) -> Result<(Layout, TokenStream), Error> {
        let [base] = bases else {
            return Err(Error(format!("record `{name}` has multiple value bases")));
        };
        if base.offset_bits != 0 || self.interface_record(snapshot_id, base.record) {
            return Err(Error(format!(
                "record `{name}` requires a value base at offset zero"
            )));
        }
        if !names.insert("base".into()) {
            return Err(Error(format!("base field name collides in `{name}`")));
        }
        let reference = self
            .resolution
            .group(RecordRef {
                snapshot: snapshot_id,
                record: base.record,
            })
            .definition
            .ok_or_else(|| Error(format!("record `{name}` has an incomplete base")))?;
        let base_observation =
            &self.resolution.snapshots[reference.snapshot].observations[reference.observation.0];
        if packing.is_some()
            && self.record_required_alignment(
                reference.snapshot,
                base_observation.record,
                &mut HashSet::new(),
            ) != 0
        {
            return Err(Error(format!(
                "packed record `{name}` contains an aligned base"
            )));
        }
        let layout = base_observation.definition.as_ref().unwrap().layout;
        let ty = write_type(
            self.namespace,
            &MetadataType::value_named(
                self.namespace,
                &self.plan.names[self.plan.family(snapshot_id, base.record)],
            ),
        );
        Ok((
            layout,
            quote! { #[Windows::Win32::Metadata::NativeBase] base: #ty, },
        ))
    }

    fn nested_declaration(
        &self,
        name: &str,
        reference: ObservationRef,
    ) -> Result<TokenStream, Error> {
        let observation =
            &self.resolution.snapshots[reference.snapshot].observations[reference.observation.0];
        Ok(self
            .record(reference.snapshot, observation, &ident(name)?, false)?
            .tokens())
    }

    fn nested_records(
        &self,
        snapshot_id: usize,
        record: RecordId,
        name: &TokenStream,
    ) -> Result<BTreeMap<&str, (RecordId, ObservationRef)>, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let mut nested = BTreeMap::new();
        for &record in &self.plan.children[snapshot_id][record.0] {
            let full_name = &self.plan.names[self.plan.family(snapshot_id, record)];
            let leaf = full_name.rsplit('/').next().unwrap();
            if nested.contains_key(leaf) {
                return Err(Error(format!(
                    "nested type name collision in `{name}`: {leaf}"
                )));
            }
            let child = snapshot.records[record.0]
                .observations
                .iter()
                .find(|id| snapshot.observations[id.0].definition.is_some())
                .map(|&observation| ObservationRef {
                    snapshot: snapshot_id,
                    observation,
                })
                .or_else(|| {
                    self.resolution
                        .group(RecordRef {
                            snapshot: snapshot_id,
                            record,
                        })
                        .definition
                })
                .ok_or_else(|| Error(format!("nested record `{full_name}` is incomplete")))?;
            nested.insert(leaf, (record, child));
        }
        Ok(nested)
    }
}

fn round_up(value: u64, alignment: u64) -> u64 {
    value.div_ceil(alignment) * alignment
}
