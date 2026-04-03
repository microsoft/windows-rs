use proc_macro2::{Delimiter, Group, Spacing, TokenStream, TokenTree};

pub fn format(input: &str) -> String {
    let stream: TokenStream = input.parse().unwrap_or_default();
    let tokens: Vec<TokenTree> = stream.into_iter().collect();
    let mut output = String::new();
    format_seq(&tokens, &mut output, 0, false);
    output
}

fn format_seq(tokens: &[TokenTree], output: &mut String, indent: usize, inline: bool) {
    let mut i = 0;
    let mut angle_depth: usize = 0;

    while i < tokens.len() {
        // Detect outer attribute: `#` followed by a `[...]` group.
        if let TokenTree::Punct(p) = &tokens[i] {
            if p.as_char() == '#' {
                if let Some(TokenTree::Group(g)) = tokens.get(i + 1) {
                    if g.delimiter() == Delimiter::Bracket {
                        // An attribute that immediately follows `->` is a return-type
                        // attribute and must stay inline (e.g. `-> #[Const] *const u8`).
                        let after_arrow = output.ends_with("-> ");
                        let treat_inline = inline || after_arrow;
                        if !treat_inline && at_line_start(output) {
                            push_indent(output, indent);
                        }
                        format_attribute(g, output);
                        if treat_inline {
                            output.push(' ');
                        } else {
                            output.push('\n');
                        }
                        i += 2;
                        continue;
                    }
                }
            }
        }

        if !inline && at_line_start(output) {
            push_indent(output, indent);
        }

        match &tokens[i] {
            TokenTree::Ident(id) => {
                output.push_str(&id.to_string());
                output.push(' ');
            }
            TokenTree::Literal(lit) => {
                output.push_str(&lit.to_string());
                output.push(' ');
            }
            TokenTree::Punct(p) => match (p.as_char(), p.spacing()) {
                (':', Spacing::Joint) => {
                    trim_space(output);
                    output.push_str("::");
                    i += 1; // skip the second `:`
                }
                (':', _) => {
                    trim_space(output);
                    output.push_str(": ");
                }
                (',', _) => {
                    trim_space(output);
                    output.push(',');
                    if inline || angle_depth > 0 {
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
                // Single member-access dot: trim the preceding space.
                // Joint dots are part of `...` (variadic) and must not trim
                // the preceding space so that `, ...` formats correctly.
                ('.', Spacing::Alone) => {
                    trim_space(output);
                    output.push('.');
                }
                ('.', _) => output.push('.'),
                // `->` return-type arrow
                ('-', Spacing::Joint) => {
                    output.push_str("-> ");
                    i += 1; // skip the `>`
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
                        format_seq(&inner, output, indent + 1, false);
                        push_indent(output, indent);
                        output.push('}');
                        if matches!(tokens.get(i + 1), Some(TokenTree::Punct(p)) if p.as_char() == ',')
                        {
                            output.push_str(",\n");
                            i += 1; // skip the `,`
                        } else {
                            output.push('\n');
                        }
                    }
                }
                Delimiter::Parenthesis => {
                    trim_space(output);
                    output.push('(');
                    let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                    format_seq(&inner, output, indent, true);
                    trim_space(output);
                    output.push_str(") ");
                }
                Delimiter::Bracket => {
                    output.push('[');
                    let inner: Vec<TokenTree> = g.stream().into_iter().collect();
                    format_seq(&inner, output, indent, true);
                    trim_space(output);
                    output.push(']');
                }
                _ => {}
            },
        }

        i += 1;
    }
}

fn format_attribute(g: &Group, output: &mut String) {
    output.push_str("#[");
    let inner: Vec<TokenTree> = g.stream().into_iter().collect();
    format_seq(&inner, output, 0, true);
    trim_space(output);
    output.push(']');
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
