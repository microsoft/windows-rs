use super::*;

/// Source of a translation unit for the macro second-pass evaluator: either a header
/// file path or an in-memory header string.
#[derive(Clone, Copy)]
pub(crate) enum MacroSource<'a> {
    File(&'a str),
    Str(&'a str),
}

/// Evaluate every stem's pending macro names concurrently, returning one result vector
/// per input entry in the original order.
///
/// Evaluate the macro second pass for every partition, but parse the header closure as
/// few times as possible.
///
/// A macro's *value* does not depend on which partition referenced it: every synthetic
/// evaluation TU `#include`s the **whole** closure, so a given `#define` resolves to one
/// translation-unit-wide value regardless of the file it is attributed to. The
/// per-partition split only decides *which file owns* the emitted const (first-owner-wins).
/// So instead of re-parsing the full closure once per macro-bearing partition (the dominant
/// cost of the scrape — hundreds of redundant full-closure parses), the deduplicated
/// **union** of all pending macro names is evaluated in a handful of synthetic TUs — one
/// per worker thread — and the results are routed back to the first partition that
/// requested each name, leaving the deterministic first-owner-wins merge byte-for-byte
/// identical to a per-partition pass.
///
/// A libclang `CXIndex` must not be shared across threads, but independent indexes parse
/// concurrently safely, so every worker owns its own [`Index`].
pub(crate) fn evaluate_macros_parallel(
    all_consts: &[(String, Vec<String>)],
    source: MacroSource<'_>,
    args: &[&str],
) -> Result<Vec<Vec<Const>>, Error> {
    let n = all_consts.len();
    if n == 0 {
        return Ok(vec![]);
    }

    // The deduplicated union of every partition's pending macro names, in first-seen order.
    // Evaluating each name once (rather than once per partition that references it) is what
    // collapses the redundant full-closure re-parses.
    let mut seen = HashSet::new();
    let mut union: Vec<String> = vec![];
    for (_, names) in all_consts {
        for name in names {
            if seen.insert(name.as_str()) {
                union.push(name.clone());
            }
        }
    }

    // Evaluate the union, split into one contiguous chunk per worker. Each chunk is a single
    // synthetic TU (the closure plus that chunk's evaluation enums), so the closure is parsed
    // `workers` times total instead of once per partition.
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
                        // clang-sys loads `libclang` into thread-local storage, so each
                        // worker must load it before use; the guard unloads it at thread end.
                        let _library = Library::new()?;
                        // One index per worker: created and dropped on this thread, never shared.
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

    // Route each evaluated const back to the *first* partition (in `all_consts` order) whose
    // pending list requested it: `remove` hands the value to that partition and leaves later
    // partitions with `None`, exactly reproducing the first-owner-wins selection a
    // per-partition pass would make. Names that failed to evaluate are absent from the map
    // and dropped everywhere, as before.
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

/// The context the per-header walk needs to run the macro second-pass evaluator: the
/// translation-unit source and argument list. Each evaluation creates its own libclang
/// index so the per-stem passes can run concurrently (see [`evaluate_macros_parallel`]).
#[derive(Clone, Copy)]
pub(crate) struct MacroEval<'a> {
    pub(crate) source: MacroSource<'a>,
    pub(crate) args: &'a [&'a str],
}

/// Returns `true` for C/C++ builtin arithmetic-type and signedness keywords that
/// may legitimately appear inside an integer-constant cast (e.g. `(int)`,
/// `(unsigned long)`, `(__int64)`). Such casts — used by `#define`s like
/// `CW_USEDEFAULT ((int)0x80000000)` — are valid constant expressions and must
/// not be filtered out along with genuinely non-constant keyword macros
/// (`extern "C"`, `static`, ...).
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

