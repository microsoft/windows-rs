#![doc = include_str!("../readme.md")]

#[expect(
    non_snake_case,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::too_many_arguments,
    clippy::missing_transmute_annotations
)]
mod bindings;
mod factory;
mod fallback;
mod font;
mod format;
mod layout;
mod types;

use bindings::*;
use factory::{factory, localized, memory_loader, user_locale, wide};
use windows_core::*;

pub use fallback::*;
pub use font::*;
pub use format::*;
pub use layout::*;
pub use types::*;

pub use windows_core::Result;
pub use windows_numerics::Vector2;
