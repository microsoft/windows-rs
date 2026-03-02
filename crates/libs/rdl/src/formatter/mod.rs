use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token<'a> {
    #[token("&")]
    Ampersand,

    #[token("->")]
    Arrow,

    #[token("*")]
    Asterisk,

    #[regex(r#"#\s*\[[^\]]*\]"#)]
    Attribute(&'a str),

    #[token("}")]
    CloseBrace,

    #[token(")")]
    CloseParenthesis,

    #[token(":")]
    Colon,

    #[token("::", priority = 2)]
    ColonColon,

    #[token(",")]
    Comma,

    #[token("const")]
    Const,

    #[token("enum")]
    Enum,

    #[token("=")]
    Equals,

    #[token("fn")]
    Fn,

    #[token("-")]
    Hyphen,

    #[regex(r"\w+")]
    Identifier(&'a str),

    #[token("mod")]
    Mod,

    #[token("{")]
    OpenBrace,

    #[token("(")]
    OpenParenthesis,

    #[token("[")]
    OpenBracket,

    #[token("]")]
    CloseBracket,

    #[token(";")]
    Semicolon,

    #[token("<")]
    LessThan,

    #[token(">")]
    GreaterThan,

    #[token("struct")]
    Struct,

    #[token("r#")]
    Raw,

    #[regex(r"[\s\t\r\n]+", logos::skip)]
    Whitespace,
}

pub fn format(input: &str) -> String {
    let mut output = String::new();
    let mut indent_level = 0;
    let mut paren_depth = 0;
    let mut angle_depth = 0;
    let mut within_brackets = false;

    let tokens: Vec<_> = Token::lexer(input).spanned().collect();
    let mut token_idx = 0;

    while token_idx < tokens.len() {
        let (token, span) = &tokens[token_idx];

        let token = match token {
            Ok(token) => token,
            Err(_) => {
                emit_error(input, span.start, "unexpected token");
                panic!();
            }
        };

        if at_line_start(&output) && !matches!(token, Token::CloseBrace) {
            output.push_indent(indent_level);
        }

        match token {
            Token::Raw => {
                output.push_str("r#");
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
                output.push('\n');
            }
            Token::CloseBrace => {
                indent_level -= 1;
                output.push_indent(indent_level);
                output.push('}');
                if matches!(tokens.get(token_idx + 1), Some((Ok(Token::Comma), _))) {
                    output.push_str(",\n");
                    token_idx += 2;
                    continue;
                } else {
                    output.push('\n');
                }
            }
            Token::CloseParenthesis => {
                output.trim_space();
                output.push_str(") ");
                paren_depth -= 1;
            }
            Token::Colon => {
                output.trim_space();
                output.push_str(": ");
            }
            Token::ColonColon => {
                output.trim_space();
                output.push_str("::");
            }
            Token::Comma => {
                output.trim_space();
                output.push(',');
                if paren_depth > 0 || angle_depth > 0 {
                    output.push(' ');
                } else {
                    output.push('\n');
                }
            }
            Token::Const => {
                output.push_str("const ");
            }
            Token::Enum => {
                output.push_str("enum ");
            }
            Token::Equals => {
                output.trim_space();
                output.push_str(" = ");
            }
            Token::Fn => {
                output.push_str("fn ");
            }
            Token::Hyphen => {
                output.push('-');
            }
            Token::OpenBracket => {
                output.push('[');
                within_brackets = true;
            }
            Token::CloseBracket => {
                output.trim_space();
                output.push(']');
                within_brackets = false;
            }
            Token::LessThan => {
                output.trim_space();
                output.push('<');
                angle_depth += 1;
            }
            Token::GreaterThan => {
                output.trim_space();
                output.push('>');
                angle_depth -= 1;
            }
            Token::Identifier(ident) => {
                output.push_str(ident);
                output.push(' ');
            }
            Token::Mod => {
                output.push_str("mod ");
            }
            Token::OpenBrace => {
                if matches!(tokens.get(token_idx + 1), Some((Ok(Token::CloseBrace), _))) {
                    output.push_str("{}");
                    output.push('\n');
                    token_idx += 2;
                    continue;
                } else {
                    if !output.ends_with(' ') {
                        output.push(' ');
                    }
                    output.push('{');
                    output.push('\n');
                    indent_level += 1;
                }
            }
            Token::OpenParenthesis => {
                paren_depth += 1;
                output.trim_space();
                output.push('(');
            }
            Token::Semicolon => {
                output.trim_space();
                output.push(';');

                if within_brackets {
                    output.push(' ');
                } else {
                    output.push('\n');
                }
            }
            Token::Struct => {
                output.push_str("struct ");
            }
            Token::Whitespace => {}
        }

        token_idx += 1;
    }

    output
}

fn at_line_start(output: &str) -> bool {
    output.ends_with('\n') || output.is_empty()
}

fn push_attribute(attr: &str, output: &mut String) {
    let mut out = String::new();
    let mut in_literal = false;
    let mut space_previously = false;
    let mut after_open_paren = false;
    let mut after_closing_paren = false;

    for c in attr.chars() {
        match c {
            '"' => {
                in_literal = !in_literal;
                out.push(c);
            }
            '[' => {
                out.trim_space();
                out.push('[');
            }
            '(' => {
                out.trim_space();
                out.push('(');
                after_open_paren = true;
                after_closing_paren = false;
            }
            ')' => {
                out.push(c);
                after_open_paren = false;
                after_closing_paren = true;
            }
            ',' => {
                out.trim_space();
                out.push(',');
                space_previously = false;
            }
            c if c.is_whitespace() && !in_literal => {
                if !after_closing_paren && !after_open_paren && !space_previously {
                    out.push(' ');
                    space_previously = true;
                }
            }
            _ => {
                out.push(c);
                space_previously = false;
                after_open_paren = false;
            }
        }
    }
    output.push_str(&out);
}

fn emit_error(input: &str, pos: usize, msg: &str) {
    let start = input[..pos].rfind('\n').map_or(0, |i| i + 1);
    let end = input[pos..].find('\n').map_or(input.len(), |i| pos + i);
    let col = pos - start;

    const MAX_LINE_WIDTH: usize = 80;
    const WINDOW_SIZE: usize = 40;

    let line_len = end - start;

    let (line, col, prefix) = if line_len > MAX_LINE_WIDTH {
        let start_offset = col.saturating_sub(WINDOW_SIZE);
        let end_offset = (col + WINDOW_SIZE).min(line_len);
        let prefix = if start_offset > 0 { "... " } else { "" };

        let windowed = input[start + start_offset..start + end_offset].trim_end();
        (windowed, col - start_offset, prefix)
    } else {
        (input[start..end].trim_end(), col, "")
    };

    let padding = " ".repeat(col + prefix.len());
    eprintln!("{}{}", prefix, line);
    eprintln!("{}▲", padding);
    eprintln!("{}└─── {}", padding, msg);
}

trait StringMethods {
    fn push_indent(&mut self, indent_level: i32);
    fn trim_space(&mut self);
}

impl StringMethods for String {
    fn push_indent(&mut self, indent_level: i32) {
        for _ in 0..indent_level {
            self.push_str("    ");
        }
    }

    fn trim_space(&mut self) {
        if self.ends_with(' ') {
            self.pop();
        }
    }
}
