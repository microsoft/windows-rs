use super::*;

/// Parsed Rust-use-style filter entry after flattening grouped paths.
#[derive(Debug, Clone)]
pub struct FilterEntry {
    /// True for `!` (exclusion) entries.
    pub exclude: bool,
    /// Flattened path segments.
    pub segments: Vec<String>,
}

/// Parse a single filter line into one or more `FilterEntry` values.
/// Grouping (`{}`) causes one input to expand into multiple entries.
pub fn parse_filter_entry(input: &str) -> Vec<FilterEntry> {
    let input = input.trim();
    if input.is_empty() {
        return Vec::new();
    }

    // Strip exclusion prefix.
    let (exclude, rest) = if let Some(rest) = input.strip_prefix('!') {
        (true, rest.trim_start())
    } else {
        (false, input)
    };

    // Parse and flatten grouped paths.
    let paths = parse_tree(rest, input);

    paths
        .into_iter()
        .map(|segments| FilterEntry { exclude, segments })
        .collect()
}

/// Parse a `::` separated tree, returning all flattened paths.
fn parse_tree(input: &str, filter: &str) -> Vec<Vec<String>> {
    let parts = split_path(input);
    let mut result = vec![Vec::new()];

    for part in parts {
        let part = part.trim();
        if part.starts_with('{') && part.ends_with('}') {
            // Expand grouped branches.
            let inner = &part[1..part.len() - 1];
            if inner.trim().is_empty() {
                for path in &mut result {
                    path.push(String::new());
                }
                continue;
            }
            let branches = split_group(inner);
            let mut expanded = Vec::new();
            for branch in branches {
                let sub_paths = parse_tree(branch.trim(), filter);
                for prefix in &result {
                    for sub in &sub_paths {
                        let mut combined = prefix.clone();
                        combined.extend(sub.iter().cloned());
                        expanded.push(combined);
                    }
                }
            }
            result = expanded;
        } else {
            // Keep legacy dotted paths compatible with the `::` grammar.
            for segment in part.split('.') {
                assert!(
                    !segment.trim().is_empty(),
                    "empty path segment in filter `{filter}`; use `Type::{{}}` for a name-only shell"
                );
                for path in &mut result {
                    path.push(segment.to_string());
                }
            }
        }
    }

    result
}

/// Split on `::` but respect `{}` nesting.
fn split_path(input: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0;
    let mut start = 0;
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                assert!(depth > 0, "unbalanced `}}` in filter: `{input}`");
                depth -= 1;
            }
            b':' if depth == 0 && i + 1 < bytes.len() && bytes[i + 1] == b':' => {
                parts.push(&input[start..i]);
                i += 2;
                start = i;
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    assert!(depth == 0, "unbalanced `{{` in filter: `{input}`");
    parts.push(&input[start..]);
    parts
}

