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
#[cfg(feature = "reactor")]
mod reactor;
#[cfg(feature = "reactor")]
#[expect(non_snake_case, clippy::upper_case_acronyms)]
mod reactor_bindings;
mod render_target;
mod session;
mod swap_chain;
mod text;
mod types;

pub use bindings::ID2D1DeviceContext;
use bindings::*;
pub use device_lost::{check_device_lost, device_lost_error, is_device_lost};
use std::cell::Cell;
use std::os::windows::ffi::OsStrExt;
use windows_core::*;

// Temporary diagnostics for the x64 CI hang.
#[expect(
    clippy::explicit_write,
    reason = "Bypass libtest capture so hung tests leave diagnostics."
)]
fn diagnose(stage: &str) {
    use std::io::Write;

    if std::env::var_os("WINDOWS_CANVAS_DIAGNOSTICS").is_none() {
        return;
    }
    static START: std::sync::Once = std::sync::Once::new();
    START.call_once(|| {
        std::thread::spawn(|| {
            std::thread::sleep(std::time::Duration::from_secs(90));
            writeln!(std::io::stderr(), "Canvas diagnostic deadline exceeded").unwrap();
            std::process::abort();
        });
    });
    writeln!(
        std::io::stderr(),
        "Canvas {:?} {:?}: {stage}",
        std::thread::current().id(),
        std::thread::current().name()
    )
    .unwrap();
}

pub use bitmap::Bitmap;
pub use color::ColorF;
#[cfg(feature = "composition")]
pub use composition::CanvasCompositionExt;
pub use device::GpuDevice;
pub use effect::Effect;
pub use geometry::*;
#[cfg(feature = "reactor")]
pub use reactor::*;
pub use render_target::RenderTarget;
pub use session::DrawingSession;
pub use swap_chain::SwapChain;
pub use text::*;
pub use types::*;

pub use windows_core::Result;
pub use windows_numerics::{Matrix3x2, Vector2};
