use super::*;

/// Restore member names rewritten by active A/W macros such as `DeleteFile`.
pub(crate) fn demacro_member_name(
    name: String,
    macro_defs: &HashMap<String, Vec<String>>,
) -> String {
    if let Some(base) = name.strip_suffix('A').or_else(|| name.strip_suffix('W'))
        && macro_defs
            .get(base)
            .is_some_and(|body| body.len() == 1 && body[0] == name)
    {
        return base.to_string();
    }
    name
}

/// Build `tag_name -> typedef_name` for public C struct/enum aliases.
pub(crate) fn build_tag_rename_map(tu: &TranslationUnit) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for child in tu.cursor().children() {
        collect_typedef_renames(child, &mut map);
    }
    map
}

/// Merge `enum _FOO { ... }; typedef DWORD FOO;` into one public enum.
///
/// The typedef supplies the backing type and signedness; the enum supplies the members.
pub(crate) fn merge_enum_typedef_idiom(
    tu: &TranslationUnit,
    tag_rename: &mut HashMap<String, String>,
) -> HashMap<String, &'static str> {
    let mut enum_tags: HashSet<String> = HashSet::new();
    let mut int_typedefs: Vec<(String, &'static str)> = Vec::new();
    collect_enum_typedef_pairs(tu.cursor(), &mut enum_tags, &mut int_typedefs);

    let mut merge = HashMap::new();
    for (name, repr) in int_typedefs {
        let tag = format!("_{name}");
        if enum_tags.contains(&tag) && !tag_rename.contains_key(&tag) {
            tag_rename.insert(tag, name.clone());
            merge.insert(name, repr);
        }
    }
    merge
}

/// Gather enum tags and integer typedefs, recursing into linkage-spec blocks.
fn collect_enum_typedef_pairs(
    cursor: Cursor,
    enum_tags: &mut HashSet<String>,
    int_typedefs: &mut Vec<(String, &'static str)>,
) {
    for child in cursor.children() {
        match child.kind() {
            CXCursor_LinkageSpec => {
                collect_enum_typedef_pairs(child, enum_tags, int_typedefs);
            }
            CXCursor_EnumDecl if child.is_definition() => {
                let name = child.name();
                if !name.is_empty() {
                    enum_tags.insert(name);
                }
            }
            CXCursor_TypedefDecl => {
                let canonical = child.typedef_underlying_type().canonical_type();
                if let Some(repr) = builtin_int_repr(canonical.kind()) {
                    int_typedefs.push((child.name(), repr));
                }
            }
            _ => {}
        }
    }
}

/// Enum `repr` string for a builtin integer type kind.
fn builtin_int_repr(kind: CXTypeKind) -> Option<&'static str> {
    Some(match kind {
        CXType_Int | CXType_Long => "i32",
        CXType_UInt | CXType_ULong => "u32",
        CXType_Short => "i16",
        CXType_UShort => "u16",
        CXType_Char_S | CXType_SChar => "i8",
        CXType_Char_U | CXType_UChar => "u8",
        CXType_LongLong => "i64",
        CXType_ULongLong => "u64",
        _ => return None,
    })
}

/// Inspect one cursor for tag->typedef renames, recursing into linkage-spec blocks.
pub(crate) fn collect_typedef_renames(cursor: Cursor, map: &mut HashMap<String, String>) {
    if cursor.kind() == CXCursor_LinkageSpec {
        for inner in cursor.children() {
            collect_typedef_renames(inner, map);
        }
        return;
    }
    if cursor.kind() != CXCursor_TypedefDecl {
        return;
    }
    let underlying = cursor.typedef_underlying_type();
    let inner = if underlying.kind() == CXType_Elaborated {
        underlying.underlying_type()
    } else {
        underlying
    };
    if inner.kind() == CXType_Record || inner.kind() == CXType_Enum {
        let tag_name = inner.ty().name();
        let typedef_name = cursor.name();
        // Rename only inline/private tags; already-public tag aliases are distinct types.
        let defines_inline = cursor.children().iter().any(|c| {
            matches!(
                c.kind(),
                CXCursor_StructDecl | CXCursor_UnionDecl | CXCursor_EnumDecl
            ) && c.is_definition()
        });
        if !tag_name.is_empty()
            && typedef_name != tag_name
            && (defines_inline || tag_name.starts_with('_') || tag_name.starts_with("tag"))
        {
            // First typedef wins, e.g. `T` before pointer alias `PT`.
            map.entry(tag_name).or_insert(typedef_name);
        }
    }
}

/// Assign synthetic flat names to nested records, keyed by tag or source location.
pub(crate) fn assign_nested_names(tu: &TranslationUnit, tag_rename: &mut HashMap<String, String>) {
    fn walk(cursor: Cursor, tag_rename: &mut HashMap<String, String>) {
        for child in cursor.children() {
            if child.kind() == CXCursor_LinkageSpec {
                walk(child, tag_rename);
            } else {
                visit_for_nested_names(child, tag_rename);
            }
        }
    }
    walk(tu.cursor(), tag_rename);
}

/// Assign nested names under one top-level named record.
pub(crate) fn visit_for_nested_names(cursor: Cursor, tag_rename: &mut HashMap<String, String>) {
    let kind = cursor.kind();
    if (kind == CXCursor_StructDecl || kind == CXCursor_UnionDecl) && cursor.is_definition() {
        let tag_name = cursor.name();
        // Anonymous top-level types have no outer name to derive from.
        if is_anonymous_name(&tag_name) {
            return;
        }
        let outer_name = tag_rename.get(&tag_name).cloned().unwrap_or(tag_name);
        assign_nested_child_names(&outer_name, cursor, tag_rename);
    }
}

/// Assign `{outer_name}_{index}` to direct nested records and recurse.
pub(crate) fn assign_nested_child_names(
    outer_name: &str,
    parent: Cursor,
    tag_rename: &mut HashMap<String, String>,
) {
    let mut index = 0usize;
    for child in parent.children() {
        let kind = child.kind();
        if (kind == CXCursor_StructDecl || kind == CXCursor_UnionDecl) && child.is_definition() {
            let synthetic = format!("{outer_name}_{index}");
            let child_name = child.name();
            if is_anonymous_name(&child_name) {
                // Anonymous records are keyed by source location.
                tag_rename.insert(child.location_id(), synthetic.clone());
            } else {
                // Named nested records override tag aliases with the synthetic name.
                tag_rename.insert(child_name, synthetic.clone());
            }
            assign_nested_child_names(&synthetic, child, tag_rename);
            index += 1;
        }
    }
}
