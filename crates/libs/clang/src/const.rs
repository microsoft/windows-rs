use super::*;

/// A Win32-style `#define` constant that has been parsed into a typed value.
#[derive(Debug)]
pub struct Const {
    pub name: String,
    pub value: metadata::Value,
}

impl Const {
    /// Try to parse a `CXCursor_MacroDefinition` cursor as a typed constant.
    ///
    /// Returns `Ok(Some(Const))` when the macro is a simple object-like macro
    /// whose body matches a recognised Win32 constant pattern, `Ok(None)` when
    /// the macro should be silently skipped (built-in, function-like, or too
    /// complex to represent as a constant).
    pub fn parse(cursor: Cursor, parser: &mut Parser<'_>) -> Result<Option<Self>, Error> {
        // Skip built-in macros (e.g. `__LINE__`, `__FILE__`).
        if cursor.is_macro_builtin() {
            return Ok(None);
        }
        // Skip function-like macros (e.g. `#define FOO(x) ...`). `is_macro_function_like`
        // is the primary check, with a source-adjacency backstop for macros it
        // misreports (e.g. `#define ASSERT(exp) ((VOID)0)`), which would otherwise leak
        // the parameter name into the constant as a bogus type.
        if cursor.is_macro_function_like()
            || parser.tu.macro_is_function_like_by_source(cursor.extent())
        {
            return Ok(None);
        }

        let name = cursor.name();
        if name.is_empty() || name.starts_with('_') {
            return Ok(None);
        }

        // Tokenize the macro definition extent.  The first token is always the
        // macro name itself; the remaining tokens are the replacement list.
        let tokens = parser.tu.tokenize(cursor.extent());
        // Body tokens = everything after the name token.
        let body: Vec<_> = tokens.into_iter().skip(1).collect();

        let Some(value) = parse_body(&body, parser.namespace, parser.ref_map, parser.header_names)
        else {
            return Ok(None);
        };

        Ok(Some(Self { name, value }))
    }

    pub fn write(&self, namespace: &str) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);
        let ty = write_type(namespace, &self.value.ty());
        let value = write_value(namespace, &self.value);
        match &self.value {
            metadata::Value::Utf8(_) => {
                let attr = native_encoding_attr("ansi");
                Ok(quote! {
                    #attr
                    const #name: #ty = #value;
                })
            }
            metadata::Value::Utf16(_) => {
                let attr = native_encoding_attr("utf-16");
                Ok(quote! {
                    #attr
                    const #name: #ty = #value;
                })
            }
            _ => Ok(quote! { const #name: #ty = #value; }),
        }
    }
}

/// A GUID constant produced from a C++ class that is only forward-declared (no body) but
/// carries a `__declspec(uuid("..."))` attribute.
///
/// This handles the MIDL pattern for COM server activation CLSIDs, e.g.:
/// ```c
/// class __declspec(uuid("e6756135-1e65-4d17-8576-610761398c3c")) DiaSource;
/// ```
/// which is emitted as: `const DiaSource: GUID = 0xe6756135_1e65_4d17_8576_610761398c3c;`
#[derive(Debug)]
pub struct GuidConst {
    pub name: String,
    /// The UUID string without braces or quotes, e.g. `"e6756135-1e65-4d17-8576-610761398c3c"`.
    pub uuid: String,
}

impl GuidConst {
    pub fn write(&self) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);
        let lit_str = uuid_to_u128_literal(&self.uuid);
        let lit = syn::LitInt::new(&lit_str, Span::call_site());
        Ok(quote! { const #name: GUID = #lit; })
    }
}

impl Const {
    /// Evaluate a batch of macro names that could not be parsed by the simple
    /// token-based parser (e.g. arithmetic expressions, bitwise shifts, or
    /// references to other macros).
    ///
    /// The technique is the industry-standard approach used by tools such as
    /// `bindgen`: for each candidate macro name we inject dedicated anonymous
    /// `enum`s into a synthetic in-memory translation unit that `#include`s the
    /// original header.  The C/C++ compiler then evaluates the constant
    /// expression in full — handling operator precedence, integer promotions,
    /// cross-macro references, etc. — and records the result as an
    /// `EnumConstantDecl` in the AST.  We read the evaluated value via
    /// `clang_getEnumConstantDeclValue`.
    ///
    /// Using independent `enum`s per name (see [`eval_probe`]) means that a
    /// single bad macro (e.g. one whose replacement list is not a valid integer
    /// constant expression) does not prevent the other macros from being
    /// evaluated, and a paired `__rdl_ok_` enumerator lets us discard it from the
    /// AST alone. Combining `CXTranslationUnit_KeepGoing` ensures libclang
    /// continues past errors.
    ///
    /// # Type inference
    ///
    /// `clang_getEnumConstantDeclValue` returns a signed 64-bit integer. A
    /// non-negative result defaults to unsigned (`u32`, widening to `u64`),
    /// matching the Win32 metadata convention for flag and mask macros; a
    /// negative result stays signed (`i32`, widening to `i64`). Explicitly
    /// unsigned macros (e.g. `0xFFFFFFFFU`) are handled by the token-based
    /// parser and never reach this path.
    pub fn evaluate_macros(
        input: &str,
        names: &[String],
        index: &Index,
        args: &[&str],
    ) -> Result<Vec<Self>, Error> {
        if names.is_empty() {
            return Ok(vec![]);
        }

        // Build the synthetic source prefix.  Use the basename of `input` in the
        // #include so that libclang resolves it relative to the synthetic
        // file's own directory (which shares the same parent directory as the
        // real header).
        let input_basename = std::path::Path::new(input)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(input);
        let prefix = format!("#include \"{input_basename}\"\n{NARG_PROLOGUE}");

        // Name the synthetic file in the same directory as the real header so
        // that relative #include paths inside the header continue to resolve.
        let synthetic = format!("{input}.__rdl_eval__.cpp");

        Self::evaluate_names(&prefix, &synthetic, names, index, args)
    }

    /// Evaluate a batch of macro names from an in-memory source string.
    ///
    /// This is the `input_str` counterpart to [`evaluate_macros`].  Because
    /// the source does not exist on disk there is no file to `#include`; the
    /// original content is instead embedded directly at the top of the
    /// synthetic translation unit, followed by one anonymous `enum` per
    /// candidate macro name.
    ///
    /// See [`evaluate_macros`] for a detailed explanation of the technique.
    pub fn evaluate_macros_str(
        content: &str,
        names: &[String],
        index: &Index,
        args: &[&str],
    ) -> Result<Vec<Self>, Error> {
        if names.is_empty() {
            return Ok(vec![]);
        }

        // Embed the original content directly; relative #includes inside that
        // content will not resolve (there is no on-disk directory context) but
        // simple self-contained headers work fine.
        let prefix = format!("{content}\n{NARG_PROLOGUE}");
        const SYNTHETIC: &str = "__rdl_input_str_eval__.cpp";

        Self::evaluate_names(&prefix, SYNTHETIC, names, index, args)
    }

