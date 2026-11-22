use super::*;
use std::fmt::Write as _;
use std::io::Write;

impl Config<'_> {
    pub fn format(&self, tokens: TokenStream) -> String {
        let tokens = format_tokens(tokens);
        self.rustfmt(&tokens).unwrap_or(tokens)
    }

    fn rustfmt(&self, tokens: &str) -> Option<String> {
        let mut cmd = std::process::Command::new("rustfmt");
        cmd.stdin(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::null());
        cmd.args(rustfmt_args(self.bindgen.rustfmt.as_deref()));

        let mut child = cmd.spawn().ok()?;
        let mut stdin = child.stdin.take()?;
        let written = stdin.write_all(tokens.as_bytes());
        drop(stdin);
        let output = child.wait_with_output().ok()?;

        if written.is_err() || !output.status.success() {
            return None;
        }

        String::from_utf8(output.stdout).ok()
    }
}

fn rustfmt_args(config: Option<&str>) -> Vec<&str> {
    let mut args = vec!["--edition", "2024", "--config", "newline_style=Unix"];

    if let Some(config) = config {
        args.extend(["--config", config]);
    }

    args
}

// Rustfmt leaves some macro bodies unchanged. Supply readable token spacing before formatting
// rather than rewriting the formatted source, where literal contents must remain untouched.
fn format_tokens(tokens: TokenStream) -> String {
    let mut output = String::new();
    write_tokens(tokens, &mut output, false);
    output
}

fn write_tokens(tokens: TokenStream, output: &mut String, in_macro: bool) {
    use proc_macro2::{Delimiter, Spacing, TokenTree};

    let mut tokens = tokens.into_iter().peekable();
    let mut previous = None;
    let mut joint = false;

    while let Some(token) = tokens.next() {
        let path = matches!(&token, TokenTree::Punct(p) if p.as_char() == ':' && p.spacing() == Spacing::Joint)
            && matches!(tokens.peek(), Some(TokenTree::Punct(p)) if p.as_char() == ':');
        let macro_group = matches!((&previous, &token),
            (Some(TokenTree::Punct(p)), TokenTree::Group(_)) if p.as_char() == '!');
        let compact = match (&previous, &token) {
            (_, TokenTree::Punct(p)) if matches!(p.as_char(), ',' | ';') => in_macro,
            (Some(TokenTree::Ident(_)), TokenTree::Punct(p)) => p.as_char() == '!',
            (_, TokenTree::Group(_)) if macro_group => true,
            (Some(TokenTree::Ident(_)), TokenTree::Group(g)) => {
                in_macro && g.delimiter() == Delimiter::Parenthesis
            }
            (Some(TokenTree::Group(p)), TokenTree::Group(g)) => {
                in_macro
                    && p.delimiter() == Delimiter::Parenthesis
                    && g.delimiter() == Delimiter::Parenthesis
            }
            (Some(TokenTree::Punct(p)), TokenTree::Ident(i)) => {
                in_macro && p.as_char() == '*' && (i == "mut" || i == "const")
            }
            _ => false,
        };

        if previous.is_some() && !joint && !path && !compact {
            output.push(' ');
        }

        if path {
            output.push_str("::");
            tokens.next();
            joint = true;
        } else {
            match &token {
                TokenTree::Group(group) => {
                    let (open, close) = match group.delimiter() {
                        Delimiter::Parenthesis => ("(", ")"),
                        Delimiter::Bracket => ("[", "]"),
                        Delimiter::Brace => ("{ ", " }"),
                        Delimiter::None => ("", ""),
                    };
                    output.push_str(open);
                    write_tokens(group.stream(), output, in_macro || macro_group);
                    output.push_str(close);
                }
                _ => write!(output, "{token}").unwrap(),
            }
            joint = matches!(&token, TokenTree::Punct(p) if p.spacing() == Spacing::Joint);
        }
        previous = Some(token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rustfmt_defaults_to_unix_line_endings() {
        assert_eq!(
            rustfmt_args(None),
            ["--edition", "2024", "--config", "newline_style=Unix"]
        );
        assert_eq!(
            rustfmt_args(Some("newline_style=Windows")),
            [
                "--edition",
                "2024",
                "--config",
                "newline_style=Unix",
                "--config",
                "newline_style=Windows"
            ]
        );
    }

    #[test]
    fn macro_header_collapsed() {
        let input = "windows_core :: link ! (\"user32.dll\" \"system\" fn EnableMouseInPointer (fenable : windows_core :: BOOL) -> windows_core :: BOOL);";
        let expected = "windows_core::link!(\"user32.dll\" \"system\" fn EnableMouseInPointer(fenable : windows_core::BOOL) -> windows_core::BOOL) ;";
        assert_eq!(format_tokens(input.parse().unwrap()), expected);
    }

    #[test]
    fn star_mut_inside_macro() {
        let input = "link ! (fn foo(x : * mut u8) -> ());";
        let expected = "link!(fn foo(x : *mut u8) -> ()) ;";
        assert_eq!(format_tokens(input.parse().unwrap()), expected);
    }

    #[test]
    fn macro_text_in_literals_is_unchanged() {
        for input in [
            r#"const TEXT: &str = "macro ! (call ) , ; :: * mut";"#,
            r##"const TEXT: &str = r#"macro ! (call ) " :: * const"#;"##,
        ] {
            let tokens: TokenStream = input.parse().unwrap();
            let expected = tokens.to_string();
            let formatted: TokenStream = format_tokens(tokens).parse().unwrap();
            assert_eq!(formatted.to_string(), expected);
        }
    }

    #[test]
    fn macro_identifiers_preserve_unicode() {
        let input = "link ! (fn Évaluer(données : * mut u8));";
        let expected = "link!(fn Évaluer(données : *mut u8)) ;";
        assert_eq!(format_tokens(input.parse().unwrap()), expected);
    }

    #[test]
    fn token_boundaries_are_preserved() {
        for input in [
            "fn f<'a>(value: &'a str) -> *const u8;",
            "m!(r#type, 'é', b'\\'', b\"quoted\\\"\", c\"text\");",
            "m!(1 . 0, a > = b, a : : b, b \"text\");",
        ] {
            let tokens: TokenStream = input.parse().unwrap();
            let expected = tokens.to_string();
            let formatted: TokenStream = format_tokens(tokens).parse().unwrap();
            assert_eq!(formatted.to_string(), expected);
        }
    }
}
