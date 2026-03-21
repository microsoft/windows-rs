/// Formats the output of `TokenStream::to_string()` into properly indented RDL.
///
/// This replaces the previous `logos`-based implementation with a simple
/// hand-written character-level scanner that produces identical output without
/// any additional dependencies.
#[derive(Debug, PartialEq)]
enum Token {
    Attribute(String),
    CloseBrace,
    CloseBracket,
    CloseParenthesis,
    Colon,
    ColonColon,
    Comma,
    Dot,
    DotDotDot,
    Equals,
    Ampersand,
    Arrow,
    Asterisk,
    Hyphen,
    Plus,
    LessThan,
    GreaterThan,
    Semicolon,
    OpenBrace,
    OpenBracket,
    OpenParenthesis,
    StringLiteral(String),
    Word(String),
}

fn tokenize(input: &str) -> Vec<Token> {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut tokens = Vec::new();
    let mut i = 0;

    while i < len {
        match bytes[i] {
            b' ' | b'\t' | b'\n' | b'\r' => {
                i += 1;
            }
            b'#' => {
                // Attribute: # followed by optional whitespace then [...]
                let start = i;
                i += 1;
                while i < len && bytes[i].is_ascii_whitespace() {
                    i += 1;
                }
                if i < len && bytes[i] == b'[' {
                    i += 1;
                    let mut in_str = false;
                    while i < len {
                        match bytes[i] {
                            b'"' => {
                                in_str = !in_str;
                                i += 1;
                            }
                            b']' if !in_str => {
                                i += 1;
                                break;
                            }
                            _ => {
                                i += 1;
                            }
                        }
                    }
                    tokens.push(Token::Attribute(input[start..i].to_string()));
                }
            }
            b'"' => {
                let start = i;
                i += 1;
                while i < len {
                    if bytes[i] == b'\\' && i + 1 < len {
                        i += 2; // skip escaped character (e.g. `\"`)
                    } else if bytes[i] == b'"' {
                        i += 1;
                        break;
                    } else {
                        i += 1;
                    }
                }
                tokens.push(Token::StringLiteral(input[start..i].to_string()));
            }
            b'.' => {
                if i + 2 < len && bytes[i + 1] == b'.' && bytes[i + 2] == b'.' {
                    tokens.push(Token::DotDotDot);
                    i += 3;
                } else {
                    tokens.push(Token::Dot);
                    i += 1;
                }
            }
            b':' => {
                if i + 1 < len && bytes[i + 1] == b':' {
                    tokens.push(Token::ColonColon);
                    i += 2;
                } else {
                    tokens.push(Token::Colon);
                    i += 1;
                }
            }
            b'-' => {
                if i + 1 < len && bytes[i + 1] == b'>' {
                    tokens.push(Token::Arrow);
                    i += 2;
                } else {
                    tokens.push(Token::Hyphen);
                    i += 1;
                }
            }
            b'{' => {
                tokens.push(Token::OpenBrace);
                i += 1;
            }
            b'}' => {
                tokens.push(Token::CloseBrace);
                i += 1;
            }
            b'(' => {
                tokens.push(Token::OpenParenthesis);
                i += 1;
            }
            b')' => {
                tokens.push(Token::CloseParenthesis);
                i += 1;
            }
            b'[' => {
                tokens.push(Token::OpenBracket);
                i += 1;
            }
            b']' => {
                tokens.push(Token::CloseBracket);
                i += 1;
            }
            b';' => {
                tokens.push(Token::Semicolon);
                i += 1;
            }
            b',' => {
                tokens.push(Token::Comma);
                i += 1;
            }
            b'<' => {
                tokens.push(Token::LessThan);
                i += 1;
            }
            b'>' => {
                tokens.push(Token::GreaterThan);
                i += 1;
            }
            b'=' => {
                tokens.push(Token::Equals);
                i += 1;
            }
            b'&' => {
                tokens.push(Token::Ampersand);
                i += 1;
            }
            b'*' => {
                tokens.push(Token::Asterisk);
                i += 1;
            }
            b'+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            _ => {
                // Word (identifier or keyword, including r# raw identifiers)
                let start = i;
                while i < len
                    && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_' || bytes[i] == b'#')
                {
                    i += 1;
                }
                if i > start {
                    tokens.push(Token::Word(input[start..i].to_string()));
                } else {
                    i += 1;
                }
            }
        }
    }

    tokens
}

