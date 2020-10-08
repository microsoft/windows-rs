mod format_ident;
mod to_snake;
mod type_limits;
mod type_namespaces;
mod type_reader;
mod type_tree;
mod types;
mod winmd;

pub mod dependencies;
use format_ident::format_ident;
use to_snake::to_snake;
pub use type_limits::{NamespaceTypes, TypeLimit, TypeLimits};
pub use type_reader::TypeReader;
pub use type_tree::TypeTree;
pub use types::*;
pub use winmd::AttributeArg;
use winmd::*;
