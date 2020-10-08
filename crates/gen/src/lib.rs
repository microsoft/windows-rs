mod format_ident;
mod to_snake;
mod type_limits;
mod type_namespaces;
mod type_reader;
mod type_tree;
pub mod gen;
mod winmd;
mod method_kind;

use format_ident::format_ident;
use to_snake::to_snake;
pub use type_limits::{NamespaceTypes, TypeLimit, TypeLimits};
pub use type_reader::TypeReader;
pub use type_tree::TypeTree;
pub use gen::*;
use winmd::*;
use method_kind::MethodKind;
