use proc_macro2::{Delimiter, Group, Spacing, TokenStream, TokenTree};

/// Formats RDL source with indentation and alphabetically ordered attribute names.
/// Repeated attributes retain their relative order.
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
        if attribute(&tokens[i..]).is_some() {
            // An attribute that immediately follows `->` is a return-type
            // attribute and must stay inline (e.g. `-> #[Const] *const u8`).
            // An attribute that immediately follows `: ` is in type position
            // (e.g. an inline nested `field: #[packed(1)] struct { .. }`) and
            // likewise stays inline.
            let after_arrow = output.ends_with("-> ");
            let after_colon = output.ends_with(": ");
            let treat_inline = inline || after_arrow || after_colon;
            let mut attributes = Vec::new();
            while let Some(group) = attribute(&tokens[i..]) {
                let name: String = group
                    .stream()
                    .into_iter()
                    .take_while(|token| match token {
                        TokenTree::Ident(_) => true,
                        TokenTree::Punct(p) => p.as_char() == ':',
                        _ => false,
                    })
                    .map(|token| token.to_string().trim_start_matches("r#").to_string())
                    .collect();
                let mut text = String::new();
                format_attribute(group, &mut text);
                attributes.push((name, text));
                i += 2;
            }
            attributes.sort_by(|left, right| left.0.cmp(&right.0));
            for (_, text) in attributes {
                if !treat_inline && at_line_start(output) {
                    push_indent(output, indent);
                }
                output.push_str(&text);
                output.push(if treat_inline { ' ' } else { '\n' });
            }
            continue;
        }

        fn attribute(tokens: &[TokenTree]) -> Option<&Group> {
            match tokens.get(..2)? {
                [TokenTree::Punct(p), TokenTree::Group(g)]
                    if p.as_char() == '#' && g.delimiter() == Delimiter::Bracket =>
                {
                    Some(g)
                }
                _ => None,
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
    if matches!(&inner[..], [TokenTree::Ident(ident)] if ident == "r#in") {
        output.push_str("in]");
        return;
    }
    // Predicate arguments are expressions, not type arguments or named attribute properties.
    if let [TokenTree::Ident(name), TokenTree::Group(args)] = inner.as_slice()
        && name == "success"
        && args.delimiter() == Delimiter::Parenthesis
    {
        output.push_str("success(");
        output.push_str(&args.stream().to_string());
        output.push_str(")]");
        return;
    }
    if let [TokenTree::Ident(name), TokenTree::Group(args)] = inner.as_slice()
        && matches!(name.to_string().as_str(), "range" | "readable" | "writable")
        && args.delimiter() == Delimiter::Parenthesis
    {
        output.push_str(&name.to_string());
        output.push('(');
        format_expression(args.stream(), output);
        output.push_str(")]");
        return;
    }
    format_seq(&inner, output, 0, true);
    trim_space(output);
    output.push(']');
}

fn format_expression(stream: TokenStream, output: &mut String) {
    let mut operand = false;
    for token in stream {
        match &token {
            TokenTree::Ident(_) | TokenTree::Literal(_) => {
                if operand {
                    output.push(' ');
                }
                output.push_str(&token.to_string());
                operand = true;
            }
            TokenTree::Group(group) if group.delimiter() == Delimiter::Parenthesis => {
                output.push('(');
                format_expression(group.stream(), output);
                output.push(')');
                operand = true;
            }
            TokenTree::Punct(punct) => {
                match punct.as_char() {
                    ',' => output.push_str(", "),
                    '+' | '-' | '*' if operand => {
                        output.push(' ');
                        output.push(punct.as_char());
                        output.push(' ');
                    }
                    _ => output.push(punct.as_char()),
                }
                operand = false;
            }
            _ => output.push_str(&token.to_string()),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_direction_uses_rdl_spelling() {
        assert_eq!(format("#[r#in]"), "#[in]\n");
    }
}
