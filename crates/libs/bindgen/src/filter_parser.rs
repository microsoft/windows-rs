use super::*;

/// Filter entry parser — Rust `use`-style path syntax.
///
/// Grammar:
/// ```text
/// entry   = ["!"] ["??" | "?"] tree
/// tree    = segment { "::" segment }
/// segment = "{" tree { "," tree } "}" | "**" | "*" | ident
/// ident   = [A-Za-z0-9_]+
/// ```
///
/// Flattens grouped paths into a list of fully-qualified entries:
///   `A::B::{C, D::E}` → `["A::B::C", "A::B::D::E"]`
///
/// A parsed filter entry after flattening.
#[derive(Debug, Clone)]
pub struct FilterEntry {
    /// True for `!` (exclusion) entries.
    pub exclude: bool,
    /// True for `?` (trait-only) or `??` (skeleton-only) entries.
    pub trait_only: bool,
    /// True for `??` (skeleton-only) entries.
    pub skeleton_only: bool,
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

    let (skeleton_only, trait_only, rest) = if let Some(rest) = rest.strip_prefix("??") {
        (true, true, rest)
    } else if let Some(rest) = rest.strip_prefix('?') {
        (false, true, rest)
    } else {
        (false, false, rest)
    };

    // Parse the path tree and flatten
    let paths = parse_tree(rest);

    paths
        .into_iter()
        .map(|segments| FilterEntry {
            exclude,
            trait_only,
            skeleton_only,
            segments,
        })
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
            // Simple segment: append to all current paths
            for path in &mut result {
                path.push(part.to_string());
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
            b'}' => depth -= 1,
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
            b'}' => depth -= 1,
            b',' if depth == 0 => {
                parts.push(&input[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(&input[start..]);
    parts
}

/// A resolved filter item ready for use by the Filter.
#[derive(Debug, Clone)]
pub struct ResolvedFilter {
    /// True for `!` (exclusion) entries.
    pub exclude: bool,
    /// True for `?` (trait-only) or `??` (skeleton-only) entries.
    pub trait_only: bool,
    /// True for `??` (skeleton-only) entries.
    pub skeleton_only: bool,
    /// What this entry resolved to.
    pub kind: ResolvedKind,
}

#[derive(Debug, Clone)]
pub enum ResolvedKind {
    /// A namespace (possibly with recursive glob): include/exclude all types in it.
    Namespace(String),
    /// A specific type with all members.
    Type { namespace: String, name: String },
    /// A specific type with specific member(s).
    Members {
        namespace: String,
        name: String,
        members: Vec<String>,
    },
    /// A glob within a namespace: `Ns::Prefix*`
    NameGlob { namespace: String, prefix: String },
}

/// Resolve parsed filter entries against the metadata reader.
///
/// Each `FilterEntry` is resolved by joining segments with `.` to find the
/// longest namespace match, then checking if the next segment is a type, and
/// remaining segments are members (methods/variants).
///
/// Special cases:
/// - `**` at the end means recursive namespace inclusion
/// - `*` at the end of a namespace means all types in that namespace
/// - `*` after a type means all members
/// - A single bare ident (no `::`) is searched across all namespaces
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
        trait_only: entry.trait_only,
        skeleton_only: entry.skeleton_only,
        kind,
    };

    // Single segment: bare name (e.g. "CloseHandle") — search all namespaces
    if segments.len() == 1 {
        let name = &segments[0];
        if name == "**" || name == "*" {
            // Everything
            return reader
                .keys()
                .map(|ns| base(ResolvedKind::Namespace(ns.to_string())))
                .collect();
        }
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
            // Maybe it's a namespace?
            if reader.contains_key(name.as_str()) {
                results.push(base(ResolvedKind::Namespace(name.clone())));
            } else {
                panic!("type or namespace not found: `{name}`");
            }
        }
        return results;
    }

    // Check if last segment is `**` (recursive glob)
    if segments.last().is_some_and(|s| s == "**") {
        let ns_prefix = segments[..segments.len() - 1].join(".");
        // Validate namespace prefix exists
        let any_match = reader
            .keys()
            .any(|ns| namespace_starts_with(ns, &ns_prefix));
        assert!(
            any_match,
            "no namespaces found matching `{}`",
            segments.join("::")
        );
        return vec![base(ResolvedKind::Namespace(ns_prefix))];
    }

    // Try progressively longer namespace prefixes.
    // Join segments with "." and check reader.
    for split in (1..segments.len()).rev() {
        let ns_candidate = segments[..split].join(".");
        let rest = &segments[split..];

        if !reader.contains_key(ns_candidate.as_str()) {
            continue;
        }

        // Found namespace. Next segment should be a type or glob.
        let type_seg = &rest[0];

        // Namespace-level glob: `Ns::*`
        if type_seg == "*" {
            assert!(
                rest.len() == 1,
                "`*` must be the last segment in `{}`",
                segments.join("::")
            );
            return vec![base(ResolvedKind::Namespace(ns_candidate))];
        }

        // Name glob: `Ns::Prefix*`
        if let Some(prefix) = type_seg.strip_suffix('*') {
            assert!(
                rest.len() == 1,
                "glob must be the last segment in `{}`",
                segments.join("::")
            );
            return vec![base(ResolvedKind::NameGlob {
                namespace: ns_candidate,
                prefix: prefix.to_string(),
            })];
        }

        // Look up the type
        let ns_map = reader.get(ns_candidate.as_str());
        if ns_map.is_none() || ns_map.unwrap().get(type_seg.as_str()).is_none() {
            // Type not found in this namespace — try shorter namespace
            continue;
        }

        let namespace = ns_candidate;
        let name = type_seg.clone();

        if rest.len() == 1 {
            // Just a type, no members
            return vec![base(ResolvedKind::Type { namespace, name })];
        }

        // Remaining segments are members
        let member_segments = &rest[1..];
        if member_segments.len() == 1 && member_segments[0] == "*" {
            // Type::* — all members
            return vec![base(ResolvedKind::Type { namespace, name })];
        }

        // Specific members
        let members: Vec<String> = member_segments.to_vec();
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

    panic!("could not resolve filter entry `{}`", segments.join("::"));
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
    fn skeleton_prefix() {
        let entries = parse_filter_entry("??Windows::Win32::Com::IStream");
        assert_eq!(entries.len(), 1);
        assert!(entries[0].skeleton_only);
        assert!(entries[0].trait_only);
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
    fn glob() {
        let entries = parse_filter_entry("Windows::Foundation::*");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].segments, vec!["Windows", "Foundation", "*"]);
    }

    #[test]
    fn recursive_glob() {
        let entries = parse_filter_entry("Windows::**");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].segments, vec!["Windows", "**"]);
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

