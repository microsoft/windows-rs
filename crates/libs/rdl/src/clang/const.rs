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
        // Skip function-like macros (e.g. `#define FOO(x) ...`).
        if cursor.is_macro_function_like() {
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

        let value = match parse_body(&body, parser.namespace, parser.ref_map) {
            Some(v) => v,
            None => return Ok(None),
        };

        Ok(Some(Self { name, value }))
    }

    pub fn write(&self, namespace: &str) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);
        let ty = write_type(namespace, &self.value.ty());
        let value = write_value(namespace, &self.value);
        Ok(quote! { const #name: #ty = #value; })
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
    /// `bindgen`: for each candidate macro name we inject a dedicated anonymous
    /// `enum` into a synthetic in-memory translation unit that `#include`s the
    /// original header.  The C/C++ compiler then evaluates the constant
    /// expression in full — handling operator precedence, integer promotions,
    /// cross-macro references, etc. — and records the result as an
    /// `EnumConstantDecl` in the AST.  We read the evaluated value via
    /// `clang_getEnumConstantDeclValue`.
    ///
    /// Using one `enum` per name means that a single bad macro (e.g. one whose
    /// replacement list is not a valid integer constant expression) does not
    /// prevent the other macros from being evaluated.  Combining
    /// `CXTranslationUnit_KeepGoing` ensures libclang continues past errors.
    ///
    /// # Type inference
    ///
    /// `clang_getEnumConstantDeclValue` returns a signed 64-bit integer.  We
    /// apply Windows LLP64 conventions: if the value fits in `i32` it is
    /// emitted as `i32`; otherwise as `i64`.  Explicitly unsigned macros (e.g.
    /// `0xFFFFFFFFU`) are handled correctly by the token-based parser and
    /// therefore never reach this path.
    pub fn evaluate_macros(
        input: &str,
        names: &[String],
        index: &Index,
        args: &[&str],
    ) -> Result<Vec<Self>, Error> {
        if names.is_empty() {
            return Ok(vec![]);
        }

        // Build the synthetic source.  Use the basename of `input` in the
        // #include so that libclang resolves it relative to the synthetic
        // file's own directory (which shares the same parent directory as the
        // real header).
        let input_basename = std::path::Path::new(input)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(input);

        let mut source = format!("#include \"{input_basename}\"\n");
        for name in names {
            // Guard against macro names that are not valid C identifiers.  In
            // practice every name that reaches this point came from libclang's
            // own cursor spelling (a validated preprocessor token), but we
            // sanitise anyway before interpolating into generated C++ code.
            if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                continue;
            }
            source.push_str(&format!("enum {{ __rdl_eval_{name} = {name} }};\n"));
        }

        // Name the synthetic file in the same directory as the real header so
        // that relative #include paths inside the header continue to resolve.
        let synthetic = format!("{input}.__rdl_eval__.cpp");

        // Parse with KeepGoing so that macros that are not valid integer
        // constant expressions (e.g. string macros) don't abort the TU.
        let tu = index.parse_unsaved(&synthetic, &source, args, CXTranslationUnit_KeepGoing)?;

        collect_eval_results(&tu)
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
        let mut source = format!("{content}\n");
        for name in names {
            if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                continue;
            }
            source.push_str(&format!("enum {{ __rdl_eval_{name} = {name} }};\n"));
        }

        const SYNTHETIC: &str = "__rdl_input_str_eval__.cpp";

        let tu = index.parse_unsaved(SYNTHETIC, &source, args, CXTranslationUnit_KeepGoing)?;

        collect_eval_results(&tu)
    }
}

/// Collect the values of all `__rdl_eval_*` enum constants from a synthetic
/// evaluation translation unit.
///
/// Both [`Const::evaluate_macros`] and [`Const::evaluate_macros_str`] produce
/// translation units that contain one anonymous `enum` per candidate macro,
/// where the enumerator is named `__rdl_eval_<original_name>`.  This helper
/// walks the TU cursor, strips the prefix, and converts the evaluated integer
/// value to the narrowest fitting [`metadata::Value`] variant.
fn collect_eval_results(tu: &TranslationUnit) -> Result<Vec<Const>, Error> {
    let mut results = vec![];
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
                let raw = constant.enum_value();
                let value = if let Ok(v) = i32::try_from(raw) {
                    metadata::Value::I32(v)
                } else {
                    metadata::Value::I64(raw)
                };
                results.push(Const {
                    name: original_name.to_string(),
                    value,
                });
            }
        }
    }
    Ok(results)
}

// ---------------------------------------------------------------------------
// Token-based body parser
// ---------------------------------------------------------------------------

