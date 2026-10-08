use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum RecordKind {
    Struct,
    Union,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Record {
    pub kind: RecordKind,
    pub fields: Vec<(String, String, ProjectedType)>,
    pub layout: Layout,
    pub alignment: Option<i64>,
    pub anonymous_fields: bool,
    attributes: String,
}

impl Record {
    pub fn write(&self, output: &mut String, name: Option<&str>, indent: usize) {
        if name.is_none() {
            output.push_str(&self.attributes);
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
        let kind = match kind.as_str() {
            "StructDecl" => RecordKind::Struct,
            "UnionDecl" => RecordKind::Union,
            _ => return Err(Error(format!("unsupported record kind for `{name}`"))),
        };
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
        for (index, field) in fields.iter().enumerate() {
            if field.bit_width.is_some() {
                return Err(Error(format!(
                    "bitfield projection is not implemented for `{name}`"
                )));
            }
            if let TypeKind::Named(target) = field.ty.kind {
                let target = self.resolved.representatives[target.0];
                anonymous_fields |= matches!(
                    self.resolved.snapshot.declarations[target.0].data,
                    DeclarationData::Record { unnamed: true, .. }
                );
            }
            let ty = self.record_field(&field.ty)?;
            let field_name = if field.name.is_empty() {
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
            if kind == RecordKind::Union {
                if field.offset != 0 {
                    return Err(Error(format!("nonzero union field offset for `{name}`")));
                }
                size = size.max(field_layout.size);
            } else {
                let natural = align_up(size, field_layout.align);
                if field.offset < natural * 8 || field.offset % (field_layout.align * 8) != 0 {
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
            align = align.max(field_layout.align);
            projected.push((
                source_attributes(&annotations.fields[index]),
                field_name,
                ty,
            ));
        }
        if layout.align < align
            || layout.align > 32768
            || !layout.align.is_positive()
            || !(layout.align as u64).is_power_of_two()
        {
            return Err(Error(format!("unsupported record alignment for `{name}`")));
        }
        let expected = Layout {
            size: align_up(size, layout.align),
            align: layout.align,
        };
        if expected != *layout {
            return Err(Error(format!("projected layout differs for `{name}`")));
        }
        Ok(Record {
            kind,
            fields: projected,
            layout: layout.clone(),
            alignment: (layout.align > align).then_some(layout.align),
            anonymous_fields,
            attributes: source_attributes(&annotations.own),
        })
    }
}