/// Split on `,` but respect `{}` nesting.
fn split_group(input: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0;
    let mut start = 0;
    let bytes = input.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'{' => depth += 1,
            b'}' => {
                assert!(depth > 0, "unbalanced `}}` in filter group: `{input}`");
                depth -= 1;
            }
            b',' if depth == 0 => {
                parts.push(&input[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    assert!(depth == 0, "unbalanced `{{` in filter group: `{input}`");
    if parts.is_empty() || !input[start..].trim().is_empty() {
        parts.push(&input[start..]);
    }
    parts
}

/// A resolved filter item ready for use by the Filter.
#[derive(Debug, Clone)]
pub struct ResolvedFilter {
    /// True for `!` (exclusion) entries.
    pub exclude: bool,
    /// What this entry resolved to.
    pub kind: ResolvedKind,
}

#[derive(Debug, Clone)]
pub enum ResolvedKind {
    /// A namespace: include/exclude that namespace and everything nested under it.
    Namespace(String),
    /// A specific type with all members.
    Type { namespace: String, name: String },
    /// A specific type with specific member(s). An empty `members` list is an
    /// explicit name-only shell (`Ns::Type::{}`).
    Members {
        namespace: String,
        name: String,
        members: Vec<String>,
    },
}

/// Resolve parsed entries to namespaces, types, or member filters.
pub fn resolve_entries(reader: &Reader, entries: &[FilterEntry]) -> Vec<ResolvedFilter> {
    let mut resolved = Vec::new();

    for entry in entries {
        let items = resolve_one(reader, entry);
        resolved.extend(items);
    }

    resolved
}

#[track_caller]
fn resolve_one(reader: &Reader, entry: &FilterEntry) -> Vec<ResolvedFilter> {
    let segments = &entry.segments;
    assert!(!segments.is_empty(), "empty filter entry");

    let base = |kind: ResolvedKind| ResolvedFilter {
        exclude: entry.exclude,
        kind,
    };

    // Bare names are searched across namespaces.
    if segments.len() == 1 {
        let name = &segments[0];

        let mut found = false;
        let mut results = Vec::new();
        for namespace in reader.namespaces() {
            if reader.contains_name(namespace, name) {
                results.push(base(ResolvedKind::Type {
                    namespace: namespace.to_string(),
                    name: name.clone(),
                }));
                found = true;
            }
        }
        if !found {
            // Fall back to namespace or namespace-prefix matching.
            if reader.contains_namespace(name)
                || reader
                    .namespaces()
                    .any(|ns| namespace_starts_with(ns, name.as_str()))
            {
                results.push(base(ResolvedKind::Namespace(name.clone())));
            } else {
                panic!("type or namespace not found: `{name}`");
            }
        }
        return results;
    }

    // Prefer the longest namespace prefix.
    for split in (1..segments.len()).rev() {
        let ns_candidate = segments[..split].join(".");
        let rest = &segments[split..];

        if !reader.contains_namespace(&ns_candidate) {
            continue;
        }

        let type_seg = &rest[0];

        assert!(
            !type_seg.contains('*'),
            "`*` globs are no longer supported; use a bare type for the full \
             surface or `Type::{{}}` for a name-only shell (in `{}`)",
            segments.join("::")
        );

        if !reader.contains_name(&ns_candidate, type_seg) {
            continue;
        }

        let namespace = ns_candidate;
        let name = type_seg.clone();

        if rest.len() == 1 {
            return vec![base(ResolvedKind::Type { namespace, name })];
        }

        // `::{}` leaves a lone empty member segment, meaning a name-only shell.
        let members = collect_members(&rest[1..], entry);
        return vec![base(ResolvedKind::Members {
            namespace,
            name,
            members,
        })];
    }

    // Then try the whole path as a namespace.
    let full_path = segments.join(".");
    if reader.contains_namespace(&full_path) {
        return vec![base(ResolvedKind::Namespace(full_path))];
    }

    if reader
        .namespaces()
        .any(|ns| namespace_starts_with(ns, &full_path))
    {
        return vec![base(ResolvedKind::Namespace(full_path))];
    }

    // Bare member filters support flat-style `Type::Method` entries.
    let type_name = &segments[0];
    let mut results = Vec::new();
    for namespace in reader.namespaces() {
        if reader.contains_name(namespace, type_name) {
            results.push(base(ResolvedKind::Members {
                namespace: namespace.to_string(),
                name: type_name.clone(),
                members: collect_members(&segments[1..], entry),
            }));
        }
    }
    if !results.is_empty() {
        return results;
    }

    let prefix = if entry.exclude { "!" } else { "" };
    let mut message = format!(
        "could not resolve filter entry `{prefix}{}`",
        segments.join("::")
    );
    let (name, path) = segments.split_last().unwrap();

    // A known type earlier in the path may make the final name a member, not a type.
    if !reader.namespaces().any(|namespace| {
        path.iter()
            .any(|part| reader.contains_name(namespace, part))
    }) {
        let mut namespaces: Vec<_> = reader
            .namespaces()
            .filter(|namespace| reader.contains_name(namespace, name))
            .collect();
        namespaces.sort_unstable();
        if namespaces.len() == 1 {
            write!(message, "; try `{prefix}{name}`").unwrap();
        } else if !namespaces.is_empty() {
            message.push_str("; matching filters: ");
            for (index, namespace) in namespaces.iter().take(5).enumerate() {
                if index > 0 {
                    message.push_str(", ");
                }
                write!(message, "`{prefix}{namespace}.{name}`").unwrap();
            }
            if namespaces.len() > 5 {
                write!(message, " ({} more matches)", namespaces.len() - 5).unwrap();
            }
        }
    }
    panic!("{message}");
}

/// Collect member segments, dropping the lone empty segment produced by an
/// empty group (`::{}`) so it resolves to a name-only shell (empty list).
#[track_caller]
fn collect_members(member_segments: &[String], entry: &FilterEntry) -> Vec<String> {
    let segments = &entry.segments;
    let prefix = if entry.exclude { "!" } else { "" };
    assert!(
        member_segments.len() == 1,
        "chained members in filter `{prefix}{}`; try `{prefix}{}::{{{}}}`",
        segments.join("::"),
        segments[..segments.len() - member_segments.len()].join("::"),
        member_segments.join(", ")
    );
    assert!(
        !member_segments.iter().any(|m| m.contains('*')),
        "`*` globs are no longer supported; use a bare type for the full \
         surface or `Type::{{}}` for a name-only shell (in `{}`)",
        segments.join("::")
    );
    member_segments
        .iter()
        .filter(|m| !m.is_empty())
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_path() {
        let entries = parse_filter_entry("Windows::Foundation::DateTime");
        assert_eq!(entries.len(), 1);
        assert_eq!(
            entries[0].segments,
            vec!["Windows", "Foundation", "DateTime"]
        );
        assert!(!entries[0].exclude);
    }

    #[test]
    fn exclusion() {
        let entries = parse_filter_entry("!Windows::UI::Xaml");
        assert_eq!(entries.len(), 1);
        assert!(entries[0].exclude);
        assert_eq!(entries[0].segments, vec!["Windows", "UI", "Xaml"]);
    }

    #[test]
    fn group_expansion() {
        let entries = parse_filter_entry("Windows::Foundation::{DateTime, TimeSpan}");
        assert_eq!(entries.len(), 2);
        assert_eq!(
            entries[0].segments,
            vec!["Windows", "Foundation", "DateTime"]
        );
        assert_eq!(
            entries[1].segments,
            vec!["Windows", "Foundation", "TimeSpan"]
        );
    }

    #[test]
    fn method_group() {
        let entries =
            parse_filter_entry("Windows::Win32::Dxgi::IDXGIDevice::{GetAdapter, GetParent}");
        assert_eq!(entries.len(), 2);
        assert_eq!(
            entries[0].segments,
            vec!["Windows", "Win32", "Dxgi", "IDXGIDevice", "GetAdapter"]
        );
        assert_eq!(
            entries[1].segments,
            vec!["Windows", "Win32", "Dxgi", "IDXGIDevice", "GetParent"]
        );
    }

    #[test]
    fn empty_group_shell() {
        let entries = parse_filter_entry("Windows::Foundation::IStringable::{}");
        assert_eq!(entries.len(), 1);
        assert_eq!(
            entries[0].segments,
            vec!["Windows", "Foundation", "IStringable", ""]
        );
    }

    #[test]
    fn nested_groups() {
        let entries = parse_filter_entry("A::{B::{C, D}, E}");
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].segments, vec!["A", "B", "C"]);
        assert_eq!(entries[1].segments, vec!["A", "B", "D"]);
        assert_eq!(entries[2].segments, vec!["A", "E"]);
    }

    #[test]
    fn trailing_group_commas() {
        let entries = parse_filter_entry("!A::{B::{C, D, }, E::{ }, }");
        assert_eq!(entries.len(), 3);
        assert!(entries.iter().all(|entry| entry.exclude));
        assert_eq!(entries[0].segments, ["A", "B", "C"]);
        assert_eq!(entries[1].segments, ["A", "B", "D"]);
        assert_eq!(entries[2].segments, ["A", "E", ""]);
    }

    #[test]
    fn bare_name() {
        let entries = parse_filter_entry("CloseHandle");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].segments, vec!["CloseHandle"]);
    }

    #[test]
    fn empty_path_segments_are_rejected() {
        for input in [
            "IStream::",
            "::IStream",
            "IStream::::Read",
            "Windows..Win32.IStream",
            "IStream::{Read,,Write}",
            "IStream::{,Read}",
        ] {
            assert!(
                std::panic::catch_unwind(|| parse_filter_entry(input)).is_err(),
                "accepted malformed filter `{input}`"
            );
        }
    }
}

