#![windows_subsystem = "windows"]

use windows_canvas::{ColorF, Ellipse, GpuDevice, Vector2, device_lost_error};
use windows_reactor::{Element, RenderCx, SwapChainHost, SwapChainHostContent};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let host = cx.use_swap_chain_host_ref::<f32>();
    SwapChainHost::new(
        &host,
        |layout| {
            let device = GpuDevice::new_or_warp()?;
            let mut swap_chain =
                device.create_swap_chain(layout.pixel_width, layout.pixel_height)?;
            swap_chain.set_dpi(96.0 * layout.scale_x, 96.0 * layout.scale_y)?;
            swap_chain.set_composition_scale(layout.scale_x, layout.scale_y)?;
            Ok(SwapChainHostContent::new(0.0_f32, swap_chain))
        },
        |_phase, swap_chain, layout| {
            swap_chain.resize_with_dpi(
                layout.pixel_width,
                layout.pixel_height,
                96.0 * layout.scale_x,
                96.0 * layout.scale_y,
            )?;
            swap_chain.set_composition_scale(layout.scale_x, layout.scale_y)
        },
        |phase, swap_chain, frame| {
            *phase += 0.02;
            let session = swap_chain.begin_draw()?;
            session.clear(ColorF::DARK_SLATE_BLUE);
            let brush = session.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;
            let radius = frame.layout.width.min(frame.layout.height) * 0.2;
            let x = frame.layout.width * (0.5 + phase.sin() * 0.25);
            session.fill_ellipse(
                &Ellipse::circle(Vector2::new(x, frame.layout.height * 0.5), radius),
                &brush,
            );
            drop(session);
            if swap_chain.present()? {
                Ok(())
            } else {
                Err(device_lost_error())
            }
        },
    )
    .continuous(true)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Typed Direct2D Host", app)
}
