use super::*;

/// Filter entry parser - Rust `use`-style path syntax.
///
/// Grammar:
/// ```text
/// entry   = ["!"] tree
/// tree    = segment { "::" segment }
/// segment = "{" [ tree { "," tree } ] "}" | ident
/// ident   = [A-Za-z0-9_]+
/// ```
///
/// Specificity drives granularity (inspired by Rust's `use`): a bare mention
/// (`Ns::Type`) includes the whole type; `Ns::Type::{}` is a name-only shell;
/// `Ns::Type::{a, b}` includes only the named methods.
///
/// Flattens grouped paths into a list of fully-qualified entries:
///   `A::B::{C, D::E}` -> `["A::B::C", "A::B::D::E"]`
///
/// A parsed filter entry after flattening.
#[derive(Debug, Clone)]
pub struct FilterEntry {
    /// True for `!` (exclusion) entries.
    pub exclude: bool,
    /// Flattened path segments: ["Windows", "Win32", "Graphics", "Dxgi", "IDXGIDevice", "GetAdapter"]
    pub segments: Vec<String>,
}

/// Parse a single filter line into one or more `FilterEntry` values.
/// Grouping (`{}`) causes one input to expand into multiple entries.
pub fn parse_filter_entry(input: &str) -> Vec<FilterEntry> {
    let input = input.trim();
    if input.is_empty() {
        return Vec::new();
    }

    // Strip prefixes
    let (exclude, rest) = if let Some(rest) = input.strip_prefix('!') {
        (true, rest.trim_start())
    } else {
        (false, input)
    };

    // Parse the path tree and flatten
    let paths = parse_tree(rest);

    paths
        .into_iter()
        .map(|segments| FilterEntry { exclude, segments })
        .collect()
}

/// Parse a `::` separated tree, returning all flattened paths.
fn parse_tree(input: &str) -> Vec<Vec<String>> {
    let parts = split_path(input);
    let mut result = vec![Vec::new()];

    for part in parts {
        let part = part.trim();
        if part.starts_with('{') && part.ends_with('}') {
            // Group: expand combinatorially
            let inner = &part[1..part.len() - 1];
            let branches = split_group(inner);
            let mut expanded = Vec::new();
            for branch in branches {
                let sub_paths = parse_tree(branch.trim());
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
            // Simple segment: append to all current paths.
            // If it contains dots, split further (legacy dot-path compat).
            if part.contains('.') {
                for dot_part in part.split('.') {
                    for path in &mut result {
                        path.push(dot_part.to_string());
                    }
                }
            } else {
                for path in &mut result {
                    path.push(part.to_string());
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
    parts.push(&input[start..]);
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

/// Resolve parsed filter entries against the metadata reader.
///
/// Each `FilterEntry` is resolved by joining segments with `.` to find the
/// longest namespace match, then checking if the next segment is a type, and
/// remaining segments are members (methods/variants).
///
/// Special cases:
/// - `Ns::Type::{}` (empty group) means a name-only shell (no members)
/// - A single bare ident (no `::`) is searched across all namespaces
///
/// A bare namespace (e.g. `Windows` or `Windows::Foundation`) is recursive: it
/// matches that namespace and every namespace nested beneath it.
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

    // Single segment: bare name (e.g. "CloseHandle") - search all namespaces
    if segments.len() == 1 {
        let name = &segments[0];
        // Search all namespaces for this name
        let mut found = false;
        let mut results = Vec::new();
        for (namespace, types) in reader.iter() {
            if types.get(name.as_str()).is_some() {
                results.push(base(ResolvedKind::Type {
                    namespace: namespace.to_string(),
                    name: name.clone(),
                }));
                found = true;
            }
        }
        if !found {
            // Maybe it's a namespace (exact or prefix)?
            if reader.contains_key(name.as_str())
                || reader
                    .keys()
                    .any(|ns| namespace_starts_with(ns, name.as_str()))
            {
                results.push(base(ResolvedKind::Namespace(name.clone())));
            } else {
                panic!("type or namespace not found: `{name}`");
            }
        }
        return results;
    }

    // Try progressively longer namespace prefixes.
    // Join segments with "." and check reader.
    for split in (1..segments.len()).rev() {
        let ns_candidate = segments[..split].join(".");
        let rest = &segments[split..];

        if !reader.contains_key(ns_candidate.as_str()) {
            continue;
        }

        // Found namespace. Next segment should be a type.
        let type_seg = &rest[0];

        assert!(
            !type_seg.contains('*'),
            "`*` globs are no longer supported; use a bare type for the full \
             surface or `Type::{{}}` for a name-only shell (in `{}`)",
            segments.join("::")
        );

        // Look up the type
        let ns_map = reader.get(ns_candidate.as_str());
        if ns_map.is_none() || ns_map.unwrap().get(type_seg.as_str()).is_none() {
            // Type not found in this namespace - try shorter namespace
            continue;
        }

        let namespace = ns_candidate;
        let name = type_seg.clone();

        if rest.len() == 1 {
            // Just a type, no members
            return vec![base(ResolvedKind::Type { namespace, name })];
        }

        // Remaining segments are members; an empty group (`::{}`) leaves a lone
        // empty segment and means a name-only shell.
        let members = collect_members(&rest[1..], segments);
        return vec![base(ResolvedKind::Members {
            namespace,
            name,
            members,
        })];
    }

    // If nothing matched as namespace + type, try as a namespace path
    let full_path = segments.join(".");
    if reader.contains_key(full_path.as_str()) {
        return vec![base(ResolvedKind::Namespace(full_path))];
    }

    // Check if it's a namespace prefix
    if reader
        .keys()
        .any(|ns| namespace_starts_with(ns, &full_path))
    {
        return vec![base(ResolvedKind::Namespace(full_path))];
    }

    // Bare type with members: no namespace prefix matched, so treat the first
    // segment as a type searched across all namespaces and the rest as members.
    // This lets `--flat` filters drop namespaces entirely (e.g. `IAgileReference::Resolve`
    // instead of `Windows::Win32::System::WinRT::IAgileReference::Resolve`), resolving
    // identically against both namespaced and flat metadata.
    let type_name = &segments[0];
    let members = collect_members(&segments[1..], segments);
    let mut results = Vec::new();
    for (namespace, types) in reader.iter() {
        if types.get(type_name.as_str()).is_some() {
            results.push(base(ResolvedKind::Members {
                namespace: namespace.to_string(),
                name: type_name.clone(),
                members: members.clone(),
            }));
        }
    }
    if !results.is_empty() {
        return results;
    }

    panic!("could not resolve filter entry `{}`", segments.join("::"));
}

/// Collect member segments, dropping the lone empty segment produced by an
/// empty group (`::{}`) so it resolves to a name-only shell (empty list).
#[track_caller]
fn collect_members(member_segments: &[String], segments: &[String]) -> Vec<String> {
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
    fn bare_name() {
        let entries = parse_filter_entry("CloseHandle");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].segments, vec!["CloseHandle"]);
    }
}

#[cfg(test)]
mod resolution_tests {
    use super::*;

    fn test_reader() -> &'static Reader {
        use std::sync::OnceLock;
        static READER: OnceLock<Reader> = OnceLock::new();
        READER.get_or_init(|| Reader::new(expand_input(&["default"]), false))
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
