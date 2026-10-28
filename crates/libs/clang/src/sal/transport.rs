use super::*;

pub(crate) fn read(payload: &str) -> Result<Option<SalAnnotation>, Error> {
    let Some(payload) = payload.strip_prefix("windows_clang:sal:") else {
        return Ok(None);
    };
    let invalid = || Error("invalid SAL annotation transport".into());
    let (version, payload) = payload.split_once(':').ok_or_else(invalid)?;
    if !matches!(version, "1" | "1.1" | "1.2" | "2" | "L") {
        return Err(invalid());
    }
    let parts: Vec<_> = payload.split('\n').collect();
    let [name, arguments, expanded] = parts.as_slice() else {
        return Err(invalid());
    };
    if name.is_empty()
        || !name.bytes().enumerate().all(|(index, byte)| {
            byte == b'_' || byte.is_ascii_alphabetic() || index != 0 && byte.is_ascii_digit()
        })
    {
        return Err(invalid());
    }
    let arguments = arguments
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
        .ok_or_else(invalid)?;
    let expanded = expanded
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
        .ok_or_else(invalid)?;
    Ok(Some(SalAnnotation {
        version: version.into(),
        name: (*name).into(),
        arguments: arguments.into(),
        expanded: expanded.into(),
    }))
}

// Decode only the serialization emitted by the SAL hooks. Other expression text is opaque.
#[cfg(feature = "projection")]
pub(crate) fn expression(mut text: &str) -> Result<String, Error> {
    if !text.contains("windows_clang:sal:") {
        return Ok(text.into());
    }
    const PREFIX: &str = "__attribute__((annotate(";
    let invalid = || Error("invalid nested SAL annotation transport".into());
    let mut result = String::new();
    while !text.is_empty() {
        if let Some(mut tail) = text.strip_prefix(PREFIX)
            && tail.starts_with("\"windows_clang:sal:")
        {
            let mut payload = String::new();
            while tail.starts_with('"') {
                let (value, rest) = string(tail)?;
                payload.push_str(&value);
                tail = rest.trim_start();
            }
            let value = read(&payload)?.ok_or_else(invalid)?;
            text = tail.strip_prefix(")))").ok_or_else(invalid)?;
            result.push_str(&value.name);
            let expanded = expression(&value.expanded)?;
            if !expanded.is_empty() {
                result.push('(');
                result.push_str(&expanded);
                result.push(')');
            }
        } else if let Some(prefix) = ["R\"", "u8R\"", "uR\"", "UR\"", "LR\""]
            .into_iter()
            .find(|prefix| text.starts_with(prefix))
        {
            let tail = &text[prefix.len()..];
            let (delimiter, _) = tail.split_once('(').ok_or_else(invalid)?;
            let end = format!("){delimiter}\"");
            let length = prefix.len() + delimiter.len() + 1;
            let length = length + text[length..].find(&end).ok_or_else(invalid)? + end.len();
            result.push_str(&text[..length]);
            text = &text[length..];
        } else if text.starts_with(['"', '\'']) {
            let quote = text.chars().next().unwrap();
            let mut chars = text[1..].char_indices();
            let mut end = None;
            while let Some((index, ch)) = chars.next() {
                if ch == '\\' {
                    chars.next().ok_or_else(invalid)?;
                } else if ch == quote {
                    end = Some(index + 2);
                    break;
                }
            }
            let end = end.ok_or_else(invalid)?;
            result.push_str(&text[..end]);
            text = &text[end..];
        } else if text.starts_with(|ch: char| ch.is_ascii_digit()) {
            let end = text
                .find(|ch: char| !ch.is_ascii_alphanumeric() && !matches!(ch, '_' | '.' | '\''))
                .unwrap_or(text.len());
            result.push_str(&text[..end]);
            text = &text[end..];
        } else {
            let ch = text.chars().next().unwrap();
            result.push(ch);
            text = &text[ch.len_utf8()..];
        }
    }
    Ok(result)
}

#[cfg(feature = "projection")]
fn string(text: &str) -> Result<(String, &str), Error> {
    let mut value = String::new();
    let mut chars = text[1..].char_indices();
    while let Some((index, ch)) = chars.next() {
        match ch {
            '"' => return Ok((value, &text[index + 2..])),
            '\\' => match chars.next().map(|(_, ch)| ch) {
                Some('\\') => value.push('\\'),
                Some('"') => value.push('"'),
                Some('n') => value.push('\n'),
                _ => return Err(Error("unsupported SAL transport string escape".into())),
            },
            ch => value.push(ch),
        }
    }
    Err(Error("unterminated SAL transport string".into()))
}
