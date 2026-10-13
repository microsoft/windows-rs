use super::*;
use windows_metadata::{
    Type,
    reader::{File, Index, TypeCategory, TypeDef},
};

pub(super) fn run(output: &Path, baseline: &Path) -> Result<()> {
    let mut report = String::from(
        "architecture\tprofile\titem\tfacet\tlegacy\tcandidate\tclassification\tevidence\n",
    );
    let mut blocked = 0;
    let mut source_names = BTreeSet::new();
    for header in ["tlhelp32", "shellscalingapi"] {
        let inventory = windows_rdl::inventory(format!("metadata\\win32\\{header}.rdl"))?;
        source_names.extend(
            inventory
                .iter()
                .filter(|entry| {
                    entry
                        .parent
                        .is_some_and(|parent| inventory[parent].kind == "namespace")
                        && entry.kind != "namespace"
                })
                .map(|entry| entry.name.clone()),
        );
    }
    for (architecture, bit) in [("x86", 1), ("x64", 2), ("arm64", 4)] {
        let legacy = Index::new_for_architecture(
            vec![File::read(baseline).ok_or("legacy comparison unavailable")?],
            bit,
        );
        let reference = Index::new_for_architecture(
            vec![
                File::read("crates\\libs\\default\\Windows.Win32.winmd")
                    .ok_or("legacy reference unavailable")?,
            ],
            bit,
        );
        for encoding in ["ansi", "unicode"] {
            let profile = format!("combined-{encoding}");
            let candidate = Index::read(output.join(architecture).join(format!("{profile}.winmd")))
                .ok_or("candidate comparison unavailable")?;
            let candidate_types = Model {
                index: &candidate,
                reference: None,
            };
            let legacy_types = Model {
                index: &legacy,
                reference: Some(&reference),
            };
            let names = candidate
                .namespace_items("Windows.Win32")
                .map(|(name, _)| name.to_string())
                .collect::<BTreeSet<_>>();
            for name in &names {
                let items = candidate
                    .get_item("Windows.Win32", name)
                    .collect::<Vec<_>>();
                let [item] = items.as_slice() else {
                    return Err("ambiguous candidate comparison identity".into());
                };
                let current = facets(*item, &candidate_types)?;
                let previous = legacy.get_item("Windows.Win32", name).collect::<Vec<_>>();
                let previous = if previous.is_empty() {
                    reference
                        .iter_items()
                        .filter(|(_, candidate_name, _)| candidate_name == name)
                        .map(|(_, _, item)| item)
                        .collect::<Vec<_>>()
                } else {
                    previous
                };
                let old = match previous.as_slice() {
                    [] => None,
                    [item] => Some(facets(*item, &legacy_types)?),
                    _ => return Err(format!("ambiguous legacy item `{name}`").into()),
                };
                if let Some(old) = old {
                    let [old_item] = previous.as_slice() else {
                        unreachable!()
                    };
                    verify_memory_sizes(*old_item, *item)?;
                    for facet in old.keys().chain(current.keys()).collect::<BTreeSet<_>>() {
                        let before = old.get(facet).map_or("<absent>", String::as_str);
                        let after = current.get(facet).map_or("<absent>", String::as_str);
                        let classification = classify(facet, before, after);
                        if classification == "unclassified" {
                            blocked += 1;
                        }
                        writeln!(
                            report,
                            "{architecture}\t{profile}\t{}\t{}\t{}\t{}\t{classification}\tcompiler-graph-and-WinMD-readback;{}",
                            escape(name),
                            escape(facet),
                            escape(before),
                            escape(after),
                            escape(&format!("{architecture}\\{profile}.semantic.tsv"))
                        )?;
                    }
                } else {
                    writeln!(
                        report,
                        "{architecture}\t{profile}\t{}\tpresence\t<absent>\t{}\trestored-source-declaration\tcompiler-graph-and-closed-dependency-readback",
                        escape(name),
                        escape(&format!("{current:?}"))
                    )?;
                }
            }
            // Only the two complete SDK partitions are exhaustive legacy-root comparisons.
            for (_, name, _) in legacy
                .iter_items()
                .filter(|(_, name, _)| source_names.contains(*name))
            {
                if !names.contains(name) {
                    blocked += 1;
                    writeln!(
                        report,
                        "{architecture}\t{profile}\t{}\tpresence\tpublished\t<absent>\tunclassified\tmissing-legacy-SDK-root",
                        escape(name)
                    )?;
                }
            }
        }
    }
    std::fs::write(output.join("differences.tsv"), report)?;
    std::fs::write(
        output.join("difference-inputs.tsv"),
        format!(
            "input\tgit-blob\n{}\n",
            [
                "metadata\\win32\\tlhelp32.rdl",
                "metadata\\win32\\shellscalingapi.rdl",
                "metadata\\win32\\unknwnbase.rdl",
                "metadata\\win32\\objidlbase.rdl",
                "crates\\libs\\default\\Windows.Win32.winmd"
            ]
            .iter()
            .map(|path| Ok(format!(
                "{path}\t{}",
                git(&["hash-object", "--no-filters", path])?.trim()
            )))
            .collect::<Result<Vec<_>>>()?
            .join("\n")
        ),
    )?;
    if blocked != 0 {
        return Err(
            format!("{blocked} unclassified legacy differences; see differences.tsv").into(),
        );
    }
    Ok(())
}

