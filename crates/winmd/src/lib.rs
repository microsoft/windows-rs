mod blob;
mod case;
mod codes;
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

pub mod load_winmd;
pub use file::WinmdFile;
pub use type_limits::TypeLimits;
pub use type_reader::TypeReader;
pub use type_stage::TypeStage;

fn format_ident(name: &str) -> proc_macro2::Ident {
    if name == "Self" {
        quote::format_ident!("{}_", name)
    } else {
        quote::format_ident!("r#{}", name)
    }
}

#[cfg(target_pointer_width = "64")]
const SYSTEM32: &str = "System32";

#[cfg(target_pointer_width = "32")]
const SYSTEM32: &str = "SysNative";
