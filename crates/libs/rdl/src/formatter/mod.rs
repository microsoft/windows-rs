use proc_macro2::{Delimiter, Group, Spacing, TokenStream, TokenTree};

/// Format an RDL token stream string into canonical, indented form.
///
/// Uses `proc_macro2` token-tree traversal.  Because `proc_macro2` natively
/// groups `{ }`, `( )`, and `[ ]` into `TokenTree::Group` nodes, the
/// formatter never needs to track brace, parenthesis, or bracket depth
/// manually — it simply recurses into each group at the appropriate
/// indentation level.  The only nesting that still requires explicit
/// tracking is `< >` generics, which Rust's tokeniser does not group.
pub fn format(input: &str) -> String {
    let tokens: TokenStream = input.parse().unwrap_or_default();
    let items: Vec<TokenTree> = tokens.into_iter().collect();
    let mut output = String::new();
    format_seq(&items, &mut output, 0, false);
    output
}

/// Recursively format a slice of token trees.
///
/// * `indent`  – current indentation level (each level is 4 spaces).
/// * `inline`  – when `true` the tokens are inside `( )` or `[ ]` and
///   commas/semicolons are followed by a space rather than a
///   newline.
fn format_seq(tokens: &[TokenTree], output: &mut String, indent: usize, inline: bool) {
    let mut i = 0;
    let mut angle_depth = 0i32;
    // Set when a bare `:` at the top level of a declaration introduces an
    // inheritance/implementation list, e.g. `class Foo: Base, IFace {}`.
    // Cleared when the opening `{` (or a `;`) is consumed.
    let mut in_colon_list = false;

    while i < tokens.len() {
        // ── Outer attribute: `#` followed immediately by `[…]` ──────────
        if let TokenTree::Punct(p) = &tokens[i] {
            if p.as_char() == '#' {
                if let Some(TokenTree::Group(g)) = tokens.get(i + 1) {
                    if g.delimiter() == Delimiter::Bracket {
                        if !inline && at_line_start(output) {
                            push_indent(output, indent);
                        }
                        format_attribute(g, output);
                        if inline {
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

        // ── Add indentation at the start of each new line ────────────────
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
            TokenTree::Punct(p) => {
                match (p.as_char(), p.spacing()) {
                    // `::` path separator — consume both characters at once
                    (':', Spacing::Joint) => {
                        trim_space(output);
                        output.push_str("::");
                        i += 1; // skip the second `:`
                    }
                    // Single `:` — field annotation or inheritance colon
                    (':', _) => {
                        trim_space(output);
                        output.push_str(": ");
                        // When not already inside `( )` / `[ ]` and not
                        // inside `< >`, a bare `:` starts a colon-list when
                        // the next brace at this depth is empty.  This
                        // distinguishes `class Foo: Base, IFace {}` (inline
                        // commas) from `struct Foo { field: Type, }` (each
                        // field on its own line).
                        if !inline && angle_depth == 0 && next_brace_is_empty(tokens, i + 1) {
                            in_colon_list = true;
                        }
                    }
                    (',', _) => {
                        trim_space(output);
                        output.push(',');
                        if inline || angle_depth > 0 || in_colon_list {
                            output.push(' ');
                        } else {
                            output.push('\n');
                        }
                    }
                    (';', _) => {
                        in_colon_list = false;
                        trim_space(output);
                        output.push(';');
                        if inline {
                            output.push(' ');
                        } else {
                            output.push('\n');
                        }
                    }
                    // Single member-access dot: trim trailing space (`foo.bar`).
                    // Joint dots are part of `...` (variadic) and must NOT trim
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
                        angle_depth += 1;
                    }
                    ('>', _) => {
                        trim_space(output);
                        output.push('>');
                        angle_depth -= 1;
                    }
                    _ => output.push(p.as_char()),
                }
            }
            TokenTree::Group(g) => match g.delimiter() {
                Delimiter::Brace => {
                    in_colon_list = false;
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
                        // A `}` followed by `,` occurs when a brace-delimited
                        // type is used inline (e.g. a nested struct field).
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

/// Format an outer attribute bracket group as `#[…]`.
///
/// The content is formatted in inline mode: `::` is collapsed, parenthesised
/// argument lists are compacted, and no newlines are inserted.
fn format_attribute(g: &Group, output: &mut String) {
    output.push_str("#[");
    let inner: Vec<TokenTree> = g.stream().into_iter().collect();
    format_seq(&inner, output, 0, true);
    trim_space(output);
    output.push(']');
}

/// Returns `true` when the first `{ }` group reachable from `from` (at the
/// same angle-bracket depth) has an empty body.
///
/// Used to detect an inheritance-list colon vs a field-annotation colon:
///   `class Foo: Base, IFace {}`   — next brace is empty  → inline commas
///   `struct Foo { field: Type, }` — next brace is not empty → newline commas
fn next_brace_is_empty(tokens: &[TokenTree], from: usize) -> bool {
    let mut angle = 0i32;
    for token in tokens.iter().skip(from) {
        match token {
            TokenTree::Punct(p) if p.as_char() == '<' => angle += 1,
            TokenTree::Punct(p) if p.as_char() == '>' => angle -= 1,
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace && angle == 0 => {
                return g.stream().is_empty();
            }
            // A semicolon means we left the current declaration without finding
            // a brace, so this is not an inheritance colon.
            TokenTree::Punct(p) if p.as_char() == ';' => return false,
            _ => {}
        }
    }
    false
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