#[cfg(test)]
mod resolution_tests {
    use super::*;

    fn reader_with_types(names: &[(&str, &str)]) -> Reader {
        use windows_metadata::writer;

        let mut file = writer::File::new("filter_hints");
        let base = writer::TypeDefOrRef::TypeRef(file.TypeRef("System", "ValueType"));
        for (namespace, name) in names {
            file.TypeDef(
                namespace,
                name,
                base,
                TypeAttributes::Public | TypeAttributes::SequentialLayout,
            );
        }
        Reader::new(vec![File::new(file.into_stream()).unwrap()])
    }

    fn resolution_error(reader: &Reader, input: &str) -> String {
        let error =
            std::panic::catch_unwind(|| resolve_entries(reader, &parse_filter_entry(input)))
                .unwrap_err();
        *error.downcast::<String>().unwrap()
    }

    #[test]
    fn chained_members_are_rejected() {
        for (input, expected) in [
            ("IStream::Read::Write", "IStream::{Read, Write}"),
            ("!IStream::Read::Write", "!IStream::{Read, Write}"),
            (
                "Windows.Storage.Streams.IDataReader::ReadByte::ReadUInt32",
                "Windows::Storage::Streams::IDataReader::{ReadByte, ReadUInt32}",
            ),
        ] {
            assert_eq!(
                resolution_error(test_reader(), input),
                format!(
                    "chained members in filter `{}`; try `{expected}`",
                    input.replace('.', "::")
                ),
            );
            let resolved = resolve_entries(test_reader(), &parse_filter_entry(expected));
            assert_eq!(resolved.len(), 2);
            assert!(resolved.iter().all(|entry| matches!(
                &entry.kind, ResolvedKind::Members { members, .. } if members.len() == 1
            )));
        }
    }