    /// Evaluate `names` by packing per-candidate probe enums (see [`eval_probe`])
    /// after `prefix` into a single synthetic translation unit and reading the
    /// recovered enum values.
    ///
    /// Parsing uses `CXTranslationUnit_KeepGoing` and `-ferror-limit=0` so clang
    /// error-recovers every bad enumerator and evaluates the rest; validity is
    /// read from the recovered enum *values*, never from diagnostics, so the
    /// result is a deterministic function of the AST.
    ///
    /// A candidate whose replacement list expands to an unbalanced `{`/`(` would
    /// otherwise swallow the enum declarations of every following candidate in the
    /// same TU (an unbalanced delimiter consumes tokens up to a matching one),
    /// silently dropping valid constants. [`collect_eval_results`] reports which
    /// candidates were *fully present* in the recovered AST; any that were
    /// swallowed are re-evaluated here in progressively smaller sub-batches until
    /// the poison macro is isolated (and dropped) while its victims are recovered.
    /// This keeps the outcome deterministic and independent of how the union batch
    /// happened to be chunked.
    fn evaluate_names(
        prefix: &str,
        synthetic: &str,
        names: &[String],
        index: &Index,
        args: &[&str],
    ) -> Result<Vec<Self>, Error> {
        let eval_args = with_unlimited_errors(args);
        let mut results = vec![];
        // Work list of candidate batches still to evaluate. A batch that suffers
        // a swallow re-queues just its swallowed names, split in half, so the
        // poison converges to a singleton batch (where it is simply dropped).
        let mut queue: Vec<Vec<String>> = vec![names.to_vec()];
        while let Some(batch) = queue.pop() {
            let mut source = String::from(prefix);
            for name in &batch {
                // Guard against macro names that are not valid C identifiers. In
                // practice every name that reaches this point came from libclang's
                // own cursor spelling (a validated preprocessor token), but we
                // sanitise anyway before interpolating into generated C++ code.
                if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                    continue;
                }
                source.push_str(&eval_probe(name));
            }
            let tu =
                index.parse_unsaved(synthetic, &source, &eval_args, CXTranslationUnit_KeepGoing)?;
            let (consts, present) = collect_eval_results(&tu);
            results.extend(consts);

            // Names whose probe enums never made it into the AST were swallowed by
            // a poison macro earlier in this batch. Re-evaluate them isolated.
            let missing: Vec<String> = batch
                .iter()
                .filter(|n| n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
                .filter(|n| !present.contains(n.as_str()))
                .cloned()
                .collect();
            if !missing.is_empty() && batch.len() > 1 {
                // If nothing was recovered the poison is at the batch head, so
                // split the whole batch; otherwise retry only the swallowed names.
                let retry = if missing.len() == batch.len() {
                    &batch
                } else {
                    &missing
                };
                if retry.len() > 1 {
                    let mid = retry.len() / 2;
                    queue.push(retry[mid..].to_vec());
                    queue.push(retry[..mid].to_vec());
                }
                // A singleton `retry` is an isolated poison: it evaluates to
                // nothing on its own and is dropped (its constants were never
                // added), so no further work is queued.
            }
        }
        Ok(results)
    }
}

/// Append `-ferror-limit=0` to the parse arguments for a synthetic evaluation TU.
///
/// A batch eval TU packs hundreds of candidate macros, and every macro that is
/// not a valid integer constant expression (a pointer-valued `MAKEINTRESOURCE`,
/// a function-style alias, …) produces a compile error on its `__rdl_eval_`/
/// `__rdl_ok_` enums. Under clang's *default* error limit (~20) the TU is aborted
/// with "too many errors emitted, stopping now" once the cap is hit, so every
/// enum declared after that point is never created — and the valid integer macros
/// among them silently vanish. Worse, *which* macros fall past the cap depends on
/// how the union batch is chunked, making the drop set environment-dependent.
/// `-ferror-limit=0` (unlimited) keeps clang error-recovering each bad enumerator
/// independently and evaluating the rest, so every macro's `__rdl_ok_`/`__rdl_eval_`
/// pair is present in the AST and the keep/drop decision is deterministic.
fn with_unlimited_errors<'a>(args: &[&'a str]) -> Vec<&'a str> {
    let mut out = args.to_vec();
    out.push("-ferror-limit=0");
    out
}

/// Preprocessor argument-counting macros injected once at the top of every
/// synthetic evaluation TU. `__RDL_NARG(X)` expands to the number of top-level,
/// comma-separated elements `X` yields *after* full macro expansion — `1` for a
/// scalar constant, `> 1` for an initializer list.
///
/// This is what rejects GUID-valued macros. Some are raw comma lists
/// (`#define STATIC_X 0x17f89cb3, 0xc38d, …`); others hide the commas behind a
/// helper macro (`#define STATIC_KSDATAFORMAT_SUBTYPE_ADPCM
/// DEFINE_WAVEFORMATEX_GUID(WAVE_FORMAT_ADPCM)`, which expands to
/// `(USHORT)(…), 0x0000, 0x0010, …`). Wrapped in `(X)` by the value probe, the C
/// comma operator would silently fold either form to its last element and leak a
/// bogus integer. A raw-token comma scan cannot see the post-expansion form, but
/// passing `X` as a *macro argument* expands it first, so the count is exact for
/// both. The sequence runs to 20 to cover the 11-element GUID initializer.
const NARG_PROLOGUE: &str = "\
#define __RDL_NARG(...) __RDL_NARG_(__VA_ARGS__,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0)\n\
#define __RDL_NARG_(_1,_2,_3,_4,_5,_6,_7,_8,_9,_10,_11,_12,_13,_14,_15,_16,_17,_18,_19,_20,N,...) N\n";

