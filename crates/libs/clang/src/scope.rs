use super::*;

/// Reads an import library and extends `map` with its symbol -> DLL entries without
/// overwriting existing mappings.
pub(crate) fn extend_libraries(map: &mut HashMap<String, String>, path: &str) -> Result<(), Error> {
    let bytes = std::fs::read(path).map_err(|_| Error::new("invalid input", path, 0, 0))?;
    for import in implib::read(&bytes)? {
        map.entry(import.symbol).or_insert(import.dll);
    }
    Ok(())
}

/// Builds the name -> namespace resolution map from the reference metadata, excluding
/// the namespace under construction so its own types are re-emitted from source rather
/// than resolving to the (possibly stale or upstream) reference copy. Types from other
/// namespaces stay in the map so cross-namespace dependencies (`Foundation::HRESULT`,
/// `Gdi::HMONITOR`, ...) resolve as qualified references.
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

/// Overlays the in-house `name -> namespace` table (built from what each partition
/// emits) onto the upstream reference map so cross-namespace references resolve
/// against types we build ourselves, falling back to the upstream `reference` only
/// for names not yet built in-house. In-house entries win, and the namespace under
/// construction is excluded from both so its own types re-emit from source. This is
/// the resolution half of becoming self-sustaining: as in-house coverage grows the
/// upstream fallback shrinks toward zero, the prerequisite for dropping `--reference`.
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

/// Maps a clang declaration cursor to the namespace of its **defining header**,
/// e.g. a typedef declared in `wingdi.h` with `root = "Windows.Win32"` becomes
/// `Windows.Win32.Wingdi`. This is the per-header partition key: intrinsic to the
/// source (clang's cursor location), total over all declarations, and stable across
/// SDK versions - unlike the hand-curated namespaces of `win32metadata`.
///
/// Returns the defining-header partition leaf (file stem) for `cursor`, e.g.
/// `Windef` for a declaration written in `windef.h`. Used to route each emitted
/// declaration to its per-header output file in the flat `Windows.Win32` namespace.
///
/// `CXCursor_LinkageSpec` cursors produced by linkage macros (`STDAPI`/`WINAPI`)
/// report their spelling location at the macro definition site (an unfiltered
/// header such as `winnt.h`); the *expansion* location - where the macro was
/// invoked - is the real API header, so it is preferred when present.
///
/// Returns `None` for cursors with no associated file (builtins, the predefined
/// translation-unit buffer).
pub(crate) fn header_stem_of(cursor: &Cursor) -> Option<String> {
    let file = header_path_of(cursor)?;
    let stem = header_stem_to_namespace(&file);
    if stem.is_empty() {
        // The synthetic top-level translation-unit buffer is parsed as `.h` (an empty
        // stem); anything clang attributes to it (predefined/builtin artifacts) is not
        // a real header declaration and must not create an empty partition.
        return None;
    }
    Some(stem)
}

/// Returns the full defining-header path for `cursor` (the spelling location, falling
/// back to the macro expansion location), or `None` for cursors with no associated file.
/// Unlike [`header_stem_of`] this keeps the directory, so the scope predicate can tell an
/// SDK `um`/`shared` header from a C-runtime (`ucrt`) or MSVC-toolset (`include`) one.
pub(crate) fn header_path_of(cursor: &Cursor) -> Option<String> {
    let file = cursor.file_name();
    let file = if file.is_empty() {
        cursor.expansion_file_name()
    } else {
        file
    };
    if file.is_empty() { None } else { Some(file) }
}

/// Whether a defining-header `path` is in scope: one of its directory components (after
/// collapsing `.`/`..`) equals a `scope` segment, e.g. `um`/`shared` matches
/// `.../Include/10.0.26100.0/um/winuser.h` (and the nested `.../um/gl/gl.h`) but not a
/// C-runtime header under `.../ucrt/`. Matching is case-insensitive, separator-agnostic,
/// and component-based so a sibling directory reached via `..` cannot match by substring.
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
    // The final component is the file name itself; only the directory components define
    // scope.
    let dirs = components.split_last().map_or(&[][..], |(_, dirs)| dirs);
    let want: HashSet<String> = scope.iter().map(|s| s.to_lowercase()).collect();
    dirs.iter().any(|dir| want.contains(*dir))
}