/// Parse a Win32-style `#define` replacement-list token sequence.
///
/// Recognised patterns (body tokens after the macro name):
/// - `LITERAL`                       → numeric or string constant
/// - `- LITERAL`                     → negated integer constant
/// - `( ( IDENT ) LITERAL )`         → typed integer cast (2 parens)
/// - `( IDENT ) LITERAL`             → typed integer cast (1 paren)
/// - `( ( IDENT ) - LITERAL )`       → typed negated cast (2 parens)
/// - `( IDENT ) - LITERAL`           → typed negated cast (1 paren)
///
/// Returns `None` for anything more complex (multi-identifier bodies, macro
/// calls, etc.) which are silently skipped.
fn parse_body(
    body: &[(CXTokenKind, String)],
    namespace: &str,
    ref_map: &HashMap<String, String>,
) -> Option<metadata::Value> {
    match body {
        // Single literal token.
        [(CXToken_Literal, lit)] => parse_literal(lit, false),
        // Negated literal.
        [(CXToken_Punctuation, minus), (CXToken_Literal, lit)] if minus == "-" => {
            parse_literal(lit, true)
        }
        // ((TYPE)VALUE) — double-paren typed cast.
        [(CXToken_Punctuation, lp1), (CXToken_Punctuation, lp2), (CXToken_Identifier, ty), (CXToken_Punctuation, rp1), (CXToken_Literal, lit), (CXToken_Punctuation, rp2)]
            if lp1 == "(" && lp2 == "(" && rp1 == ")" && rp2 == ")" =>
        {
            parse_named_cast(namespace, ref_map, ty, lit, false)
        }
        // ((TYPE)-VALUE) — double-paren typed negated cast.
        [(CXToken_Punctuation, lp1), (CXToken_Punctuation, lp2), (CXToken_Identifier, ty), (CXToken_Punctuation, rp1), (CXToken_Punctuation, minus), (CXToken_Literal, lit), (CXToken_Punctuation, rp2)]
            if lp1 == "(" && lp2 == "(" && rp1 == ")" && minus == "-" && rp2 == ")" =>
        {
            parse_named_cast(namespace, ref_map, ty, lit, true)
        }
        // (TYPE)VALUE — single-paren typed cast.
        [(CXToken_Punctuation, lp), (CXToken_Identifier, ty), (CXToken_Punctuation, rp), (CXToken_Literal, lit)]
            if lp == "(" && rp == ")" =>
        {
            parse_named_cast(namespace, ref_map, ty, lit, false)
        }
        // (TYPE)-VALUE — single-paren typed negated cast.
        [(CXToken_Punctuation, lp), (CXToken_Identifier, ty), (CXToken_Punctuation, rp), (CXToken_Punctuation, minus), (CXToken_Literal, lit)]
            if lp == "(" && rp == ")" && minus == "-" =>
        {
            parse_named_cast(namespace, ref_map, ty, lit, true)
        }
        _ => None,
    }
}

/// Parse a C integer or string literal spelling into a [`metadata::Value`].
///
/// Integer literals may carry type suffixes (`L`, `U`, `LL`, `ULL`) and use
/// hexadecimal (`0x…`) or decimal notation.  The value is reinterpreted into
/// the Rust type that best matches the suffix and bit-width, following
/// Windows LLP64 conventions (`long` = 32-bit).
fn parse_literal(lit: &str, negate: bool) -> Option<metadata::Value> {
    // String literal.
    if lit.starts_with('"') {
        if negate {
            return None;
        }
        // Strip surrounding quotes.  Win32 string constants use simple ASCII
        // so no additional escape-sequence translation is performed; the raw
        // C spelling is passed through as-is (Rust and C share the same
        // common escape sequences such as `\n`, `\t`, and `\\`).
        let inner = lit.strip_prefix('"')?.strip_suffix('"')?;
        return Some(metadata::Value::Utf8(inner.to_string()));
    }

    // Integer literal — strip suffix to isolate the digits.
    let (digits, suffix) = split_int_suffix(lit);
    let raw: u64 = parse_int_digits(digits)?;

    // Determine width and signedness from suffix.
    // On Windows (LLP64): long = 32-bit, long long = 64-bit.
    let value = match suffix.to_uppercase().as_str() {
        // Unsigned: U, UL, LU — negated unsigned constants are not representable.
        "U" | "UL" | "LU" => {
            if negate {
                return None;
            }
            let v = u32::try_from(raw).ok()?;
            metadata::Value::U32(v)
        }
        // Unsigned long long: ULL, LLU
        "ULL" | "LLU" => {
            if negate {
                return None;
            }
            metadata::Value::U64(raw)
        }
        // Signed long long: LL
        "LL" => {
            let v = raw as i64;
            let v = if negate { v.wrapping_neg() } else { v };
            metadata::Value::I64(v)
        }
        // No suffix or L suffix → i32 on Windows (LLP64: long is always 32-bit).
        // The wrapping cast from u64 is intentional: Win32 uses bit-pattern
        // reinterpretation for constants like HRESULT values that exceed
        // INT32_MAX (e.g. `0x80004002L` → -2147467262i32).
        _ => {
            let v = raw as u32 as i32;
            let v = if negate { v.wrapping_neg() } else { v };
            metadata::Value::I32(v)
        }
    };

    Some(value)
}

/// Parse a C literal spelling and produce a [`metadata::Value::EnumValue`] with
/// the given type name, interpreting the integer bits as `i64`.
///
/// The value is stored as a raw 64-bit signed integer (the bit pattern
/// of the literal reinterpreted as `i64`).  It will be emitted as a
/// decimal literal in the RDL and reinterpreted according to the actual
/// underlying type of `type_name` during the reader/writer roundtrip.
fn parse_named_cast(
    namespace: &str,
    ref_map: &HashMap<String, String>,
    type_name: &str,
    lit: &str,
    negate: bool,
) -> Option<metadata::Value> {
    let (digits, _suffix) = split_int_suffix(lit);
    let raw: u64 = parse_int_digits(digits)?;
    let v = if negate {
        (raw as i64).wrapping_neg()
    } else {
        raw as i64
    };
    let ns = ref_map
        .get(type_name)
        .map(|s| s.as_str())
        .unwrap_or(namespace);
    Some(metadata::Value::EnumValue(
        metadata::TypeName::named(ns, type_name),
        Box::new(metadata::Value::I64(v)),
    ))
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
        .map(|i| i + 1)
        .unwrap_or(lit.len());
    (&lit[..suffix_start], &lit[suffix_start..])
}

/// Parse hex or decimal digit string into a `u64`.
fn parse_int_digits(digits: &str) -> Option<u64> {
    if let Some(hex) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        u64::from_str_radix(hex, 16).ok()
    } else {
        digits.parse::<u64>().ok()
    }
}
