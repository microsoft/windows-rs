pub fn format_ident(name: &str) -> squote::Ident {
    // keywords list based on https://doc.rust-lang.org/reference/keywords.html
    match name {
        "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate" | "do"
        | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if" | "impl" | "in"
        | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut" | "override" | "priv"
        | "pub" | "ref" | "return" | "static" | "struct" | "super" | "trait" | "true" | "type"
        | "typeof" | "unsafe" | "unsized" | "use" | "virtual" | "where" | "while" | "yield"
        | "try" | "async" | "await" | "dyn" => squote::format_ident!("r#{}", name),
        "Self" | "self" => squote::format_ident!("{}_", name),
        "_" => squote::format_ident!("unused"), // TODO: workaround for https://github.com/microsoft/win32metadata/issues/89
        _ => squote::format_ident!("{}", name),
    }
}
