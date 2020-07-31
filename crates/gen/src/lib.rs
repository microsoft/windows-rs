mod blob;
mod case;
mod codes;
mod element_type;
mod file;
mod flags;
mod row;
mod tables;
mod type_limits;
mod type_namespaces;
mod type_reader;
mod type_stage;
mod type_tree;
mod types;

pub mod dependencies;
pub mod load_winmd;
pub use file::WinmdFile;
pub use tables::AttributeArg;
pub use type_limits::{NamespaceTypes, TypeLimit, TypeLimits};
pub use type_reader::TypeReader;
pub use type_stage::TypeStage;

fn format_ident(name: &str) -> squote::Ident {
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

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
