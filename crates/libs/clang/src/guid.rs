use super::*;

/// True when the clang type refers to `GUID`/`IID`, through const/elaborated aliases.
pub(crate) fn is_guid_type(ty: &Type) -> bool {
    let name = match ty.kind() {
        CXType_Elaborated => ty.underlying_type().ty().name(),
        CXType_Record => ty.ty().name(),
        CXType_Typedef => ty.ty().name(),
        _ => return false,
    };
    matches!(name.as_str(), "GUID" | "_GUID" | "IID")
}

/// Parse a GUID initializer from the AST so macro expressions are compiler-evaluated.
pub(crate) fn parse_guid_initializer_ast(cursor: &Cursor) -> Option<String> {
    let init_list = cursor
        .children()
        .into_iter()
        .find(|c| c.kind() == CXCursor_InitListExpr)?;

    let children = init_list.children();
    if children.len() != 4 {
        return None;
    }

    let data1 = children[0].evaluate_unsigned()?;
    let data2 = children[1].evaluate_unsigned()?;
    let data3 = children[2].evaluate_unsigned()?;

    if data1 > u32::MAX as u64 || data2 > u16::MAX as u64 || data3 > u16::MAX as u64 {
        return None;
    }

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

/// Parse a GUID initializer from tokens by collecting its eleven integer fields.
pub(crate) fn parse_guid_initializer_tokens(tokens: &[(CXTokenKind, String)]) -> Option<String> {
    let eq_pos = tokens
        .iter()
        .position(|(k, s)| *k == CXToken_Punctuation && s == "=")?;

    let mut values = Vec::with_capacity(11);
    for (kind, spelling) in &tokens[eq_pos + 1..] {
        if *kind == CXToken_Literal {
            let v = parse_c_int_literal(spelling)?;
            values.push(v);
        }
    }

    format_guid_from_values(&values)
}

/// Parse `DEFINE_GUID`/`DEFINE_OLEGUID` tokens into `(name, uuid)`.
///
/// `DEFINE_OLEGUID` omits the fixed OLE tail bytes.
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

/// Parse `DEFINE_PROPERTYKEY`/`DEFINE_DEVPROPKEY` tokens into `(name, fmtid_uuid, pid)`.
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

    if values.len() != 12 {
        return None;
    }

    let uuid = format_guid_from_values(&values[..11])?;
    let pid = u32::try_from(values[11]).ok()?;
    Some((name, uuid, pid))
}

/// Format eleven GUID field values as a hyphenated UUID string.
pub(crate) fn format_guid_from_values(values: &[u64]) -> Option<String> {
    if values.len() != 11 {
        return None;
    }

    let data1 = values[0];
    let data2 = values[1];
    let data3 = values[2];

    if data1 > u32::MAX as u64 || data2 > u16::MAX as u64 || data3 > u16::MAX as u64 {
        return None;
    }
    for &b in &values[3..11] {
        if b > u8::MAX as u64 {
            return None;
        }
    }

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

/// Parse a C integer literal into `u64`, stripping `U`/`L` suffixes.
pub(crate) fn parse_c_int_literal(lit: &str) -> Option<u64> {
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