/// Collects the bare names of every nominal type (`ClassName`/`ValueName`, including
/// generic arguments) referenced by `ty`, descending through pointer/array/reference
/// wrappers. Primitive and built-in types contribute no name.
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

/// Collects the bare names of every nominal type a constant's value references: a
/// `TypeName` value names a type directly, and an `EnumValue` names its enum type (and,
/// recursively, its inner value). Scalar/string values contribute no name.
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

/// Collects the type references of a record's fields, descending into anonymous nested
/// records (`Field::nested`). A nested anonymous member carries its real content in
/// `nested` with a `Void` `ty`, so walking only `field.ty` would miss every type the
/// nested record references.
pub(crate) fn collect_field_refs(fields: &[Field], out: &mut HashSet<String>) {
    for field in fields {
        collect_type_refs(&field.ty, out);
        if let Some(nested) = &field.nested {
            collect_field_refs(&nested.fields, out);
        }
    }
}

/// Collects the bare names of every type `item` references through its signature: function
/// and callback parameters/returns, interface base and method signatures, struct/union
/// field types (including anonymous nested records), typedef targets, and typed-constant
/// values. Enums and GUID constants reference only primitives (or the always-in-scope
/// `GUID`), so they contribute no edges.
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
        Item::Const(item) => collect_value_refs(&item.value, out),
        Item::PropertyKeyConst(item) => {
            out.insert(item.ty.clone());
        }
        Item::Enum(_) | Item::GuidConst(_) => {}
    }
}

/// Reachability-by-reference sweep. Roots are every declaration in an in-scope partition;
/// the type-reference graph is then walked to mark every transitively referenced name.
/// Each out-of-scope partition keeps only its marked declarations, so the C-runtime and
/// compiler-toolset closure that no in-scope API references falls away while genuine
/// cross-over types (referenced from in-scope code) survive without any allowlist.
pub(crate) fn sweep_unreferenced(
    collectors: &mut BTreeMap<String, Collector>,
    scope_in: &BTreeMap<String, bool>,
) {
    // A partition with no recorded scope (e.g. the hand-authored seed, or a stem the
    // bucketing never classified) is treated as in-scope so the sweep can only ever
    // *remove* known out-of-scope noise, never in-scope surface.
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
            edges.insert(name.clone(), refs);
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

/// Reduces a header path to its partition leaf name: the file stem (basename minus
/// the final extension) with its first character upper-cased, e.g.
/// `C:\sdk\um\wingdi.h` -> `Wingdi`, `shared.inl` -> `Shared`.
pub(crate) fn header_stem_to_namespace(file: &str) -> String {
    let base = file.rsplit(['/', '\\']).next().unwrap_or(file);
    let stem = base.rsplit_once('.').map_or(base, |(s, _)| s);
    // A header whose own name is dotted (the WinRT interop headers, e.g.
    // `Windows.Devices.Display.Core.Interop.h`) must collapse to a single flat
    // partition segment. The flat Win32 surface ignores header namespacing, so the
    // leftover dots must not survive to spawn a nested `Windows::Devices::...` module
    // tree under `Win32`.
    let stem: String = stem.chars().filter(|c| *c != '.').collect();
    let mut chars = stem.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Returns `true` if `file` ends with `filter` and the match falls on a
/// clean path-segment boundary.
///
/// Both paths are normalized to forward slashes before comparison, so this
/// works on both Windows and POSIX.  `filter("api1.h")` matches
/// `/path/to/api1.h` but not `/path/to/myapi1.h`.
pub(crate) fn matches_filter(file: &str, filter: &str) -> bool {
    if filter.is_empty() {
        return false;
    }
    let file = file.replace('\\', "/");
    let filter = filter.replace('\\', "/");
    file.ends_with(filter.as_str())
        && (file.len() == filter.len() || file.as_bytes()[file.len() - filter.len() - 1] == b'/')
}
