use super::*;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Record {
    pub kind: RecordKind,
    pub fields: Vec<(String, String, ProjectedType)>,
    pub layout: Layout,
    pub alignment: Option<i64>,
    pub packing: Option<i64>,
    pub bitfields: bool,
    pub anonymous_fields: bool,
    attributes: String,
    bitfield_members: BTreeMap<String, Vec<(String, i32)>>,
}

struct BitfieldUnit {
    name: String,
    offset: i64,
    layout: Layout,
    used: i64,
}

impl Record {
    pub fn write(&self, output: &mut String, name: Option<&str>, indent: usize) {
        if name.is_none() {
            output.push_str(&self.attributes);
        }
        if let Some(packing) = self.packing {
            write!(output, "#[packed({packing})]").unwrap();
            if name.is_some() {
                write!(output, "\n{:indent$}", "").unwrap();
            } else {
                output.push(' ');
            }
        }
        if let Some(alignment) = self.alignment {
            write!(output, "#[align({alignment})]").unwrap();
            if name.is_some() {
                write!(output, "\n{:indent$}", "").unwrap();
            } else {
                output.push(' ');
            }
        }
        output.push_str(match self.kind {
            RecordKind::Struct => "struct",
            RecordKind::Union => "union",
            RecordKind::Class => unreachable!(),
        });
        if let Some(name) = name {
            write!(output, " {name}").unwrap();
        }
        output.push_str(" {\n");
        for (attributes, field, ty) in &self.fields {
            let field_indent = indent + 4;
            if !attributes.is_empty() {
                writeln!(output, "{:field_indent$}{}", "", attributes.trim_end()).unwrap();
            }
            write!(output, "{:field_indent$}{field}: ", "").unwrap();
            if let ProjectedType::InlineRecord(record) = ty {
                record.write(output, None, field_indent);
            } else {
                output.push_str(&ty.text());
            }
            if let Some(members) = self.bitfield_members.get(field) {
                output.push_str(" {\n");
                for (name, width) in members {
                    let name = if name.is_empty() { "_" } else { name };
                    writeln!(
                        output,
                        "{:member_indent$}{name}: {width},",
                        "",
                        member_indent = field_indent + 4
                    )
                    .unwrap();
                }
                write!(output, "{:field_indent$}}}", "").unwrap();
            }
            output.push_str(",\n");
        }
        write!(output, "{:indent$}}}", "").unwrap();
    }
}

impl Builder<'_, '_> {
    fn record_field(&mut self, ty: &Type) -> Result<ProjectedType, Error> {
        if let TypeKind::Named(id) = ty.kind {
            let id = self.resolved.representatives[id.0];
            let declaration = &self.resolved.snapshot.declarations[id.0];
            if matches!(
                declaration.data,
                DeclarationData::Record { unnamed: true, .. }
            ) && !self.names.contains_key(&id)
                && !self.options.references.contains_key(&declaration.name)
            {
                return Ok(ProjectedType::InlineRecord(Box::new(self.record(id)?)));
            }
        }
        let (ty, object) = self.lower(ty, &mut BTreeSet::new())?;
        if object {
            return Err(Error("native interface objects require a pointer".into()));
        }
        Ok(ty)
    }

