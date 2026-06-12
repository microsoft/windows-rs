#[expect(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments
)]
mod bindings;
mod bitmap;
mod color;
mod device;
mod device_lost;
mod effect;
mod geometry;
#[cfg(feature = "reactor")]
mod reactor;
mod session;
mod swap_chain;
mod text;
mod types;

use bindings::*;
use std::cell::Cell;
use std::os::windows::ffi::OsStrExt;
use windows_core::*;

pub use bitmap::Bitmap;
pub use color::ColorF;
pub use device::GpuDevice;
pub use effect::Effect;
pub use geometry::*;
#[cfg(feature = "reactor")]
pub use reactor::{DrawContext, animated_canvas};
pub use session::DrawingSession;
pub use swap_chain::SwapChain;
pub use text::*;
pub use types::*;

pub use windows_core::Result;
pub use windows_numerics::{Matrix3x2, Vector2};
