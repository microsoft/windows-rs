use super::*;

impl Projector<'_, '_> {
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
        let Some(definition) = &observation.definition else {
            if inline_record
                || kind != RecordKind::Struct
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
                attributes: annotations::write(attributes)?,
                declaration: quote! { struct #name {} },
            });
        };
        if !definition.methods.is_empty() || self.interface_record(snapshot_id, observation.record)
        {
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
        if packing.is_some() && forced.is_some() {
            return Err(Error(format!(
                "record `{name}` combines packing and alignment"
            )));
        }
        if attributes
            .iter()
            .any(|attribute| matches!(attribute.kind, AttributeKind::Unresolved(_)))
        {
            return Err(Error(format!("unresolved attributes on `{name}`")));
        }
        let mut size = 0;
        let mut alignment = 1;
        let mut fields = Vec::new();
        let mut names =
            self.validate_record_fields(snapshot_id, name, &definition.fields, packing)?;
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
            .map(|(leaf, (_, reference))| {
                let observation = &self.resolution.snapshots[reference.snapshot].observations
                    [reference.observation.0];
                let body = self.record(reference.snapshot, observation, &ident(leaf)?, false)?;
                let Body {
                    attributes,
                    declaration,
                } = body;
                Ok(quote! { #attributes #declaration })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let bitfield_count = groups
            .iter()
            .filter(|group| group[0].bit_width.is_some())
            .count();
        let mut bitfield_index = 0;
        let mut anonymous = 0;
        for (position, group) in groups.into_iter().enumerate() {
            let field = &group[0];
            let (field_name, ty, members, member_attributes) = if field.bit_width.is_some() {
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
                    let Body {
                        attributes,
                        declaration,
                    } = body;
                    quote! { #attributes #declaration }
                } else {
                    write_type(
                        self.namespace,
                        &self.written_ty(snapshot_id, field.written, field.canonical)?,
                    )
                };
                (field_name, ty, TokenStream::new(), TokenStream::new())
            };
            let layout = snapshot.types[field.canonical.0]
                .layout
                .ok_or_else(|| Error(format!("field `{name}.{field_name}` has no layout")))?;
            let field_alignment = packing
                .map_or(layout.alignment, |packing| layout.alignment.min(packing))
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
            let attributes = annotations::field(snapshot, field, &definition.fields)?;
            fields.push(quote! { #attributes #member_attributes #field_name: #ty #members, });
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
        let layout_attributes = if let Some(packing) = packing {
            let packing = proc_macro2::Literal::u64_unsuffixed(packing);
            quote! { #[packed(#packing)] }
        } else if let Some(forced) = forced {
            let forced = proc_macro2::Literal::u32_unsuffixed(forced);
            quote! { #[align(#forced)] }
        } else {
            TokenStream::new()
        };
        Ok(Body {
            attributes: quote! { #native_attributes #layout_attributes },
            declaration: quote! { #keyword #declaration_name { #(#declarations)* #(#fields)* } },
        })
    }

    fn contains_alignment(
        &self,
        snapshot_id: usize,
        id: TypeId,
        seen: &mut HashSet<RecordId>,
    ) -> bool {
        let snapshot = self.resolution.snapshots[snapshot_id];
        match snapshot.types[id.0].kind {
            TypeKind::Array { element, .. } => self.contains_alignment(snapshot_id, element, seen),
            TypeKind::Record(id) => self.record_contains_alignment(snapshot_id, id, seen),
            _ => false,
        }
    }

    fn record_contains_alignment(
        &self,
        snapshot_id: usize,
        id: RecordId,
        seen: &mut HashSet<RecordId>,
    ) -> bool {
        let snapshot = self.resolution.snapshots[snapshot_id];
        seen.insert(id)
            && snapshot.records[id.0].observations.iter().any(|id| {
                let observation = &snapshot.observations[id.0];
                resolve::alignments(&observation.attributes)
                    .iter()
                    .any(|(bits, _)| *bits != 0)
                    || observation.definition.as_ref().is_some_and(|definition| {
                        definition.fields.iter().any(|field| {
                            resolve::alignments(&field.attributes)
                                .iter()
                                .any(|(bits, _)| *bits != 0)
                                || self.contains_alignment(snapshot_id, field.canonical, seen)
                        }) || definition.bases.iter().any(|base| {
                            self.record_contains_alignment(snapshot_id, base.record, seen)
                        })
                    })
            })
    }

    fn validate_record_fields(
        &self,
        snapshot_id: usize,
        name: &TokenStream,
        fields: &[Field],
        packing: Option<u64>,
    ) -> Result<HashSet<String>, Error> {
        let snapshot = self.resolution.snapshots[snapshot_id];
        let mut names = HashSet::new();
        for field in fields {
            if !field.name.is_empty() && !names.insert(ident(&field.name)?.to_string()) {
                return Err(Error(format!(
                    "duplicate projected field `{}` in `{name}`",
                    field.name
                )));
            }
            let native = &snapshot.types[field.canonical.0];
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
            if packing.is_some()
                && (annotations::field_alignment(&field.attributes)?.is_some()
                    || self.contains_alignment(snapshot_id, field.canonical, &mut HashSet::new()))
            {
                return Err(Error(format!(
                    "packed record `{name}` contains an aligned type"
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
            && self.record_contains_alignment(
                reference.snapshot,
                base_observation.record,
                &mut HashSet::new(),
            )
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