fn classify(facet: &str, before: &str, after: &str) -> &'static str {
    if before == after {
        return "preserved";
    }
    if let Some(member) = facet.strip_prefix("method:")
        && let Some((_, facet)) = member.split_once(':')
    {
        return classify(facet, before, after);
    }
    if let Some(parameter) = facet.strip_prefix("parameter:")
        && let Some((_, facet)) = parameter.split_once(':')
    {
        return classify(facet, before, after);
    }
    if facet.starts_with("attribute:") {
        return match facet.strip_prefix("attribute:").unwrap() {
            "NativeAnnotationAttribute"
            | "NativeAnnotationBindingAttribute"
            | "NativeBufferExtentAttribute"
            | "NativeMacroAliasAttribute" => "source-evidence-restored",
            "NativeTypedefAttribute" => "written-typedef-identity",
            "NativeVtableAttribute" | "NativeSlotAttribute" => "compiler-vtable-evidence",
            "GuidAttribute" => "compiler-uuid-contract",
            "FlagsAttribute"
            | "RAIIFreeAttribute"
            | "InvalidHandleValueAttribute"
            | "NativeEncodingAttribute"
            | "StructSizeFieldAttribute"
            | "AlsoUsableForAttribute"
            | "AssociatedEnumAttribute"
            | "SupportedOSPlatformAttribute" => "legacy-publication-policy",
            "NativeArrayInfoAttribute" | "MemorySizeAttribute" | "ComOutPtrAttribute" => {
                "source-positional-contract"
            }
            _ => "unclassified",
        };
    }
    if facet == "route" {
        return "archive-backed-import";
    }
    if facet == "convention" {
        return "target-native-ABI";
    }
    if facet == "packing" {
        return "compiler-layout-contract";
    }
    if facet == "kind" {
        return "source-typedef-tag-separation";
    }
    if facet == "signature" || facet.starts_with("field:") || facet == "underlying" {
        return "source-written-type-contract";
    }
    if facet.starts_with("parameter:") {
        return "source-positional-contract";
    }
    if facet == "base" || facet == "slot" || facet == "vtable" {
        return "compiler-vtable-contract";
    }
    if facet == "value" || facet.starts_with("member:") {
        return "compiler-value-contract";
    }
    if facet == "alias" {
        return "written-typedef-identity";
    }
    "unclassified"
}

