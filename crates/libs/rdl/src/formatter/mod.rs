use super::Error;
use proc_macro2::{Delimiter, Group, Spacing, TokenStream, TokenTree};

struct Comment {
    text: String,
    trailing: bool,
    newline_after: bool,
    line: bool,
}

struct Formatter<'a> {
    comments: &'a [Comment],
    prefix: &'a str,
}

/// Formats a complete RDL source file into canonical indented form.
pub fn format(input: &str) -> Result<String, Error> {
    format_named(".rdl", input)
}

/// Formats named RDL source so diagnostics identify the original input.
pub fn format_named(name: &str, input: &str) -> Result<String, Error> {
    crate::reader::parse_source(name, input)?;
    let (source, comments, prefix) = extract_comments(input);
    let stream: TokenStream = source.parse().map_err(|error| {
        Error::new(
            &format!("failed to tokenize RDL after preserving comments: {error}"),
            name,
            0,
            0,
        )
    })?;
    let tokens: Vec<TokenTree> = stream.into_iter().collect();
    let mut output = String::new();
    Formatter {
        comments: &comments,
        prefix: &prefix,
    }
    .format_seq(&tokens, &mut output, 0, false);
    trim_line_space(&mut output);
    if !output.ends_with('\n') {
        output.push('\n');
    }
    Ok(output)
}

impl Formatter<'_> {
    fn format_seq(&self, tokens: &[TokenTree], output: &mut String, indent: usize, inline: bool) {
        let mut i = 0;
        let mut angle_depth: usize = 0;
        let multiline = inline
            && tokens.iter().any(|token| {
                let TokenTree::Ident(ident) = token else {
                    return false;
                };
                self.comment(&ident.to_string())
                    .is_some_and(|comment| comment.line || comment.newline_after)
            });

        while i < tokens.len() {
            if let TokenTree::Punct(p) = &tokens[i]
                && p.as_char() == '#'
                && let Some(TokenTree::Group(g)) = tokens.get(i + 1)
                && g.delimiter() == Delimiter::Bracket
            {
                let after_arrow = output.ends_with("-> ");
                let after_colon = output.ends_with(": ");
                let treat_inline = inline || after_arrow || after_colon;
                if !treat_inline && at_line_start(output) {
                    push_indent(output, indent);
                }
                self.format_attribute(g, output);
                if treat_inline {
                    output.push(' ');
                } else {
                    output.push('\n');
                }
                i += 2;
                continue;
            }

            let comment = match &tokens[i] {
                TokenTree::Ident(ident) => self.comment(&ident.to_string()),
                _ => None,
            };
            if comment.is_none() && at_line_start(output) {
                push_indent(output, if inline { indent + 1 } else { indent });
            }

            match &tokens[i] {
                TokenTree::Ident(id) => {
                    let id = id.to_string();
                    if let Some(comment) = comment {
                        self.format_comment(comment, output, indent, inline);
                    } else {
                        output.push_str(&id);
                        output.push(' ');
                    }
                }
                TokenTree::Literal(lit) => {
                    output.push_str(&lit.to_string());
                    output.push(' ');
                }
                TokenTree::Punct(p) => match (p.as_char(), p.spacing()) {
                    (':', Spacing::Joint) => {
                        trim_space(output);
                        output.push_str("::");
                        i += 1;
                    }
                    (':', _) => {
                        trim_space(output);
                        output.push_str(": ");
                    }
                    (',', _) => {
                        trim_space(output);
                        output.push(',');
                        if (inline && !multiline) || angle_depth > 0 {
                            output.push(' ');
                        } else {
                            output.push('\n');
                        }
                    }
                    (';', _) => {
                        trim_space(output);
                        output.push(';');
                        if inline {
                            output.push(' ');
                        } else {
                            output.push('\n');
                        }
                    }
                    ('.', Spacing::Alone) => {
                        trim_space(output);
                        output.push('.');
                    }
                    ('.', _) => output.push('.'),
                    ('-', Spacing::Joint) => {
                        output.push_str("-> ");
                        i += 1;
                    }
                    ('-', _) => output.push('-'),
                    ('*', _) => output.push('*'),
                    ('&', _) => output.push('&'),
                    ('+', _) => {
                        trim_space(output);
                        output.push_str(" + ");
                    }
                    ('=', _) => {
                        trim_space(output);
                        output.push_str(" = ");
                    }
                    ('|', _) => {
                        trim_space(output);
                        output.push_str(" | ");
                    }
                    ('<', _) => {
                        trim_space(output);
                        output.push('<');
                        angle_depth = angle_depth.saturating_add(1);
                    }
                    ('>', _) => {
                        trim_space(output);
                        output.push('>');
                        angle_depth = angle_depth.saturating_sub(1);
                    }
                    _ => output.push(p.as_char()),
                },
                TokenTree::Group(g) => match g.delimiter() {
                    Delimiter::Brace => {
                        let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                        if inner.is_empty() {
                            output.push_str("{}\n");
                        } else {
                            if !output.ends_with(' ') {
                                output.push(' ');
                            }
                            output.push_str("{\n");
                            self.format_seq(&inner, output, indent + 1, false);
                            trim_line_space(output);
                            if !at_line_start(output) {
                                output.push('\n');
                            }
                            push_indent(output, indent);
                            output.push('}');
                            if matches!(tokens.get(i + 1), Some(TokenTree::Punct(p)) if p.as_char() == ',')
                            {
                                output.push_str(",\n");
                                i += 1;
                            } else {
                                output.push('\n');
                            }
                        }
                    }
                    Delimiter::Parenthesis => {
                        trim_space(output);
                        output.push('(');
                        let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                        self.format_seq(&inner, output, indent, true);
                        trim_space(output);
                        if at_line_start(output) {
                            push_indent(output, indent);
                        }
                        output.push_str(") ");
                    }
                    Delimiter::Bracket => {
                        output.push('[');
                        let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                        self.format_seq(&inner, output, indent, true);
                        trim_space(output);
                        if at_line_start(output) {
                            push_indent(output, indent);
                        }
                        output.push(']');
                    }
                    _ => {}
                },
            }

            i += 1;
        }
    }

    fn format_attribute(&self, group: &Group, output: &mut String) {
        output.push_str("#[");
        let inner: Vec<TokenTree> = group.stream().into_iter().collect();
        if matches!(&inner[..], [TokenTree::Ident(ident)] if ident == "r#in") {
            output.push_str("in]");
            return;
        }
        self.format_seq(&inner, output, 0, true);
        trim_space(output);
        output.push(']');
    }

    fn comment(&self, ident: &str) -> Option<&Comment> {
        ident
            .strip_prefix(self.prefix)?
            .parse::<usize>()
            .ok()
            .and_then(|index| self.comments.get(index))
    }

    fn format_comment(&self, comment: &Comment, output: &mut String, indent: usize, inline: bool) {
        if comment.trailing && output.ends_with('\n') {
            output.pop();
        } else if !comment.trailing && !at_line_start(output) {
            trim_space(output);
            output.push('\n');
        }
        if at_line_start(output) {
            push_indent(output, if inline { indent + 1 } else { indent });
        } else if !output.ends_with([' ', '\n']) {
            output.push(' ');
        }
        output.push_str(&comment.text);
        if comment.line || comment.newline_after {
            output.push('\n');
        } else {
            output.push(' ');
        }
    }
}

