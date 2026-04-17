use super::*;

/// A Win32-style `#define` constant that has been parsed into a typed value.
#[derive(Debug)]
pub struct Const {
    pub name: String,
    pub namespace: String,
    pub value: ConstValue,
}

/// The resolved value of a parsed `#define` constant.
#[derive(Debug)]
pub enum ConstValue {
    /// A signed 32-bit integer (plain int or `L`-suffixed long on Windows).
    I32(i32),
    /// An unsigned 32-bit integer (`U`/`UL`-suffixed).
    U32(u32),
    /// A signed 64-bit integer (`LL`-suffixed).
    I64(i64),
    /// An unsigned 64-bit integer (`ULL`-suffixed).
    U64(u64),
    /// A UTF-8 string literal.
    Str(String),
    /// An integer cast to a named type, e.g. `((NTSTATUS)0xC0EA0002L)`.
    ///
    /// The value is stored as a raw 64-bit signed integer (the bit pattern
    /// of the literal reinterpreted as `i64`).  It will be emitted as a
    /// decimal literal in the RDL and reinterpreted according to the actual
    /// underlying type of `type_name` during the reader/writer roundtrip.
    Named { type_name: String, value: i64 },
}

impl Const {
    /// Try to parse a `CXCursor_MacroDefinition` cursor as a typed constant.
    ///
    /// Returns `Ok(Some(Const))` when the macro is a simple object-like macro
    /// whose body matches a recognised Win32 constant pattern, `Ok(None)` when
    /// the macro should be silently skipped (built-in, function-like, or too
    /// complex to represent as a constant).
    pub fn parse(
        cursor: Cursor,
        namespace: &str,
        tu: &TranslationUnit,
    ) -> Result<Option<Self>, Error> {
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
        let tokens = tu.tokenize(cursor.extent());
        // Body tokens = everything after the name token.
        let body: Vec<_> = tokens.into_iter().skip(1).collect();

        let value = match parse_body(&body) {
            Some(v) => v,
            None => return Ok(None),
        };

        Ok(Some(Self {
            name,
            namespace: namespace.to_string(),
            value,
        }))
    }

    pub fn write(&self) -> Result<TokenStream, Error> {
        let name = write_ident(&self.name);

        Ok(match &self.value {
            ConstValue::I32(v) => {
                let lit = Literal::i32_unsuffixed(*v);
                quote! { const #name: i32 = #lit; }
            }
            ConstValue::U32(v) => {
                let lit = Literal::u32_unsuffixed(*v);
                quote! { const #name: u32 = #lit; }
            }
            ConstValue::I64(v) => {
                let lit = Literal::i64_unsuffixed(*v);
                quote! { const #name: i64 = #lit; }
            }
            ConstValue::U64(v) => {
                let lit = Literal::u64_unsuffixed(*v);
                quote! { const #name: u64 = #lit; }
            }
            ConstValue::Str(s) => {
                quote! { const #name: String = #s; }
            }
            ConstValue::Named { type_name, value } => {
                let ty = write_type(
                    &self.namespace,
                    &metadata::Type::value_named(&self.namespace, type_name),
                );
                let lit = Literal::i64_unsuffixed(*value);
                quote! { const #name: #ty = #lit; }
            }
        })
    }
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
fn parse_body(body: &[(CXTokenKind, String)]) -> Option<ConstValue> {
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
            parse_named_cast(ty, lit, false)
        }
        // ((TYPE)-VALUE) — double-paren typed negated cast.
        [(CXToken_Punctuation, lp1), (CXToken_Punctuation, lp2), (CXToken_Identifier, ty), (CXToken_Punctuation, rp1), (CXToken_Punctuation, minus), (CXToken_Literal, lit), (CXToken_Punctuation, rp2)]
            if lp1 == "(" && lp2 == "(" && rp1 == ")" && minus == "-" && rp2 == ")" =>
        {
            parse_named_cast(ty, lit, true)
        }
        // (TYPE)VALUE — single-paren typed cast.
        [(CXToken_Punctuation, lp), (CXToken_Identifier, ty), (CXToken_Punctuation, rp), (CXToken_Literal, lit)]
            if lp == "(" && rp == ")" =>
        {
            parse_named_cast(ty, lit, false)
        }
        // (TYPE)-VALUE — single-paren typed negated cast.
        [(CXToken_Punctuation, lp), (CXToken_Identifier, ty), (CXToken_Punctuation, rp), (CXToken_Punctuation, minus), (CXToken_Literal, lit)]
            if lp == "(" && rp == ")" && minus == "-" =>
        {
            parse_named_cast(ty, lit, true)
        }
        _ => None,
    }
}

/// Parse a C integer or string literal spelling into a [`ConstValue`].
///
/// Integer literals may carry type suffixes (`L`, `U`, `LL`, `ULL`) and use
/// hexadecimal (`0x…`) or decimal notation.  The value is reinterpreted into
/// the Rust type that best matches the suffix and bit-width, following
/// Windows LLP64 conventions (`long` = 32-bit).
fn parse_literal(lit: &str, negate: bool) -> Option<ConstValue> {
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
        return Some(ConstValue::Str(inner.to_string()));
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
            ConstValue::U32(v)
        }
        // Unsigned long long: ULL, LLU
        "ULL" | "LLU" => {
            if negate {
                return None;
            }
            ConstValue::U64(raw)
        }
        // Signed long long: LL
        "LL" => {
            let v = raw as i64;
            let v = if negate { v.wrapping_neg() } else { v };
            ConstValue::I64(v)
        }
        // No suffix or L suffix → i32 on Windows (LLP64: long is always 32-bit).
        // The wrapping cast from u64 is intentional: Win32 uses bit-pattern
        // reinterpretation for constants like HRESULT values that exceed
        // INT32_MAX (e.g. `0x80004002L` → -2147467262i32).
        _ => {
            let v = raw as u32 as i32;
            let v = if negate { v.wrapping_neg() } else { v };
            ConstValue::I32(v)
        }
    };

    Some(value)
}

/// Parse a C literal spelling and produce a [`ConstValue::Named`] with the
/// given type name, interpreting the integer bits as `i64`.
fn parse_named_cast(type_name: &str, lit: &str, negate: bool) -> Option<ConstValue> {
    let (digits, _suffix) = split_int_suffix(lit);
    let raw: u64 = parse_int_digits(digits)?;
    let v = if negate {
        (raw as i64).wrapping_neg()
    } else {
        raw as i64
    };
    Some(ConstValue::Named {
        type_name: type_name.to_string(),
        value: v,
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
        .map(|i| i + 1)
        .unwrap_or(lit.len());
    (&lit[..suffix_start], &lit[suffix_start..])
}

/// Parse hex or decimal digit string into a `u64`.
fn parse_int_digits(digits: &str) -> Option<u64> {
    if let Some(hex) = digits.strip_prefix("0x").or_else(|| digits.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).ok()
    } else {
        digits.parse::<u64>().ok()
    }
}
