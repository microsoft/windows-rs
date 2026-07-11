use super::*;

/// Build a map from C struct/enum tag names to their public typedef aliases.
///
/// Scans all top-level `CXCursor_TypedefDecl` cursors in the translation unit
/// (including those inside `extern "C"` / `extern "C++"` linkage-spec blocks)
/// and records the first typedef that directly aliases each tagged struct or
/// enum as `tag_name → typedef_name`.
///
/// This handles the common C idiom:
/// ```c
/// typedef struct _TEST { int value; } TEST, *PTEST;
/// ```
/// Here `_TEST` is the internal struct tag and `TEST` is the intended public
/// name.  The map entry `"_TEST" → "TEST"` is used by the code generator to
/// replace every occurrence of `_TEST` with `TEST` in the emitted RDL.
pub(crate) fn build_tag_rename_map(tu: &TranslationUnit) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for child in tu.cursor().children() {
        collect_typedef_renames(child, &mut map);
    }
    map
}

/// Inspect a single cursor for tag→typedef rename candidates and recurse
/// into `CXCursor_LinkageSpec` blocks.
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
    // Unwrap a single elaborated wrapper if present.
    let inner = if underlying.kind() == CXType_Elaborated {
        underlying.underlying_type()
    } else {
        underlying
    };
    if inner.kind() == CXType_Record || inner.kind() == CXType_Enum {
        let tag_name = inner.ty().name();
        let typedef_name = cursor.name();
        // Collapse the tag→typedef idiom when this typedef defines the record/enum inline
        // (`typedef struct _T {...} T;`) or aliases a private tag named with the `_`/`tag`
        // prefix idiom (`struct tagVARIANT {...}; typedef struct tagVARIANT VARIANT;`). A
        // typedef aliasing an already-public tag from elsewhere (e.g. dwrite's `typedef
        // interface ID2D1SimplifiedGeometrySink IDWriteGeometrySink;`) is a distinct alias,
        // not a rename: hijacking it would orphan every reference to that interface.
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
            // First typedef wins (for `typedef struct _T {} T, *PT;`, `T` is
            // registered because it appears before the pointer typedef `PT`).
            map.entry(tag_name).or_insert(typedef_name);
        }
    }
}

/// Walk the translation unit and insert `key → synthetic_name` entries into
/// `tag_rename` for every nested struct/union type — whether named or anonymous.
///
/// For named types the tag name is used as the key (since `to_type()` resolves
/// `CXType_Record` by the declaration's spelling).  For anonymous types the
/// source location (`"file:line:col"`) is used as the key because their spelling
/// is always empty.
///
/// All nested types receive a synthetic name regardless of their C name to
/// avoid collisions (two different structs could each have an inner struct
/// called `Inner`).  Names follow the same scheme as the windows-rdl writer:
/// `{OuterName}_{index}` where `index` is the 0-based position of the nested
/// definition among **all** struct/union definitions in the parent body.
///
/// Recursion handles arbitrary nesting depth.
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

/// Visit a single top-level cursor; if it is a named struct/union definition,
/// assign synthetic names to all its nested type children.
pub(crate) fn visit_for_nested_names(cursor: Cursor, tag_rename: &mut HashMap<String, String>) {
    let kind = cursor.kind();
    if (kind == CXCursor_StructDecl || kind == CXCursor_UnionDecl) && cursor.is_definition() {
        let tag_name = cursor.name();
        // Skip anonymous top-level types – they have no outer name to derive from.
        if is_anonymous_name(&tag_name) {
            return;
        }
        let outer_name = tag_rename.get(&tag_name).cloned().unwrap_or(tag_name);
        assign_nested_child_names(&outer_name, cursor, tag_rename);
    }
}

/// For each struct/union definition that is a direct child of `parent`,
/// assign it a synthetic flat name `{outer_name}_{index}` and recurse to
/// handle deeper nesting.
///
/// `index` counts **all** nested struct/union definitions in order, matching
/// the writer's convention so that a type round-tripped through
/// clang → RDL → winmd → RDL produces names consistent with what the
/// writer would have generated.
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
                // Anonymous type: key by source location (unique per declaration site).
                tag_rename.insert(child.location_id(), synthetic.clone());
            } else {
                // Named type: key by the tag name so that to_type() can look it up,
                // overriding any pre-existing typedef alias with the synthetic name.
                tag_rename.insert(child_name, synthetic.clone());
            }
            // Recurse so that nested-nested types are also handled.
            assign_nested_child_names(&synthetic, child, tag_rename);
            index += 1;
        }
    }
}
