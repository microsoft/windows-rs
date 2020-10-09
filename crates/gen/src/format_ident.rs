pub fn format_ident(name: &str) -> squote::Ident {
    if name == "Self" {
        squote::format_ident!("{}_", name)
    } else {
        // keywords list based on https://doc.rust-lang.org/reference/keywords.html
        match name {
            "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" | "crate"
            | "do" | "else" | "enum" | "extern" | "false" | "final" | "fn" | "for" | "if"
            | "impl" | "in" | "let" | "loop" | "macro" | "match" | "mod" | "move" | "mut"
            | "override" | "priv" | "pub" | "ref" | "return" | "Self" | "self" | "static"
            | "struct" | "super" | "trait" | "true" | "type" | "typeof" | "unsafe" | "unsized"
            | "use" | "virtual" | "where" | "while" | "yield" | "try" | "async" | "await"
            | "dyn" => squote::format_ident!("r#{}", name),
            _ => squote::format_ident!("{}", name),
        }
    }
}
