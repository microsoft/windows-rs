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

    #[token(";")]
    Semicolon,

    #[token("struct")]
    Struct,

    #[regex(r"[\s\t\r\n]+", logos::skip)]
    Whitespace,
}

pub fn format(input: &str) -> String {
    let mut output = String::new();
    let mut indent_level = 0;
    let mut paren_depth = 0;

    let tokens: Vec<_> = Token::lexer(input).spanned().collect();
    let mut i = 0;

    while i < tokens.len() {
        let (token, span) = &tokens[i];

        let token = match token {
            Ok(token) => token,
            Err(_) => {
                emit_error(input, span.start, "unexpected token");
                panic!();
            }
        };

        if at_line_start(&output) && !matches!(token, Token::CloseBrace) {
            push_indent(&mut output, indent_level);
        }

        match token {
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
                push_indent(&mut output, indent_level);
                output.push('}');
                output.push('\n');
            }
            Token::CloseParenthesis => {
                if output.ends_with(", ") {
                    output.truncate(output.len() - 2);
                }
                output.push_str(") ");
                paren_depth -= 1;
            }
            Token::Colon => {
                if output.ends_with(' ') {
                    output.pop();
                }
                output.push_str(": ");
            }
            Token::ColonColon => {
                if output.ends_with(' ') {
                    output.pop();
                }
                output.push_str("::");
            }
            Token::Comma => {
                if output.ends_with(' ') {
                    output.pop();
                }
                output.push(',');
                if paren_depth > 0 {
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
                if output.ends_with(' ') {
                    output.pop();
                }
                output.push_str(" = ");
            }
            Token::Fn => {
                output.push_str("fn ");
            }
            Token::Hyphen => {
                output.push('-');
            }
            Token::Identifier(ident) => {
                output.push_str(ident);
                output.push(' ');
            }
            Token::Mod => {
                output.push_str("mod ");
            }
            Token::OpenBrace => {
                if matches!(tokens.get(i + 1), Some((Ok(Token::CloseBrace), _))) {
                    output.push_str("{}");
                    output.push('\n');
                    i += 2;
                    continue;
                } else {
                    output.push('{');
                    output.push('\n');
                    indent_level += 1;
                }
            }
            Token::OpenParenthesis => {
                paren_depth += 1;
                if output.ends_with(' ') {
                    output.pop();
                }
                output.push('(');
            }
            Token::Semicolon => {
                if output.ends_with(' ') {
                    output.pop();
                }
                output.push(';');
                output.push('\n');
            }
            Token::Struct => {
                output.push_str("struct ");
            }
            Token::Whitespace => {}
        }

        i += 1;
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
            }
            ')' => {
                out.push(c);
                after_open_paren = false;
                after_closing_paren = true;
            }
            ',' => {
                if out.ends_with(' ') {
                    out.pop();
                }
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

fn push_indent(output: &mut String, indent_level: i32) {
    for _ in 0..indent_level {
        output.push_str("    ");
    }
}

fn emit_error(input: &str, pos: usize, msg: &str) {
    let start = input[..pos].rfind('\n').map_or(0, |i| i + 1);
    let end = input[pos..].find('\n').map_or(input.len(), |i| pos + i);
    let line = &input[start..end];
    let col = pos - start;
    eprintln!("{line}");
    eprintln!("{:>col$}^-- {msg}", "", col = col);
}
