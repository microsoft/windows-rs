use super::*;

/// Add symbol -> DLL entries from an import library without overwriting existing ones.
pub(crate) fn extend_libraries(map: &mut HashMap<String, String>, path: &str) -> Result<(), Error> {
    let bytes = std::fs::read(path).map_err(|_| Error::new("invalid input", path, 0, 0))?;
    for import in implib::read(&bytes)? {
        map.entry(import.symbol).or_insert(import.dll);
    }
    Ok(())
}

/// Build reference name resolution, excluding the namespace currently being generated.
pub(crate) fn build_ref_map(
    reference: &metadata::reader::Index,
    exclude: &str,
) -> HashMap<String, String> {
    let mut ref_map = HashMap::new();
    for (namespace, name, _) in reference.iter() {
        if namespace == exclude {
            continue;
        }
        ref_map.insert(name.to_string(), namespace.to_string());
    }
    ref_map
}

/// Overlay in-house name resolution on the upstream reference map; in-house entries win.
pub(crate) fn build_resolution_map(
    reference: &metadata::reader::Index,
    in_house: &HashMap<String, String>,
    exclude: &str,
) -> HashMap<String, String> {
    let mut map = build_ref_map(reference, exclude);
    for (name, namespace) in in_house {
        if namespace == exclude {
            continue;
        }
        map.insert(name.clone(), namespace.clone());
    }
    map
}

/// Return the defining-header partition leaf for a declaration cursor.
///
/// Macro-expanded linkage cursors can spell at the macro definition, so expansion
/// location is used when the spelling location has no file.
pub(crate) fn header_stem_of(cursor: &Cursor) -> Option<String> {
    let file = header_path_of(cursor)?;
    let stem = header_stem_to_namespace(&file);
    if stem.is_empty() {
        // The synthetic top-level buffer parses as `.h`; it is not a real partition.
        return None;
    }
    Some(stem)
}

/// Return the full defining-header path, keeping directories for scope checks.
pub(crate) fn header_path_of(cursor: &Cursor) -> Option<String> {
    let file = cursor.file_name();
    let file = if file.is_empty() {
        cursor.expansion_file_name()
    } else {
        file
    };
    if file.is_empty() { None } else { Some(file) }
}

/// True when a normalized directory component matches a scope segment.
pub(crate) fn header_in_scope(path: &str, scope: &[String]) -> bool {
    let norm = path.replace('\\', "/").to_lowercase();
    let mut components: Vec<&str> = vec![];
    for part in norm.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                components.pop();
            }
            other => components.push(other),
        }
    }
    // The file name is not a scope component.
    let dirs = components.split_last().map_or(&[][..], |(_, dirs)| dirs);
    let want: HashSet<String> = scope.iter().map(|s| s.to_lowercase()).collect();
    dirs.iter().any(|dir| want.contains(*dir))
}

/// Collect bare nominal type names referenced by `ty`.
pub(crate) fn collect_type_refs(ty: &metadata::Type, out: &mut HashSet<String>) {
    match ty {
        metadata::Type::ClassName(name) | metadata::Type::ValueName(name) => {
            out.insert(name.name.clone());
            for generic in &name.generics {
                collect_type_refs(generic, out);
            }
        }
        metadata::Type::Array(inner)
        | metadata::Type::RefMut(inner)
        | metadata::Type::RefConst(inner)
        | metadata::Type::PtrMut(inner, _)
        | metadata::Type::PtrConst(inner, _)
        | metadata::Type::ArrayFixed(inner, _) => collect_type_refs(inner, out),
        _ => {}
    }
}

/// Collect bare type names referenced by a constant value.
pub(crate) fn collect_value_refs(value: &metadata::Value, out: &mut HashSet<String>) {
    match value {
        metadata::Value::TypeName(tn) => {
            out.insert(tn.name.clone());
        }
        metadata::Value::EnumValue(tn, inner) => {
            out.insert(tn.name.clone());
            collect_value_refs(inner, out);
        }
        _ => {}
    }
}