pub fn format(input: &str) -> String {
    let tokens = tokenize(input);
    let mut output = String::new();
    let mut indent_level: i32 = 0;
    let mut paren_depth: i32 = 0;
    let mut angle_depth: i32 = 0;
    let mut within_brackets = false;
    let mut token_idx = 0;

    while token_idx < tokens.len() {
        let token = &tokens[token_idx];

        if at_line_start(&output) && !matches!(token, Token::CloseBrace) {
            push_indent(&mut output, indent_level);
        }

        match token {
            Token::Word(word) if word == "r#" || word.starts_with("r#") => {
                output.push_str(word);
            }
            Token::Word(word) => {
                output.push_str(word);
                output.push(' ');
            }
            Token::Ampersand => {
                output.push('&');
            }
            Token::Arrow => {
                output.push_str("-> ");
            }
            Token::Asterisk => {
                output.push('*');
            }
            Token::Attribute(attr) => {
                push_attribute(attr, &mut output);
                if paren_depth > 0 {
                    output.push(' ');
                } else {
                    output.push('\n');
                }
            }
            Token::CloseBrace => {
                indent_level -= 1;
                push_indent(&mut output, indent_level);
                output.push('}');
                if matches!(tokens.get(token_idx + 1), Some(Token::Comma)) {
                    output.push_str(",\n");
                    token_idx += 2;
                    continue;
                } else if matches!(tokens.get(token_idx + 1), Some(Token::Semicolon)) {
                    // no newline: stay on same line for `}; N]` array syntax
                } else {
                    output.push('\n');
                }
            }
            Token::CloseParenthesis => {
                trim_space(&mut output);
                output.push_str(") ");
                paren_depth -= 1;
            }
            Token::Colon => {
                trim_space(&mut output);
                output.push_str(": ");
            }
            Token::ColonColon => {
                trim_space(&mut output);
                output.push_str("::");
            }
            Token::Comma => {
                trim_space(&mut output);
                output.push(',');
                if paren_depth > 0 || angle_depth > 0 {
                    output.push(' ');
                } else {
                    output.push('\n');
                }
            }
            Token::Dot => {
                trim_space(&mut output);
                output.push('.');
            }
            Token::DotDotDot => {
                output.push_str("...");
            }
            Token::Equals => {
                trim_space(&mut output);
                output.push_str(" = ");
            }
            Token::Hyphen => {
                output.push('-');
            }
            Token::Plus => {
                trim_space(&mut output);
                output.push_str(" + ");
            }
            Token::OpenBracket => {
                output.push('[');
                within_brackets = true;
            }
            Token::CloseBracket => {
                trim_space(&mut output);
                output.push(']');
                within_brackets = false;
            }
            Token::LessThan => {
                trim_space(&mut output);
                output.push('<');
                angle_depth += 1;
            }
            Token::GreaterThan => {
                trim_space(&mut output);
                output.push('>');
                angle_depth -= 1;
            }
            Token::OpenBrace => {
                if matches!(tokens.get(token_idx + 1), Some(Token::CloseBrace)) {
                    output.push_str("{}\n");
                    token_idx += 2;
                    continue;
                } else {
                    if !output.ends_with(' ') {
                        output.push(' ');
                    }
                    output.push_str("{\n");
                    indent_level += 1;
                }
            }
            Token::OpenParenthesis => {
                paren_depth += 1;
                trim_space(&mut output);
                output.push('(');
            }
            Token::Semicolon => {
                trim_space(&mut output);
                output.push(';');
                if within_brackets {
                    output.push(' ');
                } else {
                    output.push('\n');
                }
            }
            Token::StringLiteral(s) => {
                output.push_str(s);
                output.push(' ');
            }
        }

        token_idx += 1;
    }

    output
}

fn at_line_start(output: &str) -> bool {
    output.ends_with('\n') || output.is_empty()
}

fn push_indent(output: &mut String, indent_level: i32) {
    for _ in 0..indent_level {
        output.push_str("    ");
    }
}

fn trim_space(output: &mut String) {
    if output.ends_with(' ') {
        output.pop();
    }
}

fn push_attribute(attr: &str, output: &mut String) {
    let mut out = String::new();
    let mut in_literal = false;
    let mut space_previously = false;
    let mut after_open_paren = false;
    let mut after_closing_paren = false;
    let mut after_colon_colon = false;

    let mut chars = attr.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_literal = !in_literal;
                out.push(c);
                after_colon_colon = false;
            }
            '[' => {
                if out.ends_with(' ') {
                    out.pop();
                }
                out.push('[');
            }
            '(' => {
                if out.ends_with(' ') {
                    out.pop();
                }
                out.push('(');
                after_open_paren = true;
                after_closing_paren = false;
                after_colon_colon = false;
                space_previously = false;
            }
            ')' => {
                if out.ends_with(' ') {
                    out.pop();
                }
                out.push(c);
                after_open_paren = false;
                after_closing_paren = true;
                after_colon_colon = false;
                space_previously = false;
            }
            ',' => {
                if out.ends_with(' ') {
                    out.pop();
                }
                out.push(',');
                space_previously = false;
                after_colon_colon = false;
            }
            ':' if !in_literal && chars.peek() == Some(&':') => {
                chars.next();
                if out.ends_with(' ') {
                    out.pop();
                }
                out.push_str("::");
                space_previously = false;
                after_open_paren = false;
                after_colon_colon = true;
            }
            c if c.is_whitespace() && !in_literal => {
                if !after_closing_paren
                    && !after_open_paren
                    && !after_colon_colon
                    && !space_previously
                {
                    out.push(' ');
                    space_previously = true;
                }
            }
            _ => {
                out.push(c);
                space_previously = false;
                after_open_paren = false;
                after_colon_colon = false;
            }
        }
    }
    output.push_str(&out);
}