    pub(super) fn record(&mut self, id: Id) -> Result<Record, Error> {
        let declaration = &self.resolved.snapshot.declarations[id.0];
        let name = &declaration.name;
        let DeclarationData::Record {
            kind,
            complete: true,
            layout: Some(layout),
            fields,
            bases,
            methods,
            ..
        } = &declaration.data
        else {
            return Err(Error(format!("record layout unavailable for `{name}`")));
        };
        if *kind == RecordKind::Class {
            return Err(Error(format!("unsupported record kind for `{name}`")));
        }
        if !bases.is_empty() || !methods.is_empty() {
            return Err(Error(format!(
                "unsupported C++ record storage for `{name}`"
            )));
        }
        if fields.is_empty() {
            return Err(Error(format!(
                "empty record projection is not implemented for `{name}`"
            )));
        }
        let annotations = &self.resolved.annotations[&id];
        let mut projected = vec![];
        let mut size = 0;
        let mut align = 1;
        let mut anonymous_fields = false;
        let mut names: BTreeSet<_> = fields.iter().map(|field| field.name.clone()).collect();
        if layout.align > 32768
            || !layout.align.is_positive()
            || !(layout.align as u64).is_power_of_two()
        {
            return Err(Error(format!("unsupported record alignment for `{name}`")));
        }
        let mut packing = None;
        let mut bitfield_members: BTreeMap<String, Vec<(String, i32)>> = BTreeMap::new();
        let mut bitfield_unit: Option<BitfieldUnit> = None;
        for (index, field) in fields.iter().enumerate() {
            if let TypeKind::Named(target) = field.ty.kind {
                let target = self.resolved.representatives[target.0];
                anonymous_fields |= matches!(
                    self.resolved.snapshot.declarations[target.0].data,
                    DeclarationData::Record { unnamed: true, .. }
                );
            }
            let ty = self.record_field(&field.ty)?;
            let mut field_name = if field.bit_width.is_some() {
                String::new()
            } else if field.name.is_empty() {
                if !matches!(ty, ProjectedType::InlineRecord(_)) {
                    return Err(Error(format!(
                        "anonymous field requires a captured nested record in `{name}`"
                    )));
                }
                let mut name = format!("Anonymous{index}");
                while !names.insert(name.clone()) {
                    name.push('_');
                }
                name
            } else {
                ident(&field.name)?
            };
            let field_layout = ty
                .layout(self.resolved.snapshot.pointer_size)
                .ok_or_else(|| {
                    Error(format!(
                        "projected layout unavailable for `{name}::{field_name}`"
                    ))
                })?;
            let attributes = source_attributes(&annotations.fields[index]);
            if let Some(width) = field.bit_width {
                if *kind != RecordKind::Struct
                    || !self.resolved.snapshot.target.contains("-windows-msvc")
                {
                    return Err(Error(format!(
                        "bitfield projection requires MSVC struct allocation units for `{name}`"
                    )));
                }
                let alias_qualified = if let TypeKind::Named(id) = field.ty.kind
                    && let DeclarationData::Alias { canonical, .. } =
                        &self.resolved.snapshot.declarations[self.resolved.representatives[id.0].0]
                            .data
                {
                    canonical.qualifiers != Qualifiers::default()
                } else {
                    false
                };
                if field.ty.qualifiers != Qualifiers::default()
                    || alias_qualified
                    || !attributes.is_empty()
                {
                    return Err(Error(format!(
                        "qualified or annotated bitfield projection is not implemented for `{name}`"
                    )));
                }
                if width == 0 {
                    bitfield_unit = None;
                    continue;
                }
                if !matches!(ty, ProjectedType::Scalar("u8" | "u16" | "u32" | "u64", _))
                    || width < 0
                    || i64::from(width) > field_layout.size * 8
                    || field.name == "_"
                {
                    return Err(Error(format!(
                        "bitfield projection requires an unsigned integer backing and representable member name for `{name}`"
                    )));
                }
                let member = if field.name.is_empty() {
                    String::new()
                } else {
                    ident(&field.name)?
                };
                if let Some(unit) = &mut bitfield_unit
                    && unit.layout == field_layout
                    && field.offset >= unit.offset + unit.used
                    && field.offset + i64::from(width) <= unit.offset + unit.layout.size * 8
                {
                    let members = bitfield_members.get_mut(&unit.name).unwrap();
                    let gap = field.offset - unit.offset - unit.used;
                    if gap > 0 {
                        members.push((String::new(), gap.try_into().unwrap()));
                    }
                    members.push((member, width));
                    unit.used = field.offset - unit.offset + i64::from(width);
                    continue;
                }
                field_name = format!("__bitfield{index}");
                while !names.insert(field_name.clone()) {
                    field_name.push('_');
                }
                bitfield_members.insert(field_name.clone(), vec![(member, width)]);
                bitfield_unit = Some(BitfieldUnit {
                    name: field_name.clone(),
                    offset: field.offset,
                    layout: field_layout.clone(),
                    used: i64::from(width),
                });
            } else {
                bitfield_unit = None;
            }
            // This cap describes equivalent storage, not the original packing directive.
            let field_align = field_layout.align.min(layout.align);
            if field_align < field_layout.align {
                packing = Some(layout.align);
            }
            if *kind == RecordKind::Union {
                if field.offset != 0 {
                    return Err(Error(format!("nonzero union field offset for `{name}`")));
                }
                size = size.max(field_layout.size);
            } else {
                let natural = align_up(size, field_align);
                if field.offset < natural * 8 || field.offset % (field_align * 8) != 0 {
                    return Err(Error(format!(
                        "`{name}` requires unsupported packing or field alignment"
                    )));
                }
                let offset = field.offset / 8;
                if offset > natural {
                    let mut padding = format!("__padding{index}");
                    while !names.insert(padding.clone()) {
                        padding.push('_');
                    }
                    projected.push((
                        String::new(),
                        padding,
                        ProjectedType::Padding(offset - size),
                    ));
                }
                size = offset + field_layout.size;
            }
            align = align.max(field_align);
            projected.push((attributes, field_name, ty));
        }
        let expected = Layout {
            size: align_up(size, layout.align),
            align: layout.align,
        };
        if expected != *layout {
            return Err(Error(format!("projected layout differs for `{name}`")));
        }
        Ok(Record {
            kind: *kind,
            fields: projected,
            layout: layout.clone(),
            alignment: (layout.align > align).then_some(layout.align),
            packing,
            bitfields: fields.iter().any(|field| field.bit_width.is_some()),
            anonymous_fields,
            attributes: source_attributes(&annotations.own),
            bitfield_members,
        })
    }
}

