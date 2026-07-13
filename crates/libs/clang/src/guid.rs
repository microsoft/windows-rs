use super::*;

/// Returns `true` if the clang `Type` refers to a GUID struct.
///
/// Handles `const GUID`, `const IID`, `const struct _GUID`, elaborated types,
/// and typedef aliases for `GUID`/`IID`.
pub(crate) fn is_guid_type(ty: &Type) -> bool {
    // Peel off any top-level const qualifier by looking at the canonical type.
    let name = match ty.kind() {
        CXType_Elaborated => ty.underlying_type().ty().name(),
        CXType_Record => ty.ty().name(),
        CXType_Typedef => ty.ty().name(),
        _ => return false,
    };
    matches!(name.as_str(), "GUID" | "_GUID" | "IID")
}

/// Parse a GUID struct initializer from the AST using `clang_Cursor_Evaluate`.
///
/// This handles cases where macro constants or expressions are used in the
/// GUID initializer (e.g. 7zip's `Z7_DEFINE_GUID` pattern) — the compiler
/// evaluates the expressions after macro expansion so the values are always
/// available regardless of how the initializer was spelled in source.
///
/// The VarDecl cursor for a GUID variable has the shape:
/// - `CXCursor_InitListExpr` (top-level, containing 4 children):
///   - `CXCursor_IntegerLiteral` × 3 (data1, data2, data3)
///   - `CXCursor_InitListExpr` (data4, containing 8 children):
///     - `CXCursor_IntegerLiteral` × 8
pub(crate) fn parse_guid_initializer_ast(cursor: &Cursor) -> Option<String> {
    // Find the top-level InitListExpr child of the VarDecl.
    let init_list = cursor
        .children()
        .into_iter()
        .find(|c| c.kind() == CXCursor_InitListExpr)?;

    let children = init_list.children();
    if children.len() != 4 {
        return None;
    }

    // Evaluate data1, data2, data3.
    let data1 = children[0].evaluate_unsigned()?;
    let data2 = children[1].evaluate_unsigned()?;
    let data3 = children[2].evaluate_unsigned()?;

    // Range-check: data1 ≤ u32, data2/data3 ≤ u16.
    if data1 > u32::MAX as u64 || data2 > u16::MAX as u64 || data3 > u16::MAX as u64 {
        return None;
    }

    // The 4th child should be an InitListExpr for data4[8].
    let data4_cursor = &children[3];
    if data4_cursor.kind() != CXCursor_InitListExpr {
        return None;
    }

    let data4_children = data4_cursor.children();
    if data4_children.len() != 8 {
        return None;
    }

    let mut data4 = [0u8; 8];
    for (i, child) in data4_children.iter().enumerate() {
        let v = child.evaluate_unsigned()?;
        if v > u8::MAX as u64 {
            return None;
        }
        data4[i] = v as u8;
    }

    Some(format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        data1,
        data2,
        data3,
        data4[0],
        data4[1],
        data4[2],
        data4[3],
        data4[4],
        data4[5],
        data4[6],
        data4[7],
    ))
}

/// Parse a GUID struct initializer from a token stream.
///
/// Expects the token stream for a variable declaration like:
/// ```c
/// const GUID IID_IFoo = { 0x23170F69, 0x40C1, 0x278A, { 0, 0, 0, 3, 0, 1, 0, 0 } };
/// ```
///
/// Scans past the `=` token, then collects exactly 11 integer literals from
/// the balanced `{ ... { ... } }` initializer: `data1` (u32), `data2` (u16),
/// `data3` (u16), and `data4[0..8]` (8 × u8).
///
/// Returns the UUID in standard `"xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"` format,
/// or `None` if the token sequence does not match the expected shape.
pub(crate) fn parse_guid_initializer_tokens(tokens: &[(CXTokenKind, String)]) -> Option<String> {
    // Find the `=` that starts the initializer.
    let eq_pos = tokens
        .iter()
        .position(|(k, s)| *k == CXToken_Punctuation && s == "=")?;

    // Collect all integer literals after the `=`.
    let mut values = Vec::with_capacity(11);
    for (kind, spelling) in &tokens[eq_pos + 1..] {
        if *kind == CXToken_Literal {
            let v = parse_c_int_literal(spelling)?;
            values.push(v);
        }
    }

    format_guid_from_values(&values)
}

