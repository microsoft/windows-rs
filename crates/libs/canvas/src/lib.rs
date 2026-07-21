#![doc = include_str!("../readme.md")]

#[expect(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::too_many_arguments
)]
mod bindings;
mod bitmap;
mod color;
#[cfg(feature = "composition")]
mod composition;
mod device;
mod device_lost;
mod effect;
mod geometry;
mod render_target;
mod session;
mod swap_chain;
mod text;
mod types;

pub use bindings::ID2D1DeviceContext;
use bindings::*;
pub use device_lost::{check_device_lost, is_device_lost};
use std::cell::Cell;
use std::os::windows::ffi::OsStrExt;
use windows_core::*;

pub use bitmap::Bitmap;
pub use color::ColorF;
#[cfg(feature = "composition")]
pub use composition::CanvasCompositionExt;
pub use device::GpuDevice;
pub use effect::Effect;
pub use geometry::*;
pub use render_target::RenderTarget;
pub use session::DrawingSession;
pub use swap_chain::SwapChain;
pub use text::*;
pub use types::*;

pub use windows_core::Result;
pub use windows_numerics::{Matrix3x2, Vector2};