/// The synthetic probe emitted for one candidate macro. It carries three
/// *separate* anonymous enums:
///
/// - `__rdl_eval_<name> = (<name>)` — the evaluated value.
/// - `__rdl_ok_<name>   = ((<name>) & 0) + 1` — a type gate that is `1` for
///   **any** valid *integer* constant `<name>` on every architecture (`x & 0` is
///   `0` for every integer value, so the enumerator is a fixed `1`), and
///   error-recovers to `0` (the first enumerator of its own enum) when `<name>`
///   is not a valid integer constant expression.
/// - `__rdl_nc_<name>   = __RDL_NARG(<name>)` — a shape gate: the count of
///   top-level comma-separated elements `<name>` expands to. A scalar constant
///   yields `1`; a GUID/initializer list yields `> 1`.
///
/// A candidate is kept only when `__rdl_ok_ == 1` **and** `__rdl_nc_ == 1`.
///
/// The `& 0` type gate is deliberate: bitwise-and requires *integer* operands, so
/// a macro that expands to a pointer (`MAKEINTRESOURCE(n)` → `(LPWSTR)…`,
/// `&global`) or a floating/string expression makes the gate ill-typed and clang
/// error-recovers it to `0`. An earlier `(<name>) - (<name>) + 1` gate accepted
/// those, because pointer *subtraction* is a valid integer expression
/// (`ptr - ptr == 0`), so `MAKEINTRESOURCE`-style macros leaked as bogus
/// `const X = 0`. The `__RDL_NARG` shape gate (see [`NARG_PROLOGUE`]) independently
/// rejects comma-separated GUID initializers, which the type gate alone cannot —
/// the comma operator folds them to a single valid integer.
///
/// Reading validity from the recovered enum *values* — rather than matching clang
/// error diagnostics by line, as the previous implementation did — makes the
/// keep/drop decision a deterministic function of the AST. Diagnostic emission is
/// environment-dependent (clang may cap or reorder it under `-ferror-limit`), so
/// the old approach dropped different valid constants on different machines,
/// producing spurious per-architecture `#[arch(...)]` tags on arch-invariant
/// integer `#define`s (e.g. the IIS metabase `MD_*` ids). The enums must be
/// independent so a bad `__rdl_eval` cannot bump a following gate to a non-zero
/// error-recovery value.
fn eval_probe(name: &str) -> String {
    format!(
        "enum {{ __rdl_eval_{name} = ({name}) }};\n\
         enum {{ __rdl_ok_{name} = (({name}) & 0) + 1 }};\n\
         enum {{ __rdl_nc_{name} = __RDL_NARG({name}) }};\n"
    )
}

/// Collect the values of the `__rdl_eval_*` enum constants from a synthetic
/// evaluation translation unit, keeping only those the paired `__rdl_ok_*` (type)
/// and `__rdl_nc_*` (shape) enumerators both validate.
///
/// Both [`Const::evaluate_macros`] and [`Const::evaluate_macros_str`] emit three
/// anonymous enums per candidate macro (see [`eval_probe`]): `__rdl_eval_<name>`
/// holds the evaluated value, `__rdl_ok_<name>` is `1` iff `<name>` is a valid
/// *integer* constant expression, and `__rdl_nc_<name>` is the count of top-level
/// comma-separated elements (`1` for a scalar, `> 1` for a GUID/initializer list).
/// A macro that fails to evaluate (a string macro, an empty include guard, keyword
/// tokens like `STDAPI`, a pointer cast) error-recovers its type gate to `0`; a
/// comma-list macro trips the shape gate. Only macros passing *both* gates are
/// kept — no diagnostics are consulted, so the decision is a deterministic
/// function of the AST across architectures and environments.
/// Collect the values of the `__rdl_eval_*` enum constants from a synthetic
/// evaluation translation unit, keeping only those the paired `__rdl_ok_*` (type)
/// and `__rdl_nc_*` (shape) enumerators both validate.
///
/// Returns the kept constants together with the set of candidate names that were
/// *fully present* in the recovered AST — i.e. all three of their `__rdl_eval_`,
/// `__rdl_ok_`, and `__rdl_nc_` enums were parsed. A name that is absent from
/// this set was **swallowed**: a preceding candidate whose replacement list
/// expands to an unbalanced `{`/`(` consumed the following enum declarations up
/// to a matching delimiter. The caller re-evaluates swallowed names in smaller
/// sub-batches (see [`Const::evaluate_names`]) so a single poison macro cannot
/// silently drop every constant emitted after it. A name that *is* present but
/// fails a gate (a pointer/string/init-list macro) is a genuine reject and is
/// not retried.
///
/// Both [`Const::evaluate_macros`] and [`Const::evaluate_macros_str`] emit three
/// anonymous enums per candidate macro (see [`eval_probe`]). No diagnostics are
/// consulted, so the decision is a deterministic function of the AST across
/// architectures and environments.
fn collect_eval_results(tu: &TranslationUnit) -> (Vec<Const>, HashSet<String>) {
    let mut evals: Vec<(String, i64)> = vec![];
    let mut eval_seen: HashSet<String> = HashSet::new();
    let mut ok_seen: HashSet<String> = HashSet::new();
    let mut nc_seen: HashSet<String> = HashSet::new();
    let mut type_ok: HashSet<String> = HashSet::new();
    let mut shape_ok: HashSet<String> = HashSet::new();
    for child in tu.cursor().children() {
        if !child.is_from_main_file() {
            continue;
        }
        if child.kind() != CXCursor_EnumDecl {
            continue;
        }
        for constant in child.children() {
            if constant.kind() != CXCursor_EnumConstantDecl {
                continue;
            }
            let const_name = constant.name();
            if let Some(original_name) = const_name.strip_prefix("__rdl_eval_") {
                evals.push((original_name.to_string(), constant.enum_value()));
                eval_seen.insert(original_name.to_string());
            } else if let Some(original_name) = const_name.strip_prefix("__rdl_ok_") {
                ok_seen.insert(original_name.to_string());
                if constant.enum_value() == 1 {
                    type_ok.insert(original_name.to_string());
                }
            } else if let Some(original_name) = const_name.strip_prefix("__rdl_nc_") {
                nc_seen.insert(original_name.to_string());
                if constant.enum_value() == 1 {
                    shape_ok.insert(original_name.to_string());
                }
            }
        }
    }

    // A candidate is "fully present" only when all three of its probe enums were
    // parsed. If any is missing, the candidate was swallowed by a poison macro
    // and must be retried in isolation rather than silently dropped.
    let present: HashSet<String> = eval_seen
        .iter()
        .filter(|n| ok_seen.contains(*n) && nc_seen.contains(*n))
        .cloned()
        .collect();

    let kept = evals
        .into_iter()
        .filter(|(name, _)| type_ok.contains(name) && shape_ok.contains(name))
        .map(|(name, raw)| {
            // Non-negative results default to unsigned (matching the Win32
            // metadata convention for flag/mask macros); negative results stay
            // signed. Widen to 64-bit only on overflow of the 32-bit type.
            let value = if raw >= 0 {
                if let Ok(v) = u32::try_from(raw) {
                    metadata::Value::U32(v)
                } else {
                    metadata::Value::U64(raw as u64)
                }
            } else if let Ok(v) = i32::try_from(raw) {
                metadata::Value::I32(v)
            } else {
                metadata::Value::I64(raw)
            };
            Const { name, value }
        })
        .collect();

    (kept, present)
}

