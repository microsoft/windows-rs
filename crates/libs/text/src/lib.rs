#![doc = include_str!("../readme.md")]

#[expect(
    non_snake_case,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::too_many_arguments
)]
mod bindings;
mod factory;
mod fallback;
mod font;
mod format;
mod glyph_run;
mod layout;
mod offsets;
mod types;

use bindings::*;
use factory::{factory, localized, memory_loader, user_locale, wide};
use glyph_run::Collector;
use offsets::Offsets;
use windows_core::*;

pub use fallback::*;
pub use font::*;
pub use format::*;
pub use glyph_run::*;
pub use layout::*;
pub use types::*;

pub use windows_core::Result;
pub use windows_numerics::{Rect, Vector2};
