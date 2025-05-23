mod file;
pub use file::*;

mod item;
pub use item::*;

mod item_struct;
pub use item_struct::*;

mod item_interface;
pub use item_interface::*;

mod item_enum;
pub use item_enum::*;

mod method;
pub use method::*;

syn::custom_keyword!(interface);
