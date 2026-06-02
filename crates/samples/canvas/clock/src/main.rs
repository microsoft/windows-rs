//! Analog clock rendered with `windows-canvas` inside a reactor window.
//!
//! Compare with `crates/samples/windows/direct2d/src/main.rs` — that sample
//! requires ~350 lines of raw D2D/DXGI/UIAnimation setup. This version uses
//! `animated_canvas()` and draws the same clock in ~60 lines.
//!
//! ## What this demonstrates
//! - Per-frame rendering with `animated_canvas`
//! - Transform composition (rotation × translation) for clock hands
//! - Stroke styles (round start cap, triangle end cap)
//! - Time-based animation (no UIAnimation Manager needed)
//! - `LocalTime` from `windows-time` for local clock readings
//!
//! ## What's missing vs the full D2D sample
//! - Drop shadow effect — not yet in canvas
//! - UIAnimation startup swing — not yet in canvas

#![windows_subsystem = "windows"]

use std::cell::OnceCell;

use windows_canvas::{
    CapStyle, Color, Ellipse, StrokeStyle, StrokeStyleBuilder, Vector2, animated_canvas,
};
use windows_numerics::Matrix3x2;
use windows_reactor::*;

thread_local! {
    static HAND_STYLE: OnceCell<StrokeStyle> = const { OnceCell::new() };
}

fn clock(cx: &mut RenderCx) -> Element {
    let _ = cx;
    animated_canvas(|ctx| {
        ctx.clear(Color::WHITE);

        let size_x = ctx.width;
        let size_y = ctx.height;
        let radius = size_x.min(size_y).max(200.0) / 2.0 - 50.0;

        // Single brush — orange/terracotta at 80% opacity (matching D2D sample).
        let Ok(brush) = ctx.create_solid_brush(Color::new(0.92, 0.38, 0.208, 0.8)) else {
            return;
        };

        // Create stroke style once (round start, triangle end — matches D2D sample).
        let style = HAND_STYLE.with(|cell| {
            cell.get_or_init(|| {
                ctx.device()
                    .create_stroke_style(
                        &StrokeStyleBuilder::new()
                            .start_cap(CapStyle::Round)
                            .end_cap(CapStyle::Triangle),
                    )
                    .unwrap()
            })
            .clone()
        });

        let translation = Matrix3x2::translation(size_x / 2.0, size_y / 2.0);

        // Clock face — ellipse centered at origin, translated to center.
        ctx.with_transform(&translation, || {
            ctx.draw_ellipse(
                &Ellipse::circle(Vector2::zero(), radius),
                &brush,
                radius / 20.0,
            );
        });

        // Current local time → angles in degrees (same math as D2D sample).
        #[repr(C)]
        struct SystemTime {
            _year: u16,
            _month: u16,
            _dow: u16,
            _day: u16,
            hour: u16,
            minute: u16,
            second: u16,
            millis: u16,
        }
        unsafe extern "system" {
            fn GetLocalTime(st: *mut SystemTime);
        }
        let mut st = unsafe { std::mem::zeroed::<SystemTime>() };
        unsafe { GetLocalTime(&mut st) };
        let second_deg = (st.second as f32 + st.millis as f32 / 1000.0) * 6.0;
        let minute_deg = st.minute as f32 * 6.0 + second_deg / 60.0;
        let hour_deg = (st.hour % 12) as f32 * 30.0 + minute_deg / 12.0;

        // Second hand.
        ctx.with_transform(&(Matrix3x2::rotation(second_deg) * translation), || {
            ctx.draw_line_styled(
                Vector2::zero(),
                Vector2::new(0.0, -(radius * 0.75)),
                &brush,
                radius / 25.0,
                &style,
            );
        });

        // Minute hand.
        ctx.with_transform(&(Matrix3x2::rotation(minute_deg) * translation), || {
            ctx.draw_line_styled(
                Vector2::zero(),
                Vector2::new(0.0, -(radius * 0.75)),
                &brush,
                radius / 15.0,
                &style,
            );
        });

        // Hour hand.
        ctx.with_transform(&(Matrix3x2::rotation(hour_deg) * translation), || {
            ctx.draw_line_styled(
                Vector2::zero(),
                Vector2::new(0.0, -(radius * 0.5)),
                &brush,
                radius / 10.0,
                &style,
            );
        });
    })
    .into()
}

fn main() -> Result<()> {
    App::new().title("Canvas Clock").render(clock)
}