/// Returns `true` if the delimiters `()`, `[]`, `{}` in the token stream are
/// balanced and correctly nested. Used to reject object-like macros whose
/// replacement list has unbalanced delimiters before they reach the batch macro
/// evaluator, where an unbalanced `{` or `(` would swallow the enum declarations
/// of every following candidate in the same synthetic translation unit.
///
/// Delimiter *characters* are counted (skipping string/character literals and
/// comments, whose contents may contain unmatched `(`/`{`), so a delimiter glued
/// to a line-continuation by the tokenizer (e.g. `"\\\r\n}"`) is still counted
/// correctly and a valid multi-line scalar macro is never mis-rejected.
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

/// Collect every object-like macro definition in the translation unit, mapping
/// each macro name to the spellings of its replacement-list tokens.
///
/// Used to resolve calling-convention macros (`WINAPI`, `CALLBACK`,
/// `STDMETHODCALLTYPE`, ...) back to the underlying `__stdcall` / `__cdecl` /
/// `__fastcall` keyword, transitively and regardless of which (possibly system)
/// header defined them. Only short replacement lists are retained, since
/// convention macros expand to a single token; this keeps the map small.
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
        // The first token is the macro name; the rest is the replacement list.
        let mut body: Vec<String> = tokens.into_iter().skip(1).map(|(_, s)| s).collect();

        // A function-like macro (`STDAPI_(type) ...`) carries its parameter list
        // between the name and the replacement list; strip `( ... )` so the body
        // holds only replacement tokens (e.g. `EXTERN_C type STDAPICALLTYPE`).
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

        // A storage-class specifier such as `__declspec(dllimport)` may prefix an
        // export macro's replacement list (e.g. DWrite's
        // `#define DWRITE_EXPORT __declspec(dllimport) WINAPI`). It is irrelevant to
        // the calling convention but would otherwise push the body past the
        // small-macro length gate below, dropping the macro — and with it the
        // `WINAPI` token — from the table, so `DWriteCreateFactory` would fall back
        // to its `EXTERN_C` linkage and be mis-typed `extern "C"` instead of the
        // `extern "system"` (`__stdcall`) it really is. That mismatch corrupts the
        // stack on the x86 `__stdcall`/`__cdecl` ABI split.
        strip_declspec(&mut body);

        if !body.is_empty() && body.len() <= 4 {
            defs.insert(name, body);
        }
    }

    defs
}

/// Build the reverse alias map consumed by [`Parser::alias_map`] and [`Fn::parse`].
///
/// Scans the object-like macro definitions for the `#define <Alias> <Export>` forwarders
/// the SDK uses to expose an export under a documented name (`RtlGenRandom` →
/// `SystemFunction036`, `EnumProcesses` → `K32EnumProcesses`, `GetMappedFileNameW` →
/// `K32GetMappedFileNameW`) and inverts them to `export -> alias` so a scraped function —
/// whose clang name is the expanded export — can recover the source spelling.
///
/// ANSI/Unicode charset-selection macros (`#define GetWindowText GetWindowTextA`, the
/// `#ifdef UNICODE` idiom) are excluded: their replacement is the same name with an `A`/`W`
/// suffix, selecting a character-set variant rather than forwarding to a distinct export.
/// The reference metadata emits only the explicit `…A`/`…W` functions, so renaming the `A`
/// variant back to the bare name would both diverge from the reference and delete the
/// variant. On the rare chance two aliases share one export, the lexicographically smallest
/// is chosen so the result is deterministic across the unordered macro map.
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

/// True when `s` is a well-formed C identifier (leading letter/underscore, then
/// letters/digits/underscores), used to reject macro replacement lists that are not a bare
/// symbol name (`#define TRUE 1`, operator spellings, ...).
pub(crate) fn is_c_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Strip a leading `__declspec(...)` / `_declspec(...)` storage-class specifier (either MSVC
/// spelling) from a macro body, leaving only the tokens that matter for calling-convention
/// detection — so an export macro like `#define ORAPI _declspec(dllimport) __stdcall` keeps its
/// `__stdcall` and stays within the small-macro length gate.
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