fn verify_memory_sizes(old: Item<'_>, new: Item<'_>) -> Result<()> {
    fn method(
        old: windows_metadata::reader::MethodDef<'_>,
        new: windows_metadata::reader::MethodDef<'_>,
    ) -> Result<()> {
        let old_params = old.params_by_sequence(old.signature(&[]).types.len())?;
        let new_params = new.params_by_sequence(new.signature(&[]).types.len())?;
        for (position, parameter) in old_params.params().iter().enumerate() {
            let Some(parameter) = parameter else {
                continue;
            };
            for attribute in parameter
                .attributes()
                .filter(|attribute| attribute.name() == "MemorySizeAttribute")
            {
                let values = attribute.value();
                let [(name, Value::I16(count))] = values.as_slice() else {
                    return Err("unclassified legacy memory-size descriptor".into());
                };
                if name != "BytesParamIndex" || *count < 0 {
                    return Err("unclassified legacy memory-size binding".into());
                }
                let new = new_params
                    .params()
                    .get(position)
                    .copied()
                    .flatten()
                    .ok_or("legacy buffer parameter missing from candidate")?;
                let expected = [
                    Value::Utf8("capacity".into()),
                    Value::Utf8("bytes".into()),
                    Value::I32(i32::from(*count)),
                    Value::Bool(false),
                ];
                if !new.attributes().any(|attribute| {
                    attribute.name() == "NativeBufferExtentAttribute"
                        && attribute
                            .value()
                            .into_iter()
                            .map(|(_, value)| value)
                            .collect::<Vec<_>>()
                            == expected
                }) {
                    return Err(
                        "legacy byte capacity is not preserved by source-derived extent".into(),
                    );
                }
            }
        }
        Ok(())
    }
    match (old, new) {
        (Item::Fn(old), Item::Fn(new)) => method(old, new),
        (Item::Type(old), Item::Type(new)) if old.category() == TypeCategory::Interface => {
            for old in old.methods() {
                let new = new
                    .methods()
                    .find(|method| method.name() == old.name())
                    .ok_or("legacy COM method missing")?;
                method(old, new)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

struct Model<'a> {
    index: &'a Index,
    reference: Option<&'a Index>,
}

impl Model<'_> {
    fn definition(&self, name: &windows_metadata::TypeName) -> Result<Option<TypeDef<'_>>> {
        let mut types = self
            .index
            .get(&name.namespace, &name.name)
            .collect::<Vec<_>>();
        if types.is_empty()
            && let Some(reference) = self.reference
        {
            types.extend(reference.get(&name.namespace, &name.name));
        }
        match types.as_slice() {
            [] => Ok(None),
            [ty] => Ok(Some(*ty)),
            _ => Err(format!("ambiguous comparison type {name:?}").into()),
        }
    }

    fn ty(&self, ty: &Type, depth: usize) -> Result<String> {
        if depth > 64 {
            return Err("cyclic or excessive legacy type expansion".into());
        }
        Ok(match ty {
            Type::ValueName(name) | Type::ClassName(name) => {
                if let Some(definition) = self.definition(name)?
                    && definition
                        .find_attribute("NativeTypedefAttribute")
                        .is_some()
                {
                    self.ty(
                        &definition
                            .underlying_type()
                            .ok_or("legacy typedef target missing")?,
                        depth + 1,
                    )?
                } else {
                    format!("named:{}", name.name)
                }
            }
            Type::PtrMut(inner, count) => {
                format!("pointer-mut:{count}:{}", self.ty(inner, depth + 1)?)
            }
            Type::PtrConst(inner, count) => {
                format!("pointer-const:{count}:{}", self.ty(inner, depth + 1)?)
            }
            Type::RefMut(inner) => format!("reference-mut:{}", self.ty(inner, depth + 1)?),
            Type::RefConst(inner) => format!("reference-const:{}", self.ty(inner, depth + 1)?),
            Type::ArrayFixed(inner, count) => {
                format!("array:{count}:{}", self.ty(inner, depth + 1)?)
            }
            Type::Const(inner) => format!("const:{}", self.ty(inner, depth + 1)?),
            Type::Volatile(inner) => format!("volatile:{}", self.ty(inner, depth + 1)?),
            other => format!("{other:?}"),
        })
    }
}

fn attributes<'a>(
    attributes: impl Iterator<Item = windows_metadata::reader::Attribute<'a>>,
    result: &mut BTreeMap<String, String>,
) {
    let mut values = BTreeMap::<_, Vec<_>>::new();
    for attribute in attributes {
        values
            .entry(format!("attribute:{}", attribute.name()))
            .or_default()
            .push(attribute.value());
    }
    for (name, values) in values {
        result.insert(name, format!("{values:?}"));
    }
}

fn method(
    method: windows_metadata::reader::MethodDef<'_>,
    model: &Model<'_>,
) -> Result<BTreeMap<String, String>> {
    let mut result = BTreeMap::new();
    let signature = method.signature(&[]);
    result.insert(
        "signature".into(),
        format!(
            "{}({})",
            model.ty(&signature.return_type, 0)?,
            signature
                .types
                .iter()
                .map(|ty| model.ty(ty, 0))
                .collect::<Result<Vec<_>>>()?
                .join(",")
        ),
    );
    result.insert("convention".into(), method.calling_convention().into());
    if let Some(route) = method.impl_map() {
        result.insert(
            "route".into(),
            format!("{}:{}", route.import_scope().name(), route.import_name()),
        );
    }
    attributes(method.attributes(), &mut result);
    let params = method.params_by_sequence(signature.types.len())?;
    for (position, parameter) in params.params().iter().enumerate() {
        let parameter = parameter.ok_or("comparison parameter row missing")?;
        let mut contract = BTreeMap::new();
        attributes(parameter.attributes(), &mut contract);
        for (facet, value) in contract {
            result.insert(format!("parameter:{position}:{facet}"), value);
        }
        result.insert(
            format!("parameter:{position}"),
            format!("{:?}", parameter.flags()),
        );
    }
    Ok(result)
}