/// Parse a Win32-style `#define` replacement-list token sequence.
///
/// Recognised patterns (body tokens after the macro name):
/// - `LITERAL`                       → numeric or string constant
/// - `- LITERAL`                     → negated integer constant
/// - `( LITERAL )`                   → parenthesized literal
/// - `( ( IDENT ) LITERAL )`         → typed integer cast (2 parens)
/// - `( IDENT ) LITERAL`             → typed integer cast (1 paren)
/// - `( ( IDENT ) - LITERAL )`       → typed negated cast (2 parens)
/// - `( IDENT ) - LITERAL`           → typed negated cast (1 paren)
///
/// Bodies that don't match a fixed pattern fall through to
/// [`parse_nested_cast`], which recognises the nested handle/pointer casts used
/// by macros such as `HKEY_LOCAL_MACHINE` and `INVALID_HANDLE_VALUE`. Anything
/// still unrecognised (macro calls, arithmetic, etc.) returns `None` and is
/// silently skipped.
fn parse_body(
    body: &[(CXTokenKind, String)],
    namespace: &str,
    ref_map: &HashMap<String, String>,
    header_names: Option<&HashMap<String, String>>,
) -> Option<metadata::Value> {
    match body {
        // Single literal token.
        [(CXToken_Literal, lit)] => parse_literal(lit, false),
        // Negated literal.
        [(CXToken_Punctuation, minus), (CXToken_Literal, lit)] if minus == "-" => {
            parse_literal(lit, true)
        }
        // (LITERAL) — parenthesized literal (e.g. `#define FOO ( "value" )`).
        [
            (CXToken_Punctuation, lp),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp),
        ] if lp == "(" && rp == ")" => parse_literal(lit, false),
        // ((TYPE)VALUE) — double-paren typed cast.
        [
            (CXToken_Punctuation, lp1),
            (CXToken_Punctuation, lp2),
            (CXToken_Identifier, ty),
            (CXToken_Punctuation, rp1),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp2),
        ] if lp1 == "(" && lp2 == "(" && rp1 == ")" && rp2 == ")" => {
            parse_named_cast(namespace, ref_map, header_names, ty, lit, false)
        }
        // ((TYPE)-VALUE) — double-paren typed negated cast.
        [
            (CXToken_Punctuation, lp1),
            (CXToken_Punctuation, lp2),
            (CXToken_Identifier, ty),
            (CXToken_Punctuation, rp1),
            (CXToken_Punctuation, minus),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp2),
        ] if lp1 == "(" && lp2 == "(" && rp1 == ")" && minus == "-" && rp2 == ")" => {
            parse_named_cast(namespace, ref_map, header_names, ty, lit, true)
        }
        // (TYPE)VALUE — single-paren typed cast.
        [
            (CXToken_Punctuation, lp),
            (CXToken_Identifier, ty),
            (CXToken_Punctuation, rp),
            (CXToken_Literal, lit),
        ] if lp == "(" && rp == ")" => {
            parse_named_cast(namespace, ref_map, header_names, ty, lit, false)
        }
        // (TYPE)-VALUE — single-paren typed negated cast.
        [
            (CXToken_Punctuation, lp),
            (CXToken_Identifier, ty),
            (CXToken_Punctuation, rp),
            (CXToken_Punctuation, minus),
            (CXToken_Literal, lit),
        ] if lp == "(" && rp == ")" && minus == "-" => {
            parse_named_cast(namespace, ref_map, header_names, ty, lit, true)
        }
        // ((KEYWORD)VALUE) — double-paren builtin-keyword cast, e.g. `((int)5)`.
        [
            (CXToken_Punctuation, lp1),
            (CXToken_Punctuation, lp2),
            (CXToken_Keyword, kw),
            (CXToken_Punctuation, rp1),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp2),
        ] if lp1 == "(" && lp2 == "(" && rp1 == ")" && rp2 == ")" => {
            parse_keyword_cast(kw, lit, false)
        }
        // ((KEYWORD)-VALUE) — double-paren builtin-keyword negated cast.
        [
            (CXToken_Punctuation, lp1),
            (CXToken_Punctuation, lp2),
            (CXToken_Keyword, kw),
            (CXToken_Punctuation, rp1),
            (CXToken_Punctuation, minus),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp2),
        ] if lp1 == "(" && lp2 == "(" && rp1 == ")" && minus == "-" && rp2 == ")" => {
            parse_keyword_cast(kw, lit, true)
        }
        // (KEYWORD)VALUE — single-paren builtin-keyword cast.
        [
            (CXToken_Punctuation, lp),
            (CXToken_Keyword, kw),
            (CXToken_Punctuation, rp),
            (CXToken_Literal, lit),
        ] if lp == "(" && rp == ")" => parse_keyword_cast(kw, lit, false),
        // (KEYWORD)-VALUE — single-paren builtin-keyword negated cast.
        [
            (CXToken_Punctuation, lp),
            (CXToken_Keyword, kw),
            (CXToken_Punctuation, rp),
            (CXToken_Punctuation, minus),
            (CXToken_Literal, lit),
        ] if lp == "(" && rp == ")" && minus == "-" => parse_keyword_cast(kw, lit, true),
        // WRAPPER(VALUE) — SDK error-code typedef wrapper macro, e.g.
        // `_HRESULT_TYPEDEF_(0x80004005L)` (expands to `((HRESULT)0x80004005L)`).
        [
            (CXToken_Identifier, w),
            (CXToken_Punctuation, lp),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp),
        ] if lp == "(" && rp == ")" && cast_wrapper_macro(w).is_some() => parse_named_cast(
            namespace,
            ref_map,
            header_names,
            cast_wrapper_macro(w)?,
            lit,
            false,
        ),
        // WRAPPER(-VALUE) — SDK error-code typedef wrapper macro, negated value.
        [
            (CXToken_Identifier, w),
            (CXToken_Punctuation, lp),
            (CXToken_Punctuation, minus),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp),
        ] if lp == "(" && rp == ")" && minus == "-" && cast_wrapper_macro(w).is_some() => {
            parse_named_cast(
                namespace,
                ref_map,
                header_names,
                cast_wrapper_macro(w)?,
                lit,
                true,
            )
        }
        // MAKEINTRESOURCE(ORDINAL) — a resource named by integer ordinal, e.g.
        // `#define IDC_ARROW MAKEINTRESOURCE(32512)`. Same token shape as the
        // WRAPPER(VALUE) arms above, disjoint by macro name. Emitted as a
        // `PWSTR`/`PSTR` constant carrying the ordinal (see
        // [`makeintresource_macro`]); the batch evaluator would otherwise drop
        // it as pointer-valued.
        [
            (CXToken_Identifier, w),
            (CXToken_Punctuation, lp),
            (CXToken_Literal, lit),
            (CXToken_Punctuation, rp),
        ] if lp == "(" && rp == ")" && makeintresource_macro(w).is_some() => {
            let (digits, _suffix) = split_int_suffix(lit);
            let raw: u64 = parse_int_digits(digits)?;
            Some(metadata::Value::EnumValue(
                metadata::TypeName::named(namespace, makeintresource_macro(w)?),
                Box::new(metadata::Value::I32(raw as i32)),
            ))
        }
        _ => parse_nested_cast(body, namespace, ref_map, header_names),
    }
}