fn extract_comments(input: &str) -> (String, Vec<Comment>, String) {
    let mut prefix = "__RDL_FORMAT_COMMENT_".to_string();
    while input.contains(&prefix) {
        prefix.push('_');
    }

    let bytes = input.as_bytes();
    let mut source = String::with_capacity(input.len());
    let mut comments = vec![];
    let mut copied = 0;
    let mut i = 0;

    while i < bytes.len() {
        if let Some(end) = quoted_end(bytes, i) {
            i = end;
            continue;
        }
        let line = bytes[i] == b'/' && bytes.get(i + 1) == Some(&b'/');
        let block = bytes[i] == b'/' && bytes.get(i + 1) == Some(&b'*');
        if !line && !block {
            i += 1;
            continue;
        }

        source.push_str(&input[copied..i]);
        let start = i;
        if line {
            i += 2;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
        } else {
            i = block_comment_end(bytes, i);
        }

        let previous_line = &input[..start]
            .rsplit_once('\n')
            .map_or(input[..start].as_ref(), |(_, line)| line);
        let trailing = !previous_line.trim().is_empty();
        let newline_after = input[i..]
            .chars()
            .take_while(|ch| *ch == ' ' || *ch == '\t' || *ch == '\r' || *ch == '\n')
            .any(|ch| ch == '\n');
        let index = comments.len();
        comments.push(Comment {
            text: input[start..i].to_string(),
            trailing,
            newline_after,
            line,
        });
        source.push(' ');
        source.push_str(&prefix);
        source.push_str(&index.to_string());
        source.push(' ');
        copied = i;
    }
    source.push_str(&input[copied..]);
    (source, comments, prefix)
}