    #[test]
    fn unresolved_filters_suggest_unique_bare_names() {
        let reader = reader_with_types(&[("Current", "Widget"), ("Current", "Widget")]);
        for input in ["Old.Widget", "Old::Widget", "Old::{Widget}"] {
            assert_eq!(
                resolution_error(&reader, input),
                "could not resolve filter entry `Old::Widget`; try `Widget`"
            );
        }
        assert_eq!(
            resolution_error(&reader, "!Old.Widget"),
            "could not resolve filter entry `!Old::Widget`; try `!Widget`"
        );
        let resolved = resolve_entries(&reader, &parse_filter_entry("!Widget"));
        assert_eq!(resolved.len(), 1);
        assert!(resolved[0].exclude);
        assert!(matches!(
            &resolved[0].kind,
            ResolvedKind::Type { namespace, name } if namespace == "Current" && name == "Widget"
        ));
    }

    #[test]
    fn unresolved_filters_list_bounded_qualified_matches() {
        let names = [
            ("G", "Widget"),
            ("F", "Widget"),
            ("E", "Widget"),
            ("D", "Widget"),
            ("C", "Widget"),
            ("B", "Widget"),
            ("A", "Widget"),
        ];
        for (count, expected) in [
            (2, "`!A.Widget`, `!B.Widget`"),
            (
                5,
                "`!A.Widget`, `!B.Widget`, `!C.Widget`, `!D.Widget`, `!E.Widget`",
            ),
            (
                7,
                "`!A.Widget`, `!B.Widget`, `!C.Widget`, `!D.Widget`, `!E.Widget` (2 more matches)",
            ),
        ] {
            let names = &names[names.len() - count..];
            let reader = reader_with_types(names);
            assert_eq!(
                resolution_error(&reader, "!Old.Widget"),
                format!(
                    "could not resolve filter entry `!Old::Widget`; matching filters: {expected}"
                )
            );
            assert_eq!(
                resolve_entries(&reader, &parse_filter_entry("Widget")).len(),
                count
            );
            for (namespace, name) in names {
                let resolved = resolve_entries(
                    &reader,
                    &parse_filter_entry(&format!("!{namespace}.{name}")),
                );
                assert_eq!(resolved.len(), 1);
                assert!(resolved[0].exclude);
                assert!(matches!(
                    &resolved[0].kind,
                    ResolvedKind::Type { namespace: ns, name: ty } if ns == namespace && ty == name
                ));
            }
        }
    }

    #[test]
    fn unresolved_filters_use_loaded_win32_and_winrt_names() {
        let reader = test_reader();
        for (input, namespace, name) in [
            ("Windows.Win32.Foundation.HWND", "Windows.Win32", "HWND"),
            (
                "Windows.Win32.System.Threading.GetTickCount",
                "Windows.Win32",
                "GetTickCount",
            ),
            (
                "Windows.Win32.UI.WindowsAndMessaging.WM_CLOSE",
                "Windows.Win32",
                "WM_CLOSE",
            ),
            (
                "Windows.Foundation.Collections.Uri",
                "Windows.Foundation",
                "Uri",
            ),
        ] {
            assert_eq!(
                resolution_error(reader, input),
                format!(
                    "could not resolve filter entry `{}`; try `{name}`",
                    input.replace('.', "::")
                )
            );
            let resolved = resolve_entries(reader, &parse_filter_entry(name));
            assert_eq!(resolved.len(), 1);
            assert!(matches!(
                &resolved[0].kind,
                ResolvedKind::Type { namespace: ns, name: ty } if ns == namespace && ty == name
            ));
        }
    }