/// Build the `#[encoding("ansi")]` / `#[encoding("utf-16")]` pseudo-attribute used to annotate
/// narrow and wide string constants in the generated RDL; the RDL reader maps it to the metadata
/// `NativeEncodingAttribute`.
fn native_encoding_attr(encoding: &str) -> TokenStream {
    quote! { #[encoding(#encoding)] }
}

/// Parse a C integer or string literal spelling into a [`metadata::Value`].
///
/// Integer literals may carry type suffixes (`L`, `U`, `LL`, `ULL`) and use
/// hexadecimal (`0x…`) or decimal notation.  Non-negative integer constants
/// default to unsigned (`u32`, widening to `u64`); see [`integer_value`].
fn parse_literal(lit: &str, negate: bool) -> Option<metadata::Value> {
    // Wide string literal (L"...").
    if lit.starts_with("L\"") {
        if negate {
            return None;
        }
        let inner = lit.strip_prefix("L\"")?.strip_suffix('"')?;
        return Some(metadata::Value::Utf16(decode_c_wide_string(inner)?));
    }

    // Narrow string literal ("...").
    if lit.starts_with('"') {
        if negate {
            return None;
        }
        let inner = lit.strip_prefix('"')?.strip_suffix('"')?;
        return Some(metadata::Value::Utf8(decode_c_narrow_string(inner)?));
    }

    // Integer literal — strip suffix to isolate the digits.
    let (digits, suffix) = split_int_suffix(lit);
    let raw: u64 = parse_int_digits(digits)?;

    integer_value(raw, suffix, negate)
}

/// Decode a C narrow-string literal body (the text between the quotes) into its
/// actual bytes, then interpret those bytes as UTF-8.
///
/// Returns `None` when the decoded bytes are not valid UTF-8 — i.e. a raw byte
/// array such as the `"\xaa\x31…"` GUID spellings, which has no faithful `String`
/// representation. The reference metadata omits such constants, and so do we: a
/// `Value::Utf8` must hold real UTF-8, not a re-encoded copy that would change the
/// byte length.
fn decode_c_narrow_string(inner: &str) -> Option<String> {
    let mut bytes = Vec::with_capacity(inner.len());
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            let mut buf = [0u8; 4];
            bytes.extend_from_slice(c.encode_utf8(&mut buf).as_bytes());
            continue;
        }
        match chars.next()? {
            '\\' => bytes.push(b'\\'),
            '"' => bytes.push(b'"'),
            '\'' => bytes.push(b'\''),
            '?' => bytes.push(b'?'),
            'n' => bytes.push(b'\n'),
            'r' => bytes.push(b'\r'),
            't' => bytes.push(b'\t'),
            'a' => bytes.push(0x07),
            'b' => bytes.push(0x08),
            'f' => bytes.push(0x0c),
            'v' => bytes.push(0x0b),
            'x' => {
                let (value, count) = take_radix(&mut chars, 16, usize::MAX);
                if count == 0 {
                    return None;
                }
                bytes.push(value as u8);
            }
            o @ '0'..='7' => {
                let mut value = o.to_digit(8)?;
                let (rest, count) = take_radix(&mut chars, 8, 2);
                value = value * 8u32.pow(count as u32) + rest;
                bytes.push(value as u8);
            }
            other => {
                bytes.push(b'\\');
                let mut buf = [0u8; 4];
                bytes.extend_from_slice(other.encode_utf8(&mut buf).as_bytes());
            }
        }
    }
    String::from_utf8(bytes).ok()
}

/// Decode a C wide-string literal body into a `String`, resolving the standard
/// escape sequences plus `\xNN` / `\uNNNN` / `\UNNNNNNNN` universal characters.
/// Each escape yields one Unicode scalar; an escape that does not name a valid
/// scalar (e.g. a lone surrogate) drops the constant via `None`.
fn decode_c_wide_string(inner: &str) -> Option<String> {
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        let scalar = match chars.next()? {
            '\\' => '\\',
            '"' => '"',
            '\'' => '\'',
            '?' => '?',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            'a' => '\u{07}',
            'b' => '\u{08}',
            'f' => '\u{0c}',
            'v' => '\u{0b}',
            'x' | 'u' | 'U' => {
                let (value, count) = take_radix(&mut chars, 16, usize::MAX);
                if count == 0 {
                    return None;
                }
                char::from_u32(value)?
            }
            o @ '0'..='7' => {
                let mut value = o.to_digit(8)?;
                let (rest, count) = take_radix(&mut chars, 8, 2);
                value = value * 8u32.pow(count as u32) + rest;
                char::from_u32(value)?
            }
            other => {
                out.push('\\');
                other
            }
        };
        out.push(scalar);
    }
    Some(out)
}

/// Consume up to `max` leading digits of the given radix from `chars`, returning
/// the accumulated value and the number of digits consumed.
fn take_radix(
    chars: &mut std::iter::Peekable<std::str::Chars>,
    radix: u32,
    max: usize,
) -> (u32, usize) {
    let mut value = 0;
    let mut count = 0;
    while count < max {
        match chars.peek().and_then(|c| c.to_digit(radix)) {
            Some(digit) => {
                value = value * radix + digit;
                chars.next();
                count += 1;
            }
            None => break,
        }
    }
    (value, count)
}