impl Plan {
    pub(super) fn validate_record_storage(&self) -> Result<(), Error> {
        for (name, item) in &self.items {
            if let Item::Record(record) = item {
                self.validate_packed_record(record, name)?;
            }
        }
        Ok(())
    }

    fn validate_packed_record(&self, record: &Record, name: &str) -> Result<(), Error> {
        for (_, _, ty) in &record.fields {
            if record.packing.is_some()
                && self.has_unproven_packed_alignment(ty, &mut BTreeSet::new())
            {
                return Err(Error(format!(
                    "packed record `{name}` contains forced or unproven external alignment"
                )));
            }
            self.validate_inline_packing(ty, name)?;
        }
        Ok(())
    }

    fn validate_inline_packing(&self, ty: &ProjectedType, name: &str) -> Result<(), Error> {
        match ty.contract() {
            ProjectedType::InlineRecord(record) => self.validate_packed_record(record, name),
            ProjectedType::Array { element, .. } => self.validate_inline_packing(element, name),
            _ => Ok(()),
        }
    }

    fn has_unproven_packed_alignment(
        &self,
        ty: &ProjectedType,
        checked: &mut BTreeSet<String>,
    ) -> bool {
        match ty.contract() {
            ProjectedType::RecordReference(..) => true,
            ProjectedType::Array { element, .. } => {
                self.has_unproven_packed_alignment(element, checked)
            }
            ProjectedType::InlineRecord(record) => {
                record.alignment.is_some()
                    || record
                        .fields
                        .iter()
                        .any(|(_, _, ty)| self.has_unproven_packed_alignment(ty, checked))
            }
            ProjectedType::Named(name, _) if checked.insert(name.clone()) => {
                self.items.get(name).is_some_and(|item| {
                    if let Item::Record(record) = item {
                        record.alignment.is_some()
                            || record
                                .fields
                                .iter()
                                .any(|(_, _, ty)| self.has_unproven_packed_alignment(ty, checked))
                    } else {
                        false
                    }
                })
            }
            _ => false,
        }
    }
}