fn facets(item: Item<'_>, model: &Model<'_>) -> Result<BTreeMap<String, String>> {
    let mut result = BTreeMap::new();
    match item {
        Item::Fn(function) => return method(function, model),
        Item::Const(field) => {
            result.insert("kind".into(), "constant".into());
            result.insert(
                "value".into(),
                format!(
                    "{:?}:{:?}",
                    field.ty(),
                    field.constant().ok_or("legacy value missing")?.value()
                ),
            );
            attributes(field.attributes(), &mut result);
        }
        Item::Type(ty) => {
            result.insert(
                "kind".into(),
                if ty.find_attribute("NativeTypedefAttribute").is_some() {
                    "typedef".into()
                } else {
                    format!("{:?}", ty.category())
                },
            );
            attributes(ty.attributes(), &mut result);
            match ty.category() {
                TypeCategory::Enum => {
                    result.insert("underlying".into(), format!("{:?}", ty.underlying_type()));
                    for field in ty.fields().filter(|field| field.constant().is_some()) {
                        result.insert(
                            format!("member:{}", field.name()),
                            format!("{:?}", field.constant().unwrap().value()),
                        );
                    }
                }
                TypeCategory::Interface => {
                    result.insert(
                        "base".into(),
                        format!(
                            "{:?}",
                            ty.interface_impls()
                                .map(|base| base.interface(&[]))
                                .collect::<Vec<_>>()
                        ),
                    );
                    for member in ty.methods() {
                        for (facet, value) in method(member, model)? {
                            result.insert(format!("method:{}:{facet}", member.name()), value);
                        }
                    }
                }
                TypeCategory::Struct => {
                    result.insert(
                        "packing".into(),
                        format!(
                            "{:?}",
                            ty.class_layout()
                                .map(|layout| (layout.packing_size(), layout.class_size()))
                        ),
                    );
                    for field in ty.fields() {
                        result.insert(format!("field:{}", field.name()), model.ty(&field.ty(), 0)?);
                    }
                }
                _ => return Err("unsupported legacy/candidate comparison kind".into()),
            }
        }
        Item::Data(_) => return Err("unexpected imported data in frozen candidate".into()),
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unexplained_descriptors_remain_blocked() {
        assert_eq!(
            classify("attribute:UnknownContract", "<absent>", "new"),
            "unclassified"
        );
        assert_eq!(
            classify(
                "method:Read:parameter:0:attribute:UnknownContract",
                "old",
                "<absent>"
            ),
            "unclassified"
        );
        assert_eq!(classify("new-facet", "old", "new"), "unclassified");
        assert_eq!(
            classify("attribute:FlagsAttribute", "flags", "<absent>"),
            "legacy-publication-policy"
        );
    }

    #[test]
    fn legacy_capacity_requires_the_same_original_positional_binding() {
        let directory = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..\\..\\..\\target\\tool-clang\\legacy-capacity-test");
        std::fs::create_dir_all(&directory).unwrap();
        let vocabulary = directory.join("vocabulary.winmd");
        windows_rdl::reader()
            .input_text(include_str!("..\\..\\..\\..\\..\\metadata\\metadata.rdl"))
            .output(&vocabulary)
            .write()
            .unwrap();
        let old = directory.join("old.winmd");
        windows_rdl::reader().reference(&vocabulary)
            .input_text("#[win32] mod Test { #[library(\"test.dll\")] extern \"C\" fn Proof(#[size_param(1)] buffer: *mut void, length: u32, other: u32); }")
            .output(&old).write().unwrap();
        let old = Index::read(old).unwrap();
        for (name, capacity, valid) in [
            (
                "same",
                "#[Windows::Win32::Metadata::NativeBufferExtent(\"capacity\", \"bytes\", 1, false)]",
                true,
            ),
            (
                "wrong-position",
                "#[Windows::Win32::Metadata::NativeBufferExtent(\"capacity\", \"bytes\", 2, false)]",
                false,
            ),
            ("missing", "", false),
            (
                "wrong-phase",
                "#[Windows::Win32::Metadata::NativeBufferExtent(\"valid\", \"bytes\", 1, false)]",
                false,
            ),
        ] {
            let binary = directory.join(format!("{name}.winmd"));
            windows_rdl::reader().reference(&vocabulary)
                .input_text(&format!("#[win32] mod Test {{ #[library(\"test.dll\")] extern \"C\" fn Proof({capacity} buffer: *mut void, length: u32, other: u32); }}"))
                .output(&binary).write().unwrap();
            let current = Index::read(binary).unwrap();
            assert_eq!(
                verify_memory_sizes(
                    old.expect_item("Test", "Proof"),
                    current.expect_item("Test", "Proof")
                )
                .is_ok(),
                valid,
                "{name}"
            );
        }
    }
}