///
/// Win32 `#define` constants overwhelmingly denote unsigned domains — `DWORD`
/// flags, bit masks, and error codes — so a non-negative constant defaults to
/// the narrowest unsigned type that holds it (`u32`, widening to `u64`). This
/// mirrors the Windows metadata convention and, crucially, does not let a C
/// literal's incidental signedness dictate the semantic type: the `L` in
/// `#define ERROR_NO_UNICODE_TRANSLATION 1113L` marks width, not signedness, so
/// the constant is still emitted as `u32`. A negated constant (`-1`) is signed
/// (`i32`, widening to `i64`); an explicit `ll`/`LL` suffix forces 64-bit width.
fn integer_value(raw: u64, suffix: &str, negate: bool) -> Option<metadata::Value> {
    let suffix = suffix.to_ascii_uppercase();
    let has_u = suffix.contains('U');
    let force_wide = suffix.contains("LL");

    if negate {
        // A `u`-suffixed magnitude is unsigned and cannot be negated.
        if has_u {
            return None;
        }
        // A negated constant is signed; pick the narrowest signed type whose
        // magnitude holds `raw`.
        if !force_wide && raw <= i32::MAX as u64 {
            return Some(metadata::Value::I32((raw as i32).wrapping_neg()));
        }
        if raw <= i64::MAX as u64 {
            return Some(metadata::Value::I64((raw as i64).wrapping_neg()));
        }
        return None;
    }

    // Non-negative constants default to unsigned, widened to `u64` only when
    // they overflow `u32` or carry an explicit `ll`/`LL` width suffix.
    if !force_wide && raw <= u32::MAX as u64 {
        return Some(metadata::Value::U32(raw as u32));
    }
    Some(metadata::Value::U64(raw))
}

/// Parse a C literal spelling and produce a [`metadata::Value::EnumValue`] with
/// the given type name, interpreting the integer bits as `i64`.
///
/// The value is stored as a raw 64-bit signed integer (the bit pattern
/// of the literal reinterpreted as `i64`).  It will be emitted as a
/// decimal literal in the RDL and reinterpreted according to the actual
/// underlying type of `type_name` during the reader/writer roundtrip.
/// Parse a builtin-keyword integer cast (`(int)5`, `((long)0x80000000)`) into a
/// primitive [`metadata::Value`], honouring the cast type's width and signedness.
///
/// Unlike the general unsigned default applied to bare integer constants, an
/// explicit `(int)`/`(long)` cast is a deliberate signedness signal from the
/// header author (e.g. `CW_USEDEFAULT = (int)0x80000000` is `INT_MIN`), so the
/// cast type is preserved. Keywords with no single-token scalar mapping (e.g.
/// `unsigned`, `long long`) return `None` and defer to the evaluation path.
fn parse_keyword_cast(kw: &str, lit: &str, negate: bool) -> Option<metadata::Value> {
    let ty = keyword_scalar(kw)?;
    let (digits, _suffix) = split_int_suffix(lit);
    let raw: u64 = parse_int_digits(digits)?;
    scalar_value(&ty, raw, negate)
}

/// Maps an SDK error-code *typedef wrapper macro* to the cast type it applies.
///
/// The Windows SDK wraps its error-code constants in single-argument
/// function-like macros that expand to a fixed cast, e.g.
/// `#define _HRESULT_TYPEDEF_(_sc) ((HRESULT)_sc)` and
/// `#define _NDIS_ERROR_TYPEDEF_(_sc) (DWORD)(_sc)`. A constant such as
/// `#define E_FAIL _HRESULT_TYPEDEF_(0x80004005L)` therefore carries the same
/// deliberate type as an explicit `(HRESULT)` cast; recognising the wrapper by
/// name lets the token parser preserve that type (`HRESULT`) instead of letting
/// the constant flatten to a bare integer on the evaluation path. The returned
/// name is resolved through [`parse_named_cast`] like any other typed cast.
fn cast_wrapper_macro(name: &str) -> Option<&'static str> {
    Some(match name {
        "_HRESULT_TYPEDEF_" | "_ASF_HRESULT_TYPEDEF_" => "HRESULT",
        "_NDIS_ERROR_TYPEDEF_" => "DWORD",
        _ => return None,
    })
}

/// Maps a `MAKEINTRESOURCE`-family macro to the string-pointer type its integer
/// argument is cast to.
///
/// A resource named by *ordinal* — `#define IDC_ARROW MAKEINTRESOURCE(32512)` —
/// expands to `((LPWSTR)((ULONG_PTR)((WORD)(i))))`: a wide string pointer that
/// *holds the integer id* rather than pointing at a character buffer. The batch
/// evaluator rejects it as a pointer-valued (non-integer) macro, so it would
/// otherwise be dropped. It is recognised here by name from the raw `#define`
/// body and emitted as a `PWSTR`/`PSTR` constant carrying the ordinal; bindgen
/// projects the const spelling (`IDC_ARROW: PCWSTR = PCWSTR(32512 as _)`). The
/// scrape runs without `UNICODE`, so the *raw* body token is matched directly
/// (bare `MAKEINTRESOURCE` is treated as wide, matching the reference metadata)
/// rather than relying on the `…A`/`…W` expansion.
fn makeintresource_macro(name: &str) -> Option<&'static str> {
    Some(match name {
        "MAKEINTRESOURCE" | "MAKEINTRESOURCEW" => "PWSTR",
        "MAKEINTRESOURCEA" => "PSTR",
        _ => return None,
    })
}

/// Maps a single C builtin integer *keyword* to its primitive [`metadata::Type`]
/// under LLP64 (`long` = 32-bit). Multi-token spellings (`unsigned int`,
/// `long long`) are not handled here and fall through to the evaluation path.
fn keyword_scalar(name: &str) -> Option<metadata::Type> {
    Some(match name {
        "char" => metadata::Type::I8,
        "short" => metadata::Type::I16,
        "int" | "long" => metadata::Type::I32,
        "unsigned" => metadata::Type::U32,
        _ => return None,
    })
}