    #[test]
    fn star_methods() {
        let entries = parse_filter_entry("Windows::Win32::Dxgi::IDXGIDevice::*");
        assert_eq!(entries.len(), 1);
        assert_eq!(
            entries[0].segments,
            vec!["Windows", "Win32", "Dxgi", "IDXGIDevice", "*"]
        );
    }
}

#[cfg(test)]
mod resolution_tests {
    use super::*;

    fn test_reader() -> &'static Reader {
        use std::sync::OnceLock;
        static READER: OnceLock<Reader> = OnceLock::new();
        READER.get_or_init(|| Reader::new(expand_input(&["default"])))
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
        let entries =
            parse_filter_entry("Windows::Win32::Graphics::Dxgi::IDXGIDevice::{GetAdapter}");
        let resolved = resolve_entries(reader, &entries);
        assert_eq!(resolved.len(), 1);
        match &resolved[0].kind {
            ResolvedKind::Members {
                namespace,
                name,
                members,
            } => {
                assert_eq!(namespace, "Windows.Win32.Graphics.Dxgi");
                assert_eq!(name, "IDXGIDevice");
                assert_eq!(members, &["GetAdapter"]);
            }
            other => panic!("expected Members, got {other:?}"),
        }
    }

    #[test]
    fn resolve_recursive_glob() {
        let reader = test_reader();
        let entries = parse_filter_entry("Windows::Foundation::**");
        let resolved = resolve_entries(reader, &entries);
        assert_eq!(resolved.len(), 1);
        match &resolved[0].kind {
            ResolvedKind::Namespace(ns) => {
                assert_eq!(ns, "Windows.Foundation");
            }
            other => panic!("expected Namespace, got {other:?}"),
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
                assert_eq!(namespace, "Windows.Win32.Foundation");
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
    fn resolve_type_star() {
        let reader = test_reader();
        let entries = parse_filter_entry("Windows::Win32::Graphics::Dxgi::IDXGIDevice::*");
        let resolved = resolve_entries(reader, &entries);
        assert_eq!(resolved.len(), 1);
        match &resolved[0].kind {
            ResolvedKind::Type { namespace, name } => {
                assert_eq!(namespace, "Windows.Win32.Graphics.Dxgi");
                assert_eq!(name, "IDXGIDevice");
            }
            other => panic!("expected Type, got {other:?}"),
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
    fn resolve_name_glob() {
        let reader = test_reader();
        let entries = parse_filter_entry("Windows::Foundation::Async*");
        let resolved = resolve_entries(reader, &entries);
        assert_eq!(resolved.len(), 1);
        match &resolved[0].kind {
            ResolvedKind::NameGlob { namespace, prefix } => {
                assert_eq!(namespace, "Windows.Foundation");
                assert_eq!(prefix, "Async");
            }
            other => panic!("expected NameGlob, got {other:?}"),
        }
    }
}
