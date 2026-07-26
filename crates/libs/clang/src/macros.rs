use super::*;

/// Source of a translation unit for the macro second-pass evaluator: either a header
/// file path or an in-memory header string.
#[derive(Clone, Copy)]
pub(crate) enum MacroSource<'a> {
    File(&'a str),
    Str(&'a str),
}

/// Evaluate all pending macros once, then route each result to its first owning partition.
///
/// Macro values are TU-wide because every evaluator includes the full closure. Each worker
/// owns its own `CXIndex`; libclang indexes are not shared across threads.
pub(crate) fn evaluate_macros_parallel(
    all_consts: &[(String, Vec<String>)],
    source: MacroSource<'_>,
    args: &[&str],
) -> Result<Vec<Vec<Const>>, Error> {
    let n = all_consts.len();
    if n == 0 {
        return Ok(vec![]);
    }

    // Deduplicate in first-seen order to preserve first-owner-wins routing.
    let mut seen = HashSet::new();
    let mut union: Vec<String> = vec![];
    for (_, names) in all_consts {
        for name in names {
            if seen.insert(name.as_str()) {
                union.push(name.clone());
            }
        }
    }

    // Each chunk is one synthetic TU, so the full closure is parsed once per worker.
    let evaluated_union: Vec<Const> = if union.is_empty() {
        vec![]
    } else {
        let workers = std::thread::available_parallelism()
            .map_or(1, |p| p.get())
            .min(union.len());
        let chunk_size = union.len().div_ceil(workers);

        std::thread::scope(|scope| -> Result<Vec<Const>, Error> {
            let handles: Vec<_> = union
                .chunks(chunk_size)
                .map(|chunk| {
                    scope.spawn(move || -> Result<Vec<Const>, Error> {
                        // clang-sys stores libclang in TLS, so each worker loads it itself.
                        let _library = Library::new()?;
                        let index = Index::new()?;
                        match source {
                            MacroSource::File(input) => {
                                Const::evaluate_macros(input, chunk, &index, args)
                            }
                            MacroSource::Str(content) => {
                                Const::evaluate_macros_str(content, chunk, &index, args)
                            }
                        }
                    })
                })
                .collect();

            let mut all = vec![];
            for handle in handles {
                all.extend(
                    handle
                        .join()
                        .map_err(|_| Error::new("macro evaluation worker panicked", "", 0, 0))??,
                );
            }
            Ok(all)
        })?
    };

    // `remove` gives a value only to the first partition that requested it.
    let mut map: HashMap<String, Const> = evaluated_union
        .into_iter()
        .map(|c| (c.name.clone(), c))
        .collect();
    let mut out: Vec<Vec<Const>> = Vec::with_capacity(n);
    for (_, names) in all_consts {
        let mut consts = vec![];
        for name in names {
            if let Some(c) = map.remove(name) {
                consts.push(c);
            }
        }
        out.push(consts);
    }
    Ok(out)
}

/// Source and args needed by the macro second-pass evaluator.
#[derive(Clone, Copy)]
pub(crate) struct MacroEval<'a> {
    pub(crate) source: MacroSource<'a>,
    pub(crate) args: &'a [&'a str],
}

/// True for builtin type keywords allowed in integer-constant casts.
pub(crate) fn is_type_keyword(spelling: &str) -> bool {
    matches!(
        spelling,
        "int"
            | "long"
            | "short"
            | "char"
            | "unsigned"
            | "signed"
            | "bool"
            | "wchar_t"
            | "__int8"
            | "__int16"
            | "__int32"
            | "__int64"
    )
}

/// Reject unbalanced macro bodies before they can swallow later synthetic enum probes.
///
/// Counts delimiter characters outside literals/comments, including delimiters glued to
/// line-continuation tokens.
pub(crate) fn tokens_balanced<'a>(tokens: impl Iterator<Item = &'a (CXTokenKind, String)>) -> bool {
    let mut stack: Vec<char> = vec![];
    for (kind, spelling) in tokens {
        if *kind == CXToken_Literal || *kind == CXToken_Comment {
            continue;
        }
        for ch in spelling.chars() {
            match ch {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' if stack.pop() != Some(ch) => return false,
                _ => {}
            }
        }
    }
    stack.is_empty()
}

/// Collect short macro replacement lists used to resolve calling-convention aliases.
pub(crate) fn collect_macro_defs(tu: &TranslationUnit) -> HashMap<String, Vec<String>> {
    let mut defs = HashMap::new();

    for child in tu.cursor().children() {
        if child.kind() != CXCursor_MacroDefinition || child.is_macro_builtin() {
            continue;
        }

        let name = child.name();
        if name.is_empty() {
            continue;
        }

        let tokens = tu.tokenize(child.extent());
        let mut body: Vec<String> = tokens.into_iter().skip(1).map(|(_, s)| s).collect();

        // Strip a function-like macro's parameter list from its body.
        if child.is_macro_function_like() && body.first().map(String::as_str) == Some("(") {
            let mut depth = 0usize;
            let mut end = None;
            for (idx, token) in body.iter().enumerate() {
                match token.as_str() {
                    "(" => depth += 1,
                    ")" => {
                        depth -= 1;
                        if depth == 0 {
                            end = Some(idx);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if let Some(end) = end {
                body.drain(0..=end);
            }
        }

        // Keep export macros with leading `__declspec(dllimport)` under the length gate.
        strip_declspec(&mut body);

        if !body.is_empty() && body.len() <= 4 {
            defs.insert(name, body);
        }
    }

    defs
}

/// Build `export -> source alias` for SDK macro forwarders like `RtlGenRandom`.
///
/// A/W selection macros are excluded because they select charset variants, not distinct
/// export aliases. If aliases collide, the lexicographically smallest one wins.
pub(crate) fn build_alias_map(
    macro_defs: &HashMap<String, Vec<String>>,
) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for (alias, body) in macro_defs {
        let [export] = body.as_slice() else {
            continue;
        };
        if export == alias || !is_c_identifier(export) {
            continue;
        }
        if *export == format!("{alias}A") || *export == format!("{alias}W") {
            continue;
        }
        map.entry(export.clone())
            .and_modify(|current| {
                if alias < current {
                    current.clone_from(alias);
                }
            })
            .or_insert_with(|| alias.clone());
    }
    map
}

/// True when `s` is a bare C identifier.
pub(crate) fn is_c_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Strip leading MSVC `__declspec(...)` storage-class tokens from a macro body.
pub(crate) fn strip_declspec(body: &mut Vec<String>) {
    let mut i = 0;
    while i < body.len() {
        if matches!(body[i].as_str(), "__declspec" | "_declspec")
            && body.get(i + 1).map(String::as_str) == Some("(")
        {
            let mut depth = 0usize;
            let mut end = None;
            for (idx, token) in body.iter().enumerate().skip(i + 1) {
                match token.as_str() {
                    "(" => depth += 1,
                    ")" => {
                        depth -= 1;
                        if depth == 0 {
                            end = Some(idx);
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if let Some(end) = end {
                body.drain(i..=end);
                continue;
            }
        }
        i += 1;
    }
}