fn parse_named_cast(
    namespace: &str,
    ref_map: &HashMap<String, String>,
    header_names: Option<&HashMap<String, String>>,
    type_name: &str,
    lit: &str,
    negate: bool,
) -> Option<metadata::Value> {
    let (digits, _suffix) = split_int_suffix(lit);
    let raw: u64 = parse_int_digits(digits)?;

    // A cast to a *fundamental scalar* typedef (e.g. `(DWORD)0x42`) is just a typed
    // integer of the underlying primitive — the same fundamentals that collapse
    // elsewhere — so emit it as that primitive rather than a dangling named-type
    // reference. A cast to a type the reference *preserves* (a real enum, or a seed
    // scalar like `BOOL`/`HRESULT`) keeps the named type.
    if !ref_map.contains_key(type_name)
        && let Some(ty) = fundamental_scalar(type_name)
    {
        return scalar_value(&ty, raw, negate);
    }

    // A cast to a *pointer-sized* typedef (`ULONG_PTR`/`DWORD_PTR`/`LONG_PTR`/…) is
    // likewise collapsed to the native-int primitive (`usize`/`isize`) rather than a
    // dangling named-type reference — matching the flat-mode alias collapse in
    // `pointer_sized_abi`, which no longer emits a `type ULONG_PTR = …` item. Sentinel
    // constants such as `#define SSRVOPT_RESET ((ULONG_PTR)-1)` reach us here.
    if !ref_map.contains_key(type_name)
        && let Some(ty) = pointer_sized_abi(type_name)
    {
        return scalar_value(&ty, raw, negate);
    }

    // A cast to a *void-pointer* alias (`((PVOID)-1)`, a pointer sentinel) collapses to the
    // pointer-sized native integer, like the `pointer_sized_abi` aliases above:
    // `void_pointer_alias` suppresses the named `PVOID`/`LPVOID` definition, so keeping the
    // cast name here would dangle.
    if !ref_map.contains_key(type_name) && void_pointer_alias(type_name).is_some() {
        return scalar_value(&metadata::Type::USize, raw, negate);
    }

    // A cast to a string-pointer alias — `#define CAT_MEMBERINFO_STRUCT ((LPCSTR)2222)`,
    // a resource named by ordinal — normalises to the canonical `PWSTR`/`PCWSTR`/`PSTR`/
    // `PCSTR` spelling, matching the field/parameter normalization in `to_type` and the
    // reference metadata. The redundant `LP*` alias definition is suppressed, so keeping
    // the raw cast name here would leave the constant's type dangling.
    let type_name = string_alias_canonical(type_name).unwrap_or(type_name);

    // Resolve the named type's namespace. In per-header mode the type carries no
    // clang cursor (the cast is token-based), so the global name → defining-header
    // map locates its partition (e.g. `ATOM` → `…Minwindef`); legacy mode uses the
    // `ref_map`. Either way an unknown name falls back to the const's own namespace.
    let ns = header_names
        .and_then(|m| m.get(type_name))
        .or_else(|| ref_map.get(type_name))
        .map_or(namespace, |s| s.as_str());
    let v = if negate {
        (raw as i64).wrapping_neg()
    } else {
        raw as i64
    };
    Some(metadata::Value::EnumValue(
        metadata::TypeName::named(ns, type_name),
        Box::new(metadata::Value::I64(v)),
    ))
}

/// Parse a *nested* C cast whose outermost cast targets a pointer/handle type,
/// the form used by Win32 handle constants:
///
/// - `((HKEY)(ULONG_PTR)((LONG)0x80000002))`  → `HKEY_LOCAL_MACHINE`
/// - `((HANDLE)(LONG_PTR)-1)`                  → `INVALID_HANDLE_VALUE`
///
/// The simple fixed patterns in [`parse_body`] only match a single cast, so
/// these multi-cast bodies fall through to here. The value's faithful bit
/// pattern is determined by the *innermost* scalar cast (the SDK spells the
/// reinterpreted integer there): `(LONG)0x80000002` is the signed `i32`
/// `-2147483646`, which the reader/writer later sign-extends through the pointer
/// type's `as _`. Emitting the outer (unsigned `ULONG_PTR`) reading instead
/// would zero-extend and produce the wrong pointer.
///
/// The body is treated as a chain of casts wrapping one optionally-negated
/// integer literal; parentheses are ignored and any other token (operators,
/// extra identifiers) bails out as `None`. The outermost cast must be a real
/// named (non-fundamental) type — a nested cast whose outer target is itself a
/// fundamental scalar does not occur in SDK `#define`s and is left unhandled.
fn parse_nested_cast(
    body: &[(CXTokenKind, String)],
    namespace: &str,
    ref_map: &HashMap<String, String>,
    header_names: Option<&HashMap<String, String>>,
) -> Option<metadata::Value> {
    let mut casts: Vec<&str> = Vec::new();
    let mut negate = false;
    let mut literal: Option<&str> = None;

    for (kind, tok) in body {
        match *kind {
            CXToken_Punctuation => match tok.as_str() {
                "(" | ")" => {}
                "-" if literal.is_none() && !negate => negate = true,
                _ => return None,
            },
            CXToken_Identifier => {
                // An identifier after the literal is not part of a cast chain.
                if literal.is_some() {
                    return None;
                }
                casts.push(tok);
            }
            CXToken_Literal => {
                if literal.is_some() {
                    return None;
                }
                literal = Some(tok);
            }
            _ => return None,
        }
    }

    let lit = literal?;
    // A nested cast needs an outer (pointer/handle) cast plus at least one inner
    // scalar cast; single casts are already handled by the fixed patterns.
    if casts.len() < 2 {
        return None;
    }
    let outer = casts[0];
    let inner = casts[casts.len() - 1];
    // Scoped to genuine handle/pointer constants.
    if fundamental_scalar(outer).is_some() {
        return None;
    }
    // A string-pointer alias outer cast normalises to its canonical spelling, as in
    // [`parse_named_cast`] — the `LP*` alias definition is suppressed.
    let outer = string_alias_canonical(outer).unwrap_or(outer);

    // A *void*-pointer outer cast (`((PVOID)(MAXULONG_PTR - 1))`, the kernel
    // `MM_ALL_PARTITIONS_OBJECT` sentinel) has no named type to reference —
    // `void_pointer_alias` collapses `PVOID`/`LPVOID` rather than emitting it — and the
    // arithmetic inner expression cannot be faithfully evaluated by the token parser. Drop
    // it (as the batch evaluator drops any pointer-valued macro) rather than fabricate a
    // value or dangle on the collapsed name.
    if void_pointer_alias(outer).is_some() {
        return None;
    }

    let (digits, _suffix) = split_int_suffix(lit);
    let raw: u64 = parse_int_digits(digits)?;
    let inner_value = inner_scalar_value(inner, raw, negate);

    let ns = header_names
        .and_then(|m| m.get(outer))
        .or_else(|| ref_map.get(outer))
        .map_or(namespace, |s| s.as_str());
    Some(metadata::Value::EnumValue(
        metadata::TypeName::named(ns, outer),
        Box::new(inner_value),
    ))
}
/// Reads the innermost scalar cast of a nested handle constant into a faithful
/// fixed-width [`metadata::Value`]. A fundamental scalar (`LONG`, `INT`, …)
/// honours that type's width and signedness; a pointer-sized cast
/// (`LONG_PTR`/`ULONG_PTR`/…) or any other integer cast is read as a 64-bit
/// signed value (the widest faithful reinterpretation).
fn inner_scalar_value(inner: &str, raw: u64, negate: bool) -> metadata::Value {
    if let Some(ty) = fundamental_scalar(inner)
        && let Some(value) = scalar_value(&ty, raw, negate)
    {
        return value;
    }
    let v = if negate {
        (raw as i64).wrapping_neg()
    } else {
        raw as i64
    };
    metadata::Value::I64(v)
}