    #[test]
    fn unresolved_filters_do_not_guess_missing_names_or_members() {
        let reader = reader_with_types(&[("Current", "Widget"), ("Other", "Read")]);
        for input in [
            "Old::Missing",
            "Old::Widget::Read",
            "Windows::Win32::Foundation::HWND",
        ] {
            assert_eq!(
                resolution_error(&reader, input),
                format!("could not resolve filter entry `{input}`")
            );
        }
    }

    fn test_reader() -> &'static Reader {
        use std::sync::OnceLock;
        static READER: OnceLock<Reader> = OnceLock::new();
        READER.get_or_init(|| Reader::new(default_input()))
    }

    #[test]
    fn resolve_simple_type() {
        let reader = test_reader();
        let entries = parse_filter_entry("Windows::Foundation::DateTime");
        let resolved = resolve_entries(reader, &entries);
        assert_eq!(resolved.len(), 1);
        match &resolved[0].kind {
            ResolvedKind::Type { namespace, name } => {
                assert_eq!(namespace, "Windows.Foundation");
                assert_eq!(name, "DateTime");
            }
            other => panic!("expected Type, got {other:?}"),
        }
    }

    #[test]
    fn resolve_method_group() {
        let reader = test_reader();
        let entries = parse_filter_entry("IDXGIDevice::{GetAdapter}");
        let resolved = resolve_entries(reader, &entries);
        assert_eq!(resolved.len(), 1);
        match &resolved[0].kind {
            ResolvedKind::Members {
                namespace,
                name,
                members,
            } => {
                assert_eq!(namespace, "Windows.Win32");
                assert_eq!(name, "IDXGIDevice");
                assert_eq!(members, &["GetAdapter"]);
            }
            other => panic!("expected Members, got {other:?}"),
        }
    }

    #[test]
    fn resolve_bare_name() {
        let reader = test_reader();
        let entries = parse_filter_entry("CloseHandle");
        let resolved = resolve_entries(reader, &entries);
        assert_eq!(resolved.len(), 1);
        match &resolved[0].kind {
            ResolvedKind::Type { namespace, name } => {
                assert_eq!(namespace, "Windows.Win32");
                assert_eq!(name, "CloseHandle");
            }
            other => panic!("expected Type, got {other:?}"),
        }
    }

    #[test]
    fn resolve_group_expansion() {
        let reader = test_reader();
        let entries = parse_filter_entry("Windows::Foundation::{DateTime, TimeSpan}");
        let resolved = resolve_entries(reader, &entries);
        assert_eq!(resolved.len(), 2);
        match &resolved[0].kind {
            ResolvedKind::Type { namespace, name } => {
                assert_eq!(namespace, "Windows.Foundation");
                assert_eq!(name, "DateTime");
            }
            other => panic!("expected Type, got {other:?}"),
        }
        match &resolved[1].kind {
            ResolvedKind::Type { namespace, name } => {
                assert_eq!(namespace, "Windows.Foundation");
                assert_eq!(name, "TimeSpan");
            }
            other => panic!("expected Type, got {other:?}"),
        }
    }

    #[test]
    fn resolve_empty_group_shell() {
        let reader = test_reader();
        let entries = parse_filter_entry("IDXGIDevice::{}");
        let resolved = resolve_entries(reader, &entries);
        assert_eq!(resolved.len(), 1);
        match &resolved[0].kind {
            ResolvedKind::Members {
                namespace,
                name,
                members,
            } => {
                assert_eq!(namespace, "Windows.Win32");
                assert_eq!(name, "IDXGIDevice");
                assert!(members.is_empty());
            }
            other => panic!("expected empty Members (shell), got {other:?}"),
        }
    }

    #[test]
    fn resolve_exclusion() {
        let reader = test_reader();
        let entries = parse_filter_entry("!Windows::UI::Xaml");
        let resolved = resolve_entries(reader, &entries);
        assert_eq!(resolved.len(), 1);
        assert!(resolved[0].exclude);
        match &resolved[0].kind {
            ResolvedKind::Namespace(ns) => {
                assert_eq!(ns, "Windows.UI.Xaml");
            }
            other => panic!("expected Namespace, got {other:?}"),
        }
    }

    #[test]
    fn resolve_star_panics() {
        let reader = test_reader();
        let entries = parse_filter_entry("IDXGIDevice::*");
        let result = std::panic::catch_unwind(|| resolve_entries(reader, &entries));
        assert!(result.is_err(), "`::*` should no longer resolve");
    }
}