/// Parse a `DEFINE_GUID(name, l, w1, w2, b1, …, b8)` (or the
/// `DEFINE_OLEGUID(name, l, w1, w2)` shorthand) macro-expansion token stream into
/// the GUID constant's name and its standard hyphenated UUID string.
///
/// The first identifier after the opening `(` is the constant's name; the
/// remaining integer literals are the GUID field values. The OLE shorthand omits
/// the trailing eight bytes, which are the fixed `{ 0xC0, 0, 0, 0, 0, 0, 0, 0x46 }`
/// sequence shared by every OLE-defined GUID.
pub(crate) fn parse_define_guid_tokens(
    tokens: &[(CXTokenKind, String)],
    ole: bool,
) -> Option<(String, String)> {
    let lparen = tokens
        .iter()
        .position(|(k, s)| *k == CXToken_Punctuation && s == "(")?;

    let name = tokens[lparen + 1..]
        .iter()
        .find(|(k, _)| *k == CXToken_Identifier)
        .map(|(_, s)| s.clone())?;

    let mut values: Vec<u64> = tokens[lparen + 1..]
        .iter()
        .filter(|(k, _)| *k == CXToken_Literal)
        .map(|(_, s)| parse_c_int_literal(s))
        .collect::<Option<_>>()?;

    if ole {
        if values.len() != 3 {
            return None;
        }
        values.extend_from_slice(&[0xC0, 0, 0, 0, 0, 0, 0, 0x46]);
    }

    let uuid = format_guid_from_values(&values)?;
    Some((name, uuid))
}

/// Parse a `DEFINE_PROPERTYKEY(name, l, w1, w2, b1..b8, pid)` /
/// `DEFINE_DEVPROPKEY(...)` macro invocation into `(name, fmtid_uuid, pid)`. Both macros
/// take the same 13 arguments: the constant name, the eleven GUID components, and a trailing
/// `pid`. Returns `None` when the token shape does not match (e.g. a non-literal argument).
pub(crate) fn parse_define_property_key_tokens(
    tokens: &[(CXTokenKind, String)],
) -> Option<(String, String, u32)> {
    let lparen = tokens
        .iter()
        .position(|(k, s)| *k == CXToken_Punctuation && s == "(")?;

    let name = tokens[lparen + 1..]
        .iter()
        .find(|(k, _)| *k == CXToken_Identifier)
        .map(|(_, s)| s.clone())?;

    let values: Vec<u64> = tokens[lparen + 1..]
        .iter()
        .filter(|(k, _)| *k == CXToken_Literal)
        .map(|(_, s)| parse_c_int_literal(s))
        .collect::<Option<_>>()?;

    // Eleven GUID components plus the trailing `pid`.
    if values.len() != 12 {
        return None;
    }

    let uuid = format_guid_from_values(&values[..11])?;
    let pid = u32::try_from(values[11]).ok()?;
    Some((name, uuid, pid))
}

/// Format the eleven GUID field values (`data1`, `data2`, `data3`, `data4[0..8]`)
/// as a standard hyphenated UUID string, range-checking each field.
pub(crate) fn format_guid_from_values(values: &[u64]) -> Option<String> {
    // Must have exactly 11 values: data1, data2, data3, data4[0..8].
    if values.len() != 11 {
        return None;
    }

    let data1 = values[0];
    let data2 = values[1];
    let data3 = values[2];

    // Range-check: data1 ≤ u32, data2/data3 ≤ u16, data4 bytes ≤ u8.
    if data1 > u32::MAX as u64 || data2 > u16::MAX as u64 || data3 > u16::MAX as u64 {
        return None;
    }
    for &b in &values[3..11] {
        if b > u8::MAX as u64 {
            return None;
        }
    }

    // Format as standard UUID: "data1-data2-data3-d4[0]d4[1]-d4[2]..d4[7]"
    Some(format!(
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        data1,
        data2,
        data3,
        values[3],
        values[4],
        values[5],
        values[6],
        values[7],
        values[8],
        values[9],
        values[10],
    ))
}

/// Parse a C integer literal into a `u64`, stripping any type suffix
/// (`U`, `L`, `UL`, `LL`, `ULL`, etc.) and handling hex (`0x`) and decimal.
pub(crate) fn parse_c_int_literal(lit: &str) -> Option<u64> {
    // Strip trailing suffixes (L, U, LL, ULL, etc.).
    let digits = lit.trim_end_matches(['u', 'U', 'l', 'L']);
    if let Some(hex) = digits
        .strip_prefix("0x")
        .or_else(|| digits.strip_prefix("0X"))
    {
        u64::from_str_radix(hex, 16).ok()
    } else {
        digits.parse::<u64>().ok()
    }
}
