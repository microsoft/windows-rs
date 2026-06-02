#![deny(clippy::correctness)]

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::useless_transmute,
    clippy::missing_transmute_annotations,
    clippy::missing_safety_doc,
    clippy::too_many_arguments
)]
mod bindings;

mod bitmap;
mod color;
mod device;
mod device_lost;
mod geometry;
#[cfg(feature = "reactor")]
mod reactor;
mod session;
mod swap_chain;
mod text;
mod types;

pub use bitmap::Bitmap;
pub use color::Color;
pub use device::GpuDevice;
pub use geometry::{Path, PathBuilder};
#[cfg(feature = "reactor")]
pub use reactor::{DrawContext, animated_canvas};
pub use session::DrawingSession;
pub use swap_chain::SwapChain;
pub use text::{FontWeight, ParagraphAlignment, TextAlignment, TextFormat};
pub use types::{
    Brush, CapStyle, DashStyle, Ellipse, GradientStop, LineJoin, LinearGradient, Paint,
    RadialGradient, Rect, RoundedRect, StrokeStyle, StrokeStyleBuilder,
};

pub use windows_core::{Error, Result};
pub use windows_numerics::{Matrix3x2, Vector2};
