use super::*;

mod class;
mod cpp_const;
mod cpp_delegate;
mod cpp_enum;
mod cpp_fn;
mod cpp_interface;
mod cpp_method;
mod cpp_struct;
mod delegate;
mod r#enum;
mod interface;
mod method;
mod r#struct;

pub use class::*;
pub use cpp_const::*;
pub use cpp_delegate::*;
pub use cpp_enum::*;
pub use cpp_fn::*;
pub use cpp_interface::*;
pub use cpp_method::*;
pub use cpp_struct::*;
pub use delegate::*;
pub use interface::*;
pub use method::*;
pub use r#enum::*;
pub use r#struct::*;

// TODO: maybe just order on Item directly
// 1. order functions first
// 2. order everything else by name
// Otherwise it looks weird when you have things like LOAD_LIBRARY_FLAGS sorting before BOOL

// impl Ord for Item {
//     fn cmp(&self, other: &Self) -> Ordering {
//     }
// }

// impl PartialOrd for Item {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
