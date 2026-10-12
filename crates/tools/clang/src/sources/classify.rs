use super::*;

type Observation<'a> = (usize, &'a Declaration);

pub fn name_index(capture: &Capture) -> BTreeMap<&str, Vec<Observation<'_>>> {
    let mut names = BTreeMap::<_, Vec<_>>::new();
    for (id, item) in capture.declarations.iter().enumerate() {
        if !item.name.is_empty()
            && !matches!(
                item.kind.as_str(),
                "FieldDecl" | "ParmDecl" | "CXXMethod" | "Namespace" | "Constructor" | "Destructor"
            )
        {
            names
                .entry(item.name.as_str())
                .or_default()
                .push((id, item));
        }
    }
    names
}

pub fn group(
    kind: &str,
    header: &str,
    status: &str,
    candidates: &[Observation<'_>],
    named: &[Observation<'_>],
) -> &'static str {
    if status == "blocked-profile" {
        return "unavailable-profile";
    }
    if status == "ambiguous-candidate" {
        if tag_aliases(candidates) {
            return "tag-and-typedef";
        }
        if candidates
            .iter()
            .all(|(_, item)| item.kind == "FunctionDecl")
        {
            return if candidates.iter().any(|(_, item)| item.inline)
                && candidates.iter().any(|(_, item)| !item.inline)
            {
                "inline-and-noninline-callables"
            } else {
                "distinct-callables"
            };
        }
        if candidates
            .iter()
            .any(|(_, item)| item.kind == "macro-definition")
            && candidates.iter().any(|(_, item)| item.kind == "VarDecl")
        {
            return "data-and-macro";
        }
        return "distinct-native-identities";
    }
    let same_header = named
        .iter()
        .filter(|(_, item)| item.header.file_name().is_some_and(|name| name == header))
        .collect::<Vec<_>>();
    if same_header
        .iter()
        .any(|(_, item)| item.scope == "dependency" && compatible(kind, &item.kind))
    {
        return "dependency-name";
    }
    if same_header
        .iter()
        .any(|(_, item)| item.scope == "excluded" && compatible(kind, &item.kind))
    {
        return "excluded-name";
    }
    if kind == "constant" && same_header.iter().any(|(_, item)| item.kind == "ClassDecl") {
        return "class-name-value";
    }
    if kind == "function"
        && same_header
            .iter()
            .any(|(_, item)| item.kind == "macro-definition")
    {
        return "macro-name-callable";
    }
    if !same_header.is_empty() {
        return "same-header-other-kind";
    }
    if !named.is_empty() {
        return "other-header-name";
    }
    if status == "header-not-observed" {
        "unobserved-header-no-name"
    } else {
        "no-name-observation"
    }
}

fn tag_aliases(candidates: &[Observation<'_>]) -> bool {
    let mut tags = BTreeSet::new();
    let mut aliases = Vec::new();
    for (_, item) in candidates {
        match item.kind.as_str() {
            "StructDecl" | "UnionDecl" | "ClassDecl" | "EnumDecl"
                if !item.canonical_usr.is_empty() =>
            {
                tags.insert(item.canonical_usr.as_str());
            }
            "TypedefDecl" | "TypeAliasDecl" if !item.alias_usr.is_empty() => {
                aliases.push(item.alias_usr.as_str());
            }
            _ => return false,
        }
    }
    tags.len() == 1 && !aliases.is_empty() && aliases.iter().all(|alias| tags.contains(alias))
}

pub fn package(group: &str, kind: &str) -> &'static str {
    match group {
        "tag-and-typedef" => "C2",
        "inline-and-noninline-callables" | "distinct-callables" => "C5",
        "data-and-macro" | "macro-name-callable" => "C8",
        "class-name-value" => "C4",
        "dependency-name" | "excluded-name" | "other-header-name" | "unobserved-header-no-name" => {
            "C10"
        }
        "unavailable-profile" => "C0",
        _ => match kind {
            "struct" | "union" => "C3",
            "enum" | "constant" => "C4",
            "function" | "callback" | "delegate" => "C5",
            "interface" | "class" => "C6",
            "typedef" => "C2",
            _ => "C0",
        },
    }
}

pub fn coverage_status(statuses: impl Iterator<Item = &'static str>) -> &'static str {
    let mut candidate = false;
    let mut observed = false;
    for status in statuses {
        match status {
            "blocked-profile" => return "unavailable-evidence",
            "single-identity-candidate" | "ambiguous-candidate" => candidate = true,
            "no-candidate" => observed = true,
            "header-not-observed" => {}
            _ => panic!("unexpected source mapping status"),
        }
    }
    if candidate {
        "candidate-observed"
    } else if observed {
        "observed-no-candidate"
    } else {
        "unobserved-header"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn declaration(kind: &str, usr: &str, alias: &str) -> Declaration {
        Declaration {
            scope: "root",
            kind: kind.into(),
            name: "Example".into(),
            qualified: "Example".into(),
            usr: usr.into(),
            canonical_usr: usr.into(),
            parent_usr: String::new(),
            alias_usr: alias.into(),
            header: "example.h".into(),
            line: 1,
            column: 1,
            offset: 0,
            spelling_header: String::new(),
            spelling_offset: 0,
            written_type: String::new(),
            canonical_type: String::new(),
            definition: false,
            inline: false,
            function_macro: false,
        }
    }

    #[test]
    fn tag_aliases_require_compiler_target_identity() {
        let tag = declaration("StructDecl", "tag", "");
        let alias = declaration("TypedefDecl", "alias", "tag");
        let forward = declaration("StructDecl", "tag", "");
        assert!(tag_aliases(&[(0, &tag), (1, &alias), (2, &forward)]));
        let unrelated = declaration("TypedefDecl", "alias", "other");
        assert!(!tag_aliases(&[(0, &tag), (1, &unrelated)]));
        let other = declaration("StructDecl", "other", "");
        assert!(!tag_aliases(&[(0, &tag), (1, &alias), (2, &other)]));
        let missing = declaration("TypedefDecl", "alias", "");
        assert!(!tag_aliases(&[(0, &tag), (1, &missing)]));
    }

    #[test]
    fn groups_preserve_scope_and_kind_boundaries() {
        let mut item = declaration("TypedefDecl", "alias", "");
        item.scope = "dependency";
        assert_eq!(
            group("typedef", "example.h", "no-candidate", &[], &[(0, &item)]),
            "dependency-name"
        );
        assert_eq!(
            group("typedef", "other.h", "no-candidate", &[], &[(0, &item)]),
            "other-header-name"
        );
        assert_eq!(
            group(
                "typedef",
                "example.h",
                "blocked-profile",
                &[],
                &[(0, &item)]
            ),
            "unavailable-profile"
        );
        item.scope = "excluded";
        assert_eq!(
            group("typedef", "example.h", "no-candidate", &[], &[(0, &item)]),
            "excluded-name"
        );
        let item = declaration("ClassDecl", "class", "");
        assert_eq!(
            group("constant", "example.h", "no-candidate", &[], &[(0, &item)]),
            "class-name-value"
        );
        assert_eq!(
            group("struct", "example.h", "no-candidate", &[], &[]),
            "no-name-observation"
        );
        assert_eq!(
            group("struct", "example.h", "header-not-observed", &[], &[]),
            "unobserved-header-no-name"
        );
    }

    #[test]
    fn callable_and_macro_groups_do_not_choose_a_winner() {
        let first = declaration("FunctionDecl", "first", "");
        let mut second = declaration("FunctionDecl", "second", "");
        second.inline = true;
        assert_eq!(
            group(
                "function",
                "example.h",
                "ambiguous-candidate",
                &[(0, &first), (1, &second)],
                &[]
            ),
            "inline-and-noninline-callables"
        );
        second.inline = false;
        assert_eq!(
            group(
                "function",
                "example.h",
                "ambiguous-candidate",
                &[(0, &first), (1, &second)],
                &[]
            ),
            "distinct-callables"
        );
        let data = declaration("VarDecl", "data", "");
        let macro_definition = declaration("macro-definition", "macro", "");
        assert_eq!(
            group(
                "constant",
                "example.h",
                "ambiguous-candidate",
                &[(0, &data), (1, &macro_definition)],
                &[]
            ),
            "data-and-macro"
        );
        assert_eq!(
            group(
                "function",
                "example.h",
                "no-candidate",
                &[],
                &[(0, &macro_definition)]
            ),
            "macro-name-callable"
        );
    }

    #[test]
    fn coverage_does_not_call_offreg_missing_in_the_kernel_profile() {
        assert_eq!(
            coverage_status(["header-not-observed", "single-identity-candidate"].into_iter()),
            "candidate-observed"
        );
        assert_eq!(
            coverage_status(["ambiguous-candidate", "blocked-profile"].into_iter()),
            "unavailable-evidence"
        );
        assert_eq!(
            coverage_status(["header-not-observed", "no-candidate"].into_iter()),
            "observed-no-candidate"
        );
        assert_eq!(
            coverage_status(["header-not-observed"].into_iter()),
            "unobserved-header"
        );
    }
}
