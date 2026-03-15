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

    #[token(".")]
    Dot,

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

    #[regex(r#""[^"]*""#)]
    StringLiteral(&'a str),

    #[token("struct")]
    Struct,

    #[token("+")]
    Plus,

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
    let mut in_colon_list = false;
    // Tracks the most recently seen declaration keyword so that when `{` is
    // processed we can determine whether we are entering a struct/enum body
    // (where field colons must NOT start a colon-list) vs a mod/namespace body
    // (where class/interface declarations live and header colons should).
    let mut last_keyword_is_struct_like = false;
    // Stack of body kinds: `true` = struct/enum body, `false` = mod/namespace body.
    let mut body_stack: Vec<bool> = Vec::new();

    let tokens: Vec<_> = Token::lexer(input).spanned().collect();
    let mut token_idx = 0;

    while token_idx < tokens.len() {
        let (token, _span) = &tokens[token_idx];

        let token = match token {
            Ok(token) => token,
            Err(_) => {
                token_idx += 1;
                continue;
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
                if paren_depth > 0 {
                    output.push(' ');
                } else {
                    output.push('\n');
                }
            }
            Token::CloseBrace => {
                body_stack.pop();
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
                // Only treat this as a class/interface header colon when we are
                // outside parentheses/angles AND not inside a struct/enum body.
                // Struct field colons (`field: Type`) must not trigger a colon-list.
                if paren_depth == 0 && angle_depth == 0 && body_stack.last() != Some(&true) {
                    in_colon_list = true;
                }
            }
            Token::ColonColon => {
                output.trim_space();
                output.push_str("::");
            }
            Token::Comma => {
                output.trim_space();
                output.push(',');
                if paren_depth > 0 || angle_depth > 0 || in_colon_list {
                    output.push(' ');
                } else {
                    output.push('\n');
                }
            }
            Token::Const => {
                output.push_str("const ");
            }
            Token::Dot => {
                output.trim_space();
                output.push('.');
            }
            Token::Enum => {
                last_keyword_is_struct_like = true;
                output.push_str("enum ");
            }
            Token::Equals => {
                output.trim_space();
                output.push_str(" = ");
            }
            Token::Fn => {
                last_keyword_is_struct_like = true;
                output.push_str("fn ");
            }
            Token::Hyphen => {
                output.push('-');
            }
            Token::Plus => {
                output.trim_space();
                output.push_str(" + ");
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
                last_keyword_is_struct_like = false;
                output.push_str("mod ");
            }
            Token::OpenBrace => {
                in_colon_list = false;
                if matches!(tokens.get(token_idx + 1), Some((Ok(Token::CloseBrace), _))) {
                    // Shorthand empty body `{}` — do not push to body_stack.
                    output.push_str("{}");
                    output.push('\n');
                    token_idx += 2;
                    last_keyword_is_struct_like = false;
                    continue;
                } else {
                    body_stack.push(last_keyword_is_struct_like);
                    last_keyword_is_struct_like = false;
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
                in_colon_list = false;
                output.trim_space();
                output.push(';');

                if within_brackets {
                    output.push(' ');
                } else {
                    output.push('\n');
                }
            }
            Token::StringLiteral(literal) => {
                output.push_str(literal);
                output.push(' ');
            }
            Token::Struct => {
                last_keyword_is_struct_like = true;
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
                out.trim_space();
                out.push('[');
            }
            '(' => {
                out.trim_space();
                out.push('(');
                after_open_paren = true;
                after_closing_paren = false;
                after_colon_colon = false;
                space_previously = false;
            }
            ')' => {
                out.trim_space();
                out.push(c);
                after_open_paren = false;
                after_closing_paren = true;
                after_colon_colon = false;
                space_previously = false;
            }
            ',' => {
                out.trim_space();
                out.push(',');
                space_previously = false;
                after_colon_colon = false;
            }
            ':' if !in_literal && chars.peek() == Some(&':') => {
                chars.next(); // consume second ':'
                out.trim_space();
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
