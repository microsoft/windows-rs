//! Native IR to Win32 RDL projection and a three-target pipeline entry point.

use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::BTreeSet;
use windows_metadata::Type as MetadataType;
use windows_rdl::emit::{write_ident, write_type};

const TARGETS: [Target; 3] = [Target::X86, Target::X64, Target::Arm64];

#[derive(Debug, Eq, PartialEq)]
pub struct Partition {
    pub header: String,
    pub rdl: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Output {
    pub partitions: BTreeMap<String, Partition>,
    pub records: usize,
    pub aliases: usize,
}

/// Captures, reconciles, and projects a configured three-target job.
pub fn generate(inputs: &[Input], namespace: &str) -> Result<Output, Error> {
    let snapshots =
        capture(inputs, &TARGETS).map_err(|error| Error(format!("capture: {error}")))?;
    let merged = merge(&snapshots).map_err(|error| Error(format!("merge: {error}")))?;
    project(&merged, namespace).map_err(|error| Error(format!("projection: {error}")))
}

/// Projects supported definitions, preserving target variants and checking sequential ABI layout.
///
/// A unique direct record typedef supplies its public name. Other typedefs are emitted only when
/// selected as roots; fields use canonical types. Ambiguous names or header ownership are errors.
pub fn project(merged: &Merged<'_>, namespace: &str) -> Result<Output, Error> {
    if merged.targets.keys().copied().collect::<Vec<_>>() != TARGETS {
        return Err(Error(
            "projection requires x86, x64, and ARM64 observations".into(),
        ));
    }
    let modules = namespace
        .split('.')
        .map(ident)
        .collect::<Result<Vec<_>, _>>()?;
    let mut records: BTreeMap<Target, BTreeMap<String, usize>> = BTreeMap::new();
    for (id, record) in merged.records.iter().enumerate() {
        for &target in record.groups.keys() {
            if records
                .entry(target)
                .or_default()
                .insert(record.name.clone(), id)
                .is_some()
            {
                return Err(Error(format!("ambiguous native record `{}`", record.name)));
            }
        }
    }
    let mut names: Vec<_> = merged
        .records
        .iter()
        .map(|record| record.name.clone())
        .collect();
    let mut nominated = BTreeMap::new();
    for (alias_id, alias) in merged.aliases.iter().enumerate() {
        let mut record_ids = BTreeSet::new();
        let mut direct = true;
        for (&target, &group) in &alias.groups {
            let resolution = &merged.targets[&target];
            for reference in &resolution.aliases[group].observations {
                let snapshot = resolution.snapshots[reference.snapshot];
                let observation =
                    &snapshot.aliases[reference.alias.0].observations[reference.observation];
                let ty = &snapshot.types[observation.underlying.0];
                if let TypeKind::Record(record) = ty.kind
                    && ty.qualifiers == Qualifiers::default()
                {
                    record_ids.insert(records[&target][&snapshot.records[record.0].name]);
                } else {
                    direct = false;
                }
            }
        }
        if direct && record_ids.len() == 1 {
            let id = *record_ids.first().unwrap();
            if let Some(previous) = nominated.insert(id, alias_id) {
                return Err(Error(format!(
                    "record `{}` has competing public typedefs `{}` and `{}`",
                    names[id], merged.aliases[previous].name, alias.name
                )));
            }
            names[id].clone_from(&alias.name);
        }
    }
    let projector = Projector {
        namespace,
        names,
        records,
    };
    let mut items = BTreeMap::new();
    let mut published = HashSet::new();
    for (id, record) in merged.records.iter().enumerate() {
        let name = ident(&projector.names[id])?;
        if !published.insert(name.to_string()) {
            return Err(Error(format!("duplicate projected name `{name}`")));
        }
        let mut headers = BTreeSet::new();
        let mut bodies = BTreeMap::new();
        for (&target, &group) in &record.groups {
            let resolution = &merged.targets[&target];
            let group = &resolution.groups[group];
            let reference = group
                .definition
                .ok_or_else(|| Error(format!("record `{name}` is incomplete")))?;
            let snapshot = resolution.snapshots[reference.snapshot];
            let observation = &snapshot.observations[reference.observation.0];
            let definition = observation.definition.as_ref().unwrap();
            let body = projector.record(snapshot, definition, &observation.attributes, &name)?;
            bodies
                .entry(body.key())
                .or_insert_with(|| (body, Vec::new()))
                .1
                .push(target);
            if let Some(&alias_id) = nominated.get(&id) {
                let alias = &merged.aliases[alias_id];
                let group = alias.groups.get(&target).ok_or_else(|| {
                    Error(format!(
                        "public typedef `{}` is absent on {target:?}",
                        alias.name
                    ))
                })?;
                alias_headers(resolution, &resolution.aliases[*group], &mut headers);
            } else {
                for reference in &group.records {
                    let snapshot = resolution.snapshots[reference.snapshot];
                    for id in &snapshot.records[reference.record.0].observations {
                        let observation = &snapshot.observations[id.0];
                        if observation.definition.is_some() {
                            headers.insert(snapshot.files[observation.location.file.0].clone());
                        }
                    }
                }
            }
        }
        insert(&mut items, &headers, &projector.names[id], variants(bodies))?;
    }
    let root_aliases: HashSet<_> = merged
        .targets
        .values()
        .flat_map(|resolution| &resolution.snapshots)
        .flat_map(|snapshot| {
            snapshot.roots.iter().filter_map(|root| match root {
                DeclarationId::Alias(alias) => Some(snapshot.aliases[alias.0].name.as_str()),
                DeclarationId::Record(_) => None,
            })
        })
        .collect();
    let mut aliases = 0;
    let nominated: HashSet<_> = nominated.into_values().collect();
    for (alias_id, alias) in merged.aliases.iter().enumerate() {
        if !root_aliases.contains(alias.name.as_str()) || nominated.contains(&alias_id) {
            continue;
        }
        let name = ident(&alias.name)?;
        if !published.insert(name.to_string()) {
            return Err(Error(format!("duplicate projected name `{name}`")));
        }
        let mut bodies = BTreeMap::new();
        let mut headers = BTreeSet::new();
        for (&target, &group) in &alias.groups {
            let resolution = &merged.targets[&target];
            let group = &resolution.aliases[group];
            alias_headers(resolution, group, &mut headers);
            let reference = group.observations[0];
            let snapshot = resolution.snapshots[reference.snapshot];
            let observation =
                &snapshot.aliases[reference.alias.0].observations[reference.observation];
            let ty = &snapshot.types[observation.canonical.0];
            if ty.qualifiers.constant || ty.layout.is_none() {
                return Err(Error(format!(
                    "typedef `{name}` is not an unqualified value type"
                )));
            }
            let ty = write_type(namespace, &projector.ty(snapshot, observation.canonical)?);
            let body = Body {
                attributes: TokenStream::new(),
                declaration: quote! { type #name = #ty; },
            };
            bodies
                .entry(body.key())
                .or_insert_with(|| (body, Vec::new()))
                .1
                .push(target);
        }
        insert(&mut items, &headers, &alias.name, variants(bodies))?;
        aliases += 1;
    }
    let mut partitions = BTreeMap::new();
    for (stem, (header, declarations)) in items {
        let mut body: TokenStream = declarations.into_values().collect();
        for module in modules.iter().rev() {
            body = quote! { mod #module { #body } };
        }
        let text = quote! { #[win32] #body }.to_string();
        partitions.insert(
            stem,
            Partition {
                header,
                rdl: windows_rdl::formatter::format(&text),
            },
        );
    }
    Ok(Output {
        partitions,
        records: merged.records.len(),
        aliases,
    })
}

struct Projector<'a> {
    namespace: &'a str,
    names: Vec<String>,
    records: BTreeMap<Target, BTreeMap<String, usize>>,
}

impl Projector<'_> {
    fn ty(&self, snapshot: &Snapshot, id: TypeId) -> Result<MetadataType, Error> {
        let ty = &snapshot.types[id.0];
        if ty.qualifiers.volatile || ty.qualifiers.restrict {
            return Err(Error(
                "volatile/restrict type projection is not implemented".into(),
            ));
        }
        Ok(match &ty.kind {
            TypeKind::Void => MetadataType::Void,
            TypeKind::Scalar(scalar) => {
                let (value, size) = match scalar {
                    Scalar::Bool => (MetadataType::Bool, 1),
                    Scalar::Char { signed: true } | Scalar::SignedChar => (MetadataType::I8, 1),
                    Scalar::Char { signed: false } | Scalar::UnsignedChar => (MetadataType::U8, 1),
                    Scalar::Short => (MetadataType::I16, 2),
                    Scalar::UnsignedShort | Scalar::WChar => (MetadataType::U16, 2),
                    Scalar::Int | Scalar::Long => (MetadataType::I32, 4),
                    Scalar::UnsignedInt | Scalar::UnsignedLong => (MetadataType::U32, 4),
                    Scalar::LongLong => (MetadataType::I64, 8),
                    Scalar::UnsignedLongLong => (MetadataType::U64, 8),
                    Scalar::Float => (MetadataType::F32, 4),
                    Scalar::Double | Scalar::LongDouble => (MetadataType::F64, 8),
                };
                if ty.layout
                    != Some(Layout {
                        size,
                        alignment: size,
                    })
                {
                    return Err(Error(format!(
                        "unsupported {scalar:?} layout on {:?}",
                        snapshot.target
                    )));
                }
                value
            }
            TypeKind::Pointer(pointee) => {
                let size = if snapshot.target == Target::X86 { 4 } else { 8 };
                if ty.layout
                    != Some(Layout {
                        size,
                        alignment: size,
                    })
                {
                    return Err(Error("unsupported pointer layout".into()));
                }
                let inner = Box::new(self.ty(snapshot, *pointee)?);
                if snapshot.types[pointee.0].qualifiers.constant {
                    MetadataType::PtrConst(inner, 1)
                } else {
                    MetadataType::PtrMut(inner, 1)
                }
            }
            TypeKind::Array { element, length } => MetadataType::ArrayFixed(
                Box::new(self.ty(snapshot, *element)?),
                (*length)
                    .try_into()
                    .map_err(|_| Error("array extent exceeds host size".into()))?,
            ),
            TypeKind::Record(record) => {
                let name =
                    &self.names[self.records[&snapshot.target][&snapshot.records[record.0].name]];
                MetadataType::value_named(self.namespace, name)
            }
            TypeKind::Alias(_) => return Err(Error("expected a canonical projection type".into())),
        })
    }

    fn record(
        &self,
        snapshot: &Snapshot,
        definition: &RecordDefinition,
        attributes: &[Attribute],
        name: &TokenStream,
    ) -> Result<Body, Error> {
        if definition.fields.is_empty() {
            return Err(Error(format!(
                "empty record `{name}` projection is not implemented"
            )));
        }
        let packed = attributes
            .iter()
            .any(|attribute| attribute.kind == AttributeKind::Packed);
        if attributes
            .iter()
            .any(|attribute| matches!(attribute.kind, AttributeKind::Unresolved(_)))
        {
            return Err(Error(format!("unresolved attributes on `{name}`")));
        }
        let mut offset = 0;
        let mut alignment = 1;
        let mut fields = Vec::new();
        let mut names = HashSet::new();
        for field in &definition.fields {
            let field_name = ident(&field.name)?;
            if !names.insert(field_name.to_string()) {
                return Err(Error(format!(
                    "duplicate projected field `{field_name}` in `{name}`"
                )));
            }
            let native = &snapshot.types[field.canonical.0];
            if !field.attributes.is_empty() || native.qualifiers.constant {
                return Err(Error(format!(
                    "attributed or const field `{name}.{field_name}` is unsupported"
                )));
            }
            let layout = native
                .layout
                .ok_or_else(|| Error(format!("field `{name}.{field_name}` has no layout")))?;
            let field_alignment = layout.alignment.min(if packed { 1 } else { 8 });
            offset = round_up(offset, field_alignment);
            if field.offset_bits != offset * 8 {
                return Err(Error(format!(
                    "field `{name}.{field_name}` cannot use sequential metadata layout"
                )));
            }
            offset += layout.size;
            alignment = alignment.max(field_alignment);
            let ty = write_type(self.namespace, &self.ty(snapshot, field.canonical)?);
            fields.push(quote! { #field_name: #ty, });
        }
        if definition.layout
            != (Layout {
                size: round_up(offset, alignment),
                alignment,
            })
        {
            return Err(Error(format!(
                "record `{name}` cannot use sequential metadata layout"
            )));
        }
        Ok(Body {
            attributes: if packed {
                quote! { #[packed(1)] }
            } else {
                TokenStream::new()
            },
            declaration: quote! { struct #name { #(#fields)* } },
        })
    }
}

fn round_up(value: u64, alignment: u64) -> u64 {
    value.div_ceil(alignment) * alignment
}

fn ident(value: &str) -> Result<TokenStream, Error> {
    if value.is_empty()
        || !value.chars().enumerate().all(|(index, ch)| {
            ch == '_' || ch.is_ascii_alphabetic() || (index > 0 && ch.is_ascii_digit())
        })
    {
        return Err(Error(format!("unsupported RDL identifier `{value}`")));
    }
    Ok(write_ident(value))
}

fn alias_headers(resolution: &Resolution<'_>, alias: &AliasGroup, headers: &mut BTreeSet<String>) {
    for reference in &alias.observations {
        let snapshot = resolution.snapshots[reference.snapshot];
        let observation = &snapshot.aliases[reference.alias.0].observations[reference.observation];
        headers.insert(snapshot.files[observation.location.file.0].clone());
    }
}

type Items = BTreeMap<String, (String, BTreeMap<String, TokenStream>)>;

fn insert(
    items: &mut Items,
    headers: &BTreeSet<String>,
    name: &str,
    body: TokenStream,
) -> Result<(), Error> {
    if headers.len() != 1 {
        return Err(Error(format!(
            "ambiguous header ownership for `{name}`: {headers:?}"
        )));
    }
    let header = headers.first().unwrap();
    let stem = std::path::Path::new(header)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| Error(format!("invalid header path `{header}`")))?;
    let stem: String = stem
        .chars()
        .filter(|ch| *ch != '.')
        .flat_map(char::to_lowercase)
        .collect();
    if stem.is_empty() {
        return Err(Error(format!("empty header partition for `{header}`")));
    }
    let (owner, declarations) = items
        .entry(stem.clone())
        .or_insert_with(|| (header.clone(), BTreeMap::new()));
    if owner != header {
        return Err(Error(format!(
            "headers `{owner}` and `{header}` collide in partition `{stem}`"
        )));
    }
    declarations.insert(name.to_string(), body);
    Ok(())
}

struct Body {
    attributes: TokenStream,
    declaration: TokenStream,
}

impl Body {
    fn key(&self) -> String {
        format!("{} {}", self.attributes, self.declaration)
    }
}

fn variants(bodies: BTreeMap<String, (Body, Vec<Target>)>) -> TokenStream {
    let mut rendered: Vec<_> = bodies
        .into_values()
        .map(|(body, targets)| {
            let arch = if targets.len() == 3 {
                TokenStream::new()
            } else {
                let names = targets.iter().map(|target| match target {
                    Target::X86 => quote! { X86 },
                    Target::X64 => quote! { X64 },
                    Target::Arm64 => quote! { Arm64 },
                });
                quote! { #[arch(#(#names)|*)] }
            };
            let Body {
                attributes,
                declaration,
            } = body;
            quote! { #attributes #arch #declaration }
        })
        .collect();
    rendered.sort_by_key(ToString::to_string);
    rendered.into_iter().collect()
}