/// Collect field type references, including inline anonymous nested records.
pub(crate) fn collect_field_refs(fields: &[Field], out: &mut HashSet<String>) {
    for field in fields {
        collect_type_refs(&field.ty, out);
        if let Some(nested) = &field.nested {
            collect_field_refs(&nested.fields, out);
        }
    }
}

/// Collect bare names referenced by an item's signature or typed value.
pub(crate) fn item_refs(item: &Item, out: &mut HashSet<String>) {
    match item {
        Item::Fn(item) => {
            for param in &item.params {
                collect_type_refs(&param.ty, out);
            }
            collect_type_refs(&item.return_type, out);
        }
        Item::Callback(item) => {
            for param in &item.params {
                collect_type_refs(&param.ty, out);
            }
            collect_type_refs(&item.return_type, out);
        }
        Item::Interface(item) => {
            if let Some(base) = &item.base {
                collect_type_refs(base, out);
            }
            for method in &item.methods {
                for param in &method.params {
                    collect_type_refs(&param.ty, out);
                }
                collect_type_refs(&method.return_type, out);
            }
        }
        Item::Struct(item) => collect_field_refs(&item.fields, out),
        Item::Typedef(item) => collect_type_refs(&item.ty, out),
        Item::Const(item) => {
            if let Some(ty) = &item.ty {
                collect_type_refs(ty, out);
            }
            collect_value_refs(&item.value, out);
        }
        Item::PropertyKeyConst(item) => {
            out.insert(item.ty.clone());
        }
        Item::Enum(_) | Item::GuidConst(_) => {}
    }
}

/// Remove out-of-scope declarations not reachable from an in-scope declaration.
pub(crate) fn sweep_unreferenced(
    collectors: &mut BTreeMap<String, Collector>,
    scope_in: &BTreeMap<String, bool>,
) {
    // Unknown scope is treated as in-scope so the sweep only removes known out-of-scope noise.
    let in_scope = |stem: &str| scope_in.get(stem).copied().unwrap_or(true);

    let mut edges: HashMap<String, HashSet<String>> = HashMap::new();
    let mut known: HashSet<String> = HashSet::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut stack: Vec<String> = vec![];
    for (stem, collector) in collectors.iter() {
        let roots = in_scope(stem);
        for (name, item) in collector.iter() {
            known.insert(name.clone());
            let mut refs = HashSet::new();
            item_refs(item, &mut refs);
            edges.entry(name.clone()).or_default().extend(refs);
            if roots && seen.insert(name.clone()) {
                stack.push(name.clone());
            }
        }
    }

    while let Some(name) = stack.pop() {
        if let Some(refs) = edges.get(&name) {
            for r in refs {
                if known.contains(r) && seen.insert(r.clone()) {
                    stack.push(r.clone());
                }
            }
        }
    }

    for (stem, collector) in collectors.iter_mut() {
        if in_scope(stem) {
            continue;
        }
        collector.retain(|name| seen.contains(name));
    }
}

/// Reduce a header path to its upper-cased partition leaf name.
pub(crate) fn header_stem_to_namespace(file: &str) -> String {
    let base = file.rsplit(['/', '\\']).next().unwrap_or(file);
    let stem = base.rsplit_once('.').map_or(base, |(s, _)| s);
    // Dotted WinRT interop headers still map to one flat partition segment.
    let stem: String = stem.chars().filter(|c| *c != '.').collect();
    let mut chars = stem.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// True when `file` ends with `filter` on a path-segment boundary.
pub(crate) fn matches_filter(file: &str, filter: &str) -> bool {
    if filter.is_empty() {
        return false;
    }
    let file = file.replace('\\', "/");
    let filter = filter.replace('\\', "/");
    file.ends_with(filter.as_str())
        && (file.len() == filter.len() || file.as_bytes()[file.len() - filter.len() - 1] == b'/')
}
