use super::*;

impl Snapshot {
    pub(super) fn check_records(&self) -> Result<()> {
        for (id, decl) in self.declarations.iter().enumerate() {
            if matches!(decl.kind, DeclarationKind::Record { .. }) {
                self.record_layout(id, &mut BTreeSet::new())?;
            }
        }
        Ok(())
    }

    fn storage(&self, ty: &NativeType, visiting: &mut BTreeSet<Id>) -> Result<Layout> {
        match &ty.kind {
            NativeKind::Alias(id) => {
                if !visiting.insert(*id) {
                    return Err(Error::new(
                        Stage::Representation,
                        "cyclic record storage alias",
                    ));
                }
                let DeclarationKind::Alias { written, .. } = &self.declarations[*id].kind else {
                    return Err(Error::new(
                        Stage::Representation,
                        "storage alias targets a non-alias",
                    ));
                };
                let layout = self.storage(written, visiting)?;
                visiting.remove(id);
                Ok(layout)
            }
            NativeKind::Record(id) => self.record_layout(*id, visiting),
            NativeKind::Scalar(_) | NativeKind::Pointer(_) => ty.layout.ok_or_else(|| {
                Error::new(Stage::Representation, "unavailable field storage layout")
            }),
            NativeKind::Array(element, count) if *count != 0 => {
                let element = self.storage(element, visiting)?;
                let size = element.size.checked_mul(*count as u64).ok_or_else(|| {
                    Error::new(Stage::Representation, "array storage size overflow")
                })?;
                Ok(Layout {
                    size,
                    alignment: element.alignment,
                })
            }
            _ => Err(Error::new(
                Stage::Representation,
                "void, references, or zero-length arrays cannot supply C3-A field storage",
            )),
        }
    }

    fn record_layout(&self, id: Id, visiting: &mut BTreeSet<Id>) -> Result<Layout> {
        let decl = &self.declarations[id];
        let DeclarationKind::Record {
            union,
            fields,
            contract,
        } = &decl.kind
        else {
            return Err(Error::new(
                Stage::Representation,
                format!(
                    "{}: incomplete record cannot supply storage",
                    decl.evidence.qualified_name
                ),
            ));
        };
        let contract = contract.unwrap_or_default();
        if !visiting.insert(id) {
            return Err(Error::new(
                Stage::Representation,
                "cyclic by-value record storage",
            ));
        }
        let (mut size, mut alignment) = (0, 1);
        for field in fields {
            let layout = self.storage(&field.written, visiting)?;
            if field.evidence.layout != Some(layout) {
                return Err(Error::new(
                    Stage::Representation,
                    format!(
                        "{}.{}: field storage disagrees with its emitted type",
                        decl.evidence.qualified_name, field.evidence.name
                    ),
                ));
            }
            let required = self.required_alignment(&field.written, &mut BTreeSet::new())?;
            let field_alignment = layout
                .alignment
                .min(contract.packing.map_or(u64::MAX, u64::from))
                .max(required);
            alignment = alignment.max(field_alignment);
            let offset = if *union {
                0
            } else {
                round(size, field_alignment)?
            };
            if offset != field.evidence.offset {
                return Err(Error::new(
                    Stage::Representation,
                    format!(
                        "{}.{}: native offset {} differs from checked offset {offset}; \
                        explicit adjustments require matched native contracts",
                        decl.evidence.qualified_name, field.evidence.name, field.evidence.offset
                    ),
                ));
            }
            let end = offset
                .checked_add(layout.size)
                .ok_or_else(|| Error::new(Stage::Representation, "record storage size overflow"))?;
            size = size.max(end);
        }
        visiting.remove(&id);
        alignment = alignment.max(contract.alignment.map_or(1, u64::from));
        let layout = Layout {
            size: round(size, alignment)?,
            alignment,
        };
        if decl.evidence.layout != Some(layout) {
            return Err(Error::new(
                Stage::Representation,
                format!(
                    "{}: native size/alignment differs from checked storage; \
                    explicit adjustments require matched native contracts",
                    decl.evidence.qualified_name
                ),
            ));
        }
        Ok(layout)
    }

    fn required_alignment(&self, ty: &NativeType, visiting: &mut BTreeSet<Id>) -> Result<u64> {
        match &ty.kind {
            NativeKind::Alias(id) => {
                if !visiting.insert(*id) {
                    return Err(Error::new(Stage::Representation, "cyclic alignment alias"));
                }
                let DeclarationKind::Alias { written, .. } = &self.declarations[*id].kind else {
                    return Err(Error::new(
                        Stage::Representation,
                        "alignment alias targets a non-alias",
                    ));
                };
                let result = self.required_alignment(written, visiting)?;
                visiting.remove(id);
                Ok(result)
            }
            NativeKind::Array(element, _) => self.required_alignment(element, visiting),
            NativeKind::Record(id) => match &self.declarations[*id].kind {
                DeclarationKind::Record {
                    contract: Some(contract),
                    ..
                } => Ok(u64::from(contract.required_alignment)),
                _ => Ok(0),
            },
            _ => Ok(0),
        }
    }
}

fn round(size: u64, alignment: u64) -> Result<u64> {
    if !alignment.is_power_of_two() {
        return Err(Error::new(
            Stage::Representation,
            "invalid native storage alignment",
        ));
    }
    size.checked_add(alignment - 1)
        .map(|size| size & !(alignment - 1))
        .ok_or_else(|| Error::new(Stage::Representation, "aligned storage size overflow"))
}