/// Builds a primitive integer [`metadata::Value`] of the given fixed-width scalar
/// `ty` from a raw magnitude, applying `negate` as wrapping two's-complement (so an
/// explicit cast such as `(UINT)-1` yields `0xFFFF_FFFF`, which is well-defined).
fn scalar_value(ty: &metadata::Type, raw: u64, negate: bool) -> Option<metadata::Value> {
    let signed = if negate {
        (raw as i64).wrapping_neg()
    } else {
        raw as i64
    };
    Some(match ty {
        metadata::Type::U8 => metadata::Value::U8(signed as u8),
        metadata::Type::U16 => metadata::Value::U16(signed as u16),
        metadata::Type::U32 => metadata::Value::U32(signed as u32),
        metadata::Type::U64 => metadata::Value::U64(signed as u64),
        metadata::Type::I8 => metadata::Value::I8(signed as i8),
        metadata::Type::I16 => metadata::Value::I16(signed as i16),
        metadata::Type::I32 => metadata::Value::I32(signed as i32),
        metadata::Type::I64 => metadata::Value::I64(signed),
        metadata::Type::USize => metadata::Value::USize(signed as u64),
        metadata::Type::ISize => metadata::Value::ISize(signed),
        _ => return None,
    })
}

/// Split a C integer literal into its digit string and suffix string.
///
/// Examples: `"0x1"` → `("0x1", "")`, `"42L"` → `("42", "L")`,
/// `"0xC0EA0002L"` → `("0xC0EA0002", "L")`.
///
/// The `rfind` searches for the last character that is a valid hex digit or
/// `x`/`X` (the hex prefix marker).  Suffix characters (`L`, `U`, `LL`,
/// `ULL`, etc.) are never hex digits (`'L'` and `'U'` are not in `[0-9a-fA-F]`),
/// so the boundary is always found at the correct position.
fn split_int_suffix(lit: &str) -> (&str, &str) {
    let suffix_start = lit
        .rfind(|c: char| c.is_ascii_hexdigit() || c == 'x' || c == 'X')
        .map_or(lit.len(), |i| i + 1);
    (&lit[..suffix_start], &lit[suffix_start..])
}

/// Parse a C integer digit string (hex `0x…`, binary `0b…`, octal `0…`, or
/// decimal) into a `u64`. A leading `0` followed by more digits is octal per C;
/// an invalid octal digit (`08`/`09`) falls back to decimal rather than dropping
/// the constant.
fn parse_int_digits(digits: &str) -> Option<u64> {
    if let Some(hex) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        u64::from_str_radix(hex, 16).ok()
    } else if let Some(bin) = digits
        .strip_prefix("0b")
        .or_else(|| digits.strip_prefix("0B"))
    {
        u64::from_str_radix(bin, 2).ok()
    } else if digits.len() > 1 && digits.starts_with('0') {
        u64::from_str_radix(&digits[1..], 8)
            .or_else(|_| digits.parse::<u64>())
            .ok()
    } else {
        digits.parse::<u64>().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn narrow_decodes_standard_escapes() {
        assert_eq!(decode_c_narrow_string("!<arch>\\n").unwrap(), "!<arch>\n");
        assert_eq!(
            decode_c_narrow_string("Software\\\\Microsoft\\\\OID").unwrap(),
            "Software\\Microsoft\\OID"
        );
        assert_eq!(decode_c_narrow_string("M\\0\\0\\0").unwrap(), "M\0\0\0");
        assert_eq!(decode_c_narrow_string("a\\tb").unwrap(), "a\tb");
    }

    #[test]
    fn narrow_octal_uses_digit_count_not_value() {
        // `\101` is 'A' (0o101 = 65); the two trailing digits "01" must contribute
        // as octal positions, not as their numeric value 1.
        assert_eq!(decode_c_narrow_string("\\101").unwrap(), "A");
    }

    #[test]
    fn narrow_ascii_hex_bytes_are_kept() {
        // icu bit-tables: every byte is < 0x80, so the result is valid UTF-8.
        assert_eq!(
            decode_c_narrow_string("\\x20\\x30\\x10").unwrap(),
            " 0\u{10}"
        );
    }

    #[test]
    fn narrow_non_utf8_byte_array_is_omitted() {
        // GUID `\xaa\x31…` spellings are raw bytes, not UTF-8 — no faithful String.
        assert_eq!(decode_c_narrow_string("\\xaa\\x31\\x28"), None);
    }

    #[test]
    fn wide_decodes_standard_escapes() {
        assert_eq!(decode_c_wide_string("line\\n").unwrap(), "line\n");
        assert_eq!(decode_c_wide_string("path\\\\here").unwrap(), "path\\here");
        assert_eq!(decode_c_wide_string("\\x41\\x42").unwrap(), "AB");
    }

    #[test]
    fn int_digits_respect_radix_prefix() {
        assert_eq!(parse_int_digits("0x1F"), Some(31));
        assert_eq!(parse_int_digits("0b1010"), Some(10));
        assert_eq!(parse_int_digits("010"), Some(8));
        assert_eq!(parse_int_digits("0"), Some(0));
        assert_eq!(parse_int_digits("42"), Some(42));
        // Invalid octal digit falls back to decimal rather than dropping the value.
        assert_eq!(parse_int_digits("08"), Some(8));
    }
}