fn quoted_end(bytes: &[u8], start: usize) -> Option<usize> {
    match bytes[start] {
        b'"' => Some(string_end(bytes, start + 1, b'"')),
        b'\'' => char_end(bytes, start),
        b'r' => raw_string_end(bytes, start),
        b'b' if bytes.get(start + 1) == Some(&b'"') => Some(string_end(bytes, start + 2, b'"')),
        b'b' if bytes.get(start + 1) == Some(&b'r') => raw_string_end(bytes, start + 1),
        _ => None,
    }
}

fn string_end(bytes: &[u8], mut i: usize, quote: u8) -> usize {
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => i = (i + 2).min(bytes.len()),
            byte if byte == quote => return i + 1,
            _ => i += 1,
        }
    }
    bytes.len()
}

fn char_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut i = start + 1;
    if bytes.get(i) == Some(&b'\\') {
        i += 2;
    } else {
        let ch = std::str::from_utf8(bytes.get(i..)?).ok()?.chars().next()?;
        i += ch.len_utf8();
    }
    (bytes.get(i) == Some(&b'\'')).then_some(i + 1)
}

fn raw_string_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut i = start + 1;
    let mut hashes = 0;
    while bytes.get(i) == Some(&b'#') {
        hashes += 1;
        i += 1;
    }
    if bytes.get(i) != Some(&b'"') {
        return None;
    }
    i += 1;
    while i < bytes.len() {
        if bytes[i] == b'"' && (0..hashes).all(|offset| bytes.get(i + 1 + offset) == Some(&b'#')) {
            return Some(i + 1 + hashes);
        }
        i += 1;
    }
    Some(bytes.len())
}

fn block_comment_end(bytes: &[u8], start: usize) -> usize {
    let mut i = start + 2;
    let mut depth = 1;
    while i < bytes.len() {
        if bytes[i] == b'/' && bytes.get(i + 1) == Some(&b'*') {
            depth += 1;
            i += 2;
        } else if bytes[i] == b'*' && bytes.get(i + 1) == Some(&b'/') {
            depth -= 1;
            i += 2;
            if depth == 0 {
                return i;
            }
        } else {
            i += 1;
        }
    }
    bytes.len()
}

fn at_line_start(output: &str) -> bool {
    output.ends_with('\n') || output.is_empty()
}

fn push_indent(output: &mut String, level: usize) {
    for _ in 0..level {
        output.push_str("    ");
    }
}

fn trim_space(output: &mut String) {
    if output.ends_with(' ') {
        output.pop();
    }
}

fn trim_line_space(output: &mut String) {
    while output.ends_with([' ', '\t']) {
        output.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_direction_uses_rdl_spelling() {
        assert_eq!(
            format("#[win32] mod Test { struct Value { #[r#in] field: i32 } }").unwrap(),
            "#[win32]\nmod Test {\n    struct Value {\n        #[in]\n        field: i32\n    }\n}\n"
        );
    }

    #[test]
    fn comments_are_preserved_and_idempotent() {
        let input = r#"
/// Root documentation.
#[win32]
mod Test {
// Before the structure.
#[First]
// Between attributes.
#[Second]
struct Value {
first: i32, // Trailing field comment.
/* Between fields. */
second: i32,
// Before the closing brace.
}
interface IValue {
fn Method(
// Before a parameter.
first: i32,
second: i32 /* Inline block comment. */
);
}
}
"#;
        let formatted = format_named("comments.rdl", input).unwrap();
        assert!(formatted.contains("/// Root documentation."));
        assert!(formatted.contains("// Before the structure."));
        assert!(
            formatted.contains("#[First]\n    // Between attributes.\n    #[Second]"),
            "{formatted}"
        );
        assert!(
            formatted.contains("first: i32, // Trailing field comment."),
            "{formatted}"
        );
        assert!(formatted.contains("/* Between fields. */"));
        assert!(formatted.contains("// Before the closing brace."));
        assert!(formatted.contains("// Before a parameter."));
        assert!(formatted.contains("second: i32 /* Inline block comment. */"));
        assert_eq!(format_named("comments.rdl", &formatted).unwrap(), formatted);
    }

    #[test]
    fn comment_markers_inside_strings_are_ignored() {
        let input = r#"
#[win32]
mod Test {
#[Message("https://example.test/a/*b*/")]
struct Value {}
}
"#;
        let (_, comments, _) = extract_comments(input);
        assert!(comments.is_empty());
    }

    #[test]
    fn invalid_source_returns_a_named_diagnostic() {
        let input = "#[win32] mod Test { struct Value { field: } }";
        let error = format_named("invalid.rdl", input).unwrap_err();
        assert_eq!(error.file_name, "invalid.rdl");
        assert_ne!(error.line, 0);
    }
}
