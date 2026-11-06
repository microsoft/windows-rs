//! Integration tests for windows-canvas.
//!
//! These tests create real D3D11/D2D devices and verify the core API surface.
//! They require a GPU (hardware or WARP) to run.

#[cfg(test)]
#[allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::all
)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "\\bindings.rs"));
}

#[cfg(test)]
mod tests {
    use super::bindings::*;
    use windows_canvas::*;
    use windows_core::Interface;
    use windows_text::{ParagraphAlignment, TextAlignment, TextFormat, TextLayout, WordWrapping};

    fn ensure_com_initialized() {
        unsafe {
            windows_core::link!("ole32.dll" "system" fn CoIncrementMTAUsage(pcookie: *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
            let mut cookie = core::ptr::null_mut();
            let _ = CoIncrementMTAUsage(&mut cookie);
        }
    }

    #[test]
    fn create_device() {
        let device = GpuDevice::new_warp().expect("Failed to create WARP device");
        let _d3d = device.d3d_device();
        let _d2d = device.d2d_device();
        let _factory = device.dxgi_factory();
    }

    #[test]
    fn clone_shares_underlying_device() {
        use windows_core::Interface;
        // A clone shares the same COM devices, so one GpuDevice can drive many
        // independent surfaces from a single device instead of one per surface.
        let device = GpuDevice::new_warp().unwrap();
        let shared = device.clone();

        assert_eq!(device.d3d_device().as_raw(), shared.d3d_device().as_raw());
        assert_eq!(device.d2d_device().as_raw(), shared.d2d_device().as_raw());

        let a = device.create_swap_chain(64, 64).unwrap();
        let b = shared.create_swap_chain(32, 32).unwrap();
        assert_eq!(a.width(), 64);
        assert_eq!(b.width(), 32);
    }

    #[test]
    fn create_swap_chain() {
        let device = GpuDevice::new_warp().unwrap();
        let chain = device.create_swap_chain(64, 64).unwrap();
        assert_eq!(chain.width(), 64);
        assert_eq!(chain.height(), 64);
    }

    #[test]
    fn draw_and_present() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        {
            let session = chain.begin_draw().unwrap();
            session.clear(ColorF::CORNFLOWER_BLUE);
        }

        chain.present().unwrap();
    }

    #[test]
    fn resize_swap_chain() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();
        chain.resize(128, 128).unwrap();
        assert_eq!(chain.width(), 128);
        assert_eq!(chain.height(), 128);
    }

    #[test]
    fn brush_reuse_across_frames() {
        // No compositor consumes these frames; draw off-screen instead of queuing presents.
        let device = GpuDevice::new_warp().unwrap();
        let target = device.create_render_target(64, 64).unwrap();
        let mut brush = None;

        for _ in 0..3 {
            target
                .draw(|session| {
                    let brush = brush
                        .get_or_insert_with(|| session.create_solid_brush(ColorF::RED).unwrap());
                    session.clear(ColorF::BLACK);
                    session.fill_ellipse(&Ellipse::circle(Vector2::new(32.0, 32.0), 16.0), brush);
                    Ok(())
                })
                .unwrap();
            let pixels = target.read_pixels().unwrap();
            let center = (32 * 64 + 32) * 4;
            assert_eq!(&pixels[center..center + 4], &[0, 0, 255, 255]);
            assert_eq!(&pixels[..4], &[0, 0, 0, 255]);
        }
    }

    #[test]
    fn text_format_and_draw_text() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(128, 64).unwrap();

        let format = TextFormat::new("Segoe UI", 16.0).with_alignment(TextAlignment::Center);
        let brush = chain.create_solid_brush(ColorF::WHITE).unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.draw_text("Hello", &format, &Rect::new(0.0, 0.0, 128.0, 64.0), &brush);
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn path_builder_typestate() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let path = PathBuilder::new(&device)
            .unwrap()
            .begin(Vector2::new(32.0, 0.0))
            .line_to(Vector2::new(64.0, 64.0))
            .line_to(Vector2::new(0.0, 64.0))
            .close()
            .build()
            .unwrap();

        let brush = chain.create_solid_brush(ColorF::GREEN).unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.fill_path(&path, &brush);
        session.draw_path(&path, &brush, 2.0);
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn present_reports_no_device_lost() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        drop(session);

        let result = chain.present().unwrap();
        assert!(result, "Expected present to succeed (no device lost)");
        assert!(!chain.is_device_lost());
    }

    #[test]
    fn rounded_rect_draw_fill() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();
        let brush = chain.create_solid_brush(ColorF::RED).unwrap();

        let rrect = RoundedRect::uniform(Rect::new(5.0, 5.0, 55.0, 55.0), 8.0);

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.fill_rounded_rect(&rrect, &brush);
        session.draw_rounded_rect(&rrect, &brush, 2.0);
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn transform_set_get_restore() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);

        // Default is identity.
        let identity = session.transform();
        assert_eq!(identity.m11, 1.0);
        assert_eq!(identity.m22, 1.0);
        assert_eq!(identity.m31, 0.0);

        let translated = Matrix3x2 {
            m11: 1.0,
            m12: 0.0,
            m21: 0.0,
            m22: 1.0,
            m31: 10.0,
            m32: 20.0,
        };
        session.set_transform(&translated);
        let got = session.transform();
        assert_eq!(got.m31, 10.0);
        assert_eq!(got.m32, 20.0);

        // with_transform restores original.
        let scaled = Matrix3x2 {
            m11: 2.0,
            m12: 0.0,
            m21: 0.0,
            m22: 2.0,
            m31: 0.0,
            m32: 0.0,
        };
        session.with_transform(&scaled, || {
            let inside = session.transform();
            assert_eq!(inside.m11, 2.0);
        });
        let after = session.transform();
        assert_eq!(after.m31, 10.0); // restored to translated

        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn linear_gradient_brush() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);

        let gradient = session
            .create_linear_gradient(
                Vector2::new(0.0, 0.0),
                Vector2::new(64.0, 0.0),
                &[
                    GradientStop::new(0.0, ColorF::RED),
                    GradientStop::new(1.0, ColorF::BLUE),
                ],
            )
            .unwrap();

        session.fill_rect(&Rect::new(0.0, 0.0, 64.0, 32.0), &gradient);
        session.fill_ellipse(&Ellipse::circle(Vector2::new(32.0, 48.0), 12.0), &gradient);

        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn radial_gradient_brush() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);

        let gradient = session
            .create_radial_gradient(
                Vector2::new(32.0, 32.0),
                32.0,
                32.0,
                &[
                    GradientStop::new(0.0, ColorF::WHITE),
                    GradientStop::new(1.0, ColorF::BLACK),
                ],
            )
            .unwrap();

        session.fill_rect(&Rect::new(0.0, 0.0, 64.0, 64.0), &gradient);

        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn load_and_draw_bitmap() {
        ensure_com_initialized();
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let bitmap = chain
            .load_bitmap(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test.png"))
            .unwrap();

        assert!(bitmap.width() > 0.0);
        assert!(bitmap.height() > 0.0);

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.draw_bitmap(&bitmap, &Rect::new(0.0, 0.0, 64.0, 64.0), 1.0);
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn load_bitmap_nonexistent_file_returns_err() {
        ensure_com_initialized();
        let device = GpuDevice::new_warp().unwrap();
        let chain = device.create_swap_chain(64, 64).unwrap();

        let result = chain.load_bitmap("nonexistent_file_that_does_not_exist.png");
        assert!(result.is_err());
    }

    #[test]
    fn resize_zero_is_noop() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();
        chain.resize(0, 0).unwrap();
        assert_eq!(chain.width(), 64);
        assert_eq!(chain.height(), 64);
    }

    #[test]
    fn path_builder_begin_hollow_and_end_open() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let path = PathBuilder::new(&device)
            .unwrap()
            .begin_hollow(Vector2::new(0.0, 32.0))
            .line_to(Vector2::new(32.0, 0.0))
            .line_to(Vector2::new(64.0, 32.0))
            .end_open()
            .build()
            .unwrap();

        let brush = chain.create_solid_brush(ColorF::WHITE).unwrap();
        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.draw_path(&path, &brush, 2.0);
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn path_builder_multiple_figures() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        // Two separate closed triangles in one path.
        let path = PathBuilder::new(&device)
            .unwrap()
            .begin(Vector2::new(0.0, 0.0))
            .line_to(Vector2::new(30.0, 0.0))
            .line_to(Vector2::new(15.0, 30.0))
            .close()
            .begin(Vector2::new(34.0, 34.0))
            .line_to(Vector2::new(64.0, 34.0))
            .line_to(Vector2::new(49.0, 64.0))
            .close()
            .build()
            .unwrap();

        let brush = chain.create_solid_brush(ColorF::GREEN).unwrap();
        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.fill_path(&path, &brush);
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn bold_text() {
        let format = TextFormat::new("Segoe UI", 20.0).with_weight(windows_text::FontWeight::BOLD);

        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(128, 64).unwrap();
        let brush = chain.create_solid_brush(ColorF::WHITE).unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.draw_text("Bold", &format, &Rect::new(0.0, 0.0, 128.0, 64.0), &brush);
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn radial_gradient_with_fill_ellipse() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);

        let gradient = session
            .create_radial_gradient(
                Vector2::new(32.0, 32.0),
                32.0,
                32.0,
                &[
                    GradientStop::new(0.0, ColorF::WHITE),
                    GradientStop::new(1.0, ColorF::BLACK),
                ],
            )
            .unwrap();

        session.fill_ellipse(&Ellipse::circle(Vector2::new(32.0, 32.0), 30.0), &gradient);

        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn draw_line_and_styled() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();
        let brush = chain.create_solid_brush(ColorF::WHITE).unwrap();

        let style = device
            .create_stroke_style(
                &StrokeStyleBuilder::new()
                    .caps(CapStyle::Round)
                    .line_join(LineJoin::Round),
            )
            .unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.draw_line(
            Vector2::new(5.0, 5.0),
            Vector2::new(55.0, 55.0),
            &brush,
            2.0,
        );
        session.draw_line_styled(
            Vector2::new(5.0, 55.0),
            Vector2::new(55.0, 5.0),
            &brush,
            3.0,
            &style,
        );
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn draw_rect_and_styled() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();
        let brush = chain.create_solid_brush(ColorF::RED).unwrap();

        let style = device
            .create_stroke_style(
                &StrokeStyleBuilder::new()
                    .dash_style(DashStyle::Dash)
                    .dash_cap(CapStyle::Square),
            )
            .unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.draw_rect(&Rect::new(5.0, 5.0, 30.0, 30.0), &brush, 1.0);
        session.draw_rect_styled(&Rect::new(34.0, 34.0, 59.0, 59.0), &brush, 2.0, &style);
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn draw_ellipse_and_styled() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();
        let brush = chain.create_solid_brush(ColorF::GREEN).unwrap();

        let style = device
            .create_stroke_style(
                &StrokeStyleBuilder::new()
                    .start_cap(CapStyle::Triangle)
                    .end_cap(CapStyle::Flat),
            )
            .unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.draw_ellipse(
            &Ellipse::new(Vector2::new(32.0, 32.0), 25.0, 15.0),
            &brush,
            1.0,
        );
        session.draw_ellipse_styled(
            &Ellipse::circle(Vector2::new(32.0, 32.0), 20.0),
            &brush,
            2.0,
            &style,
        );
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn draw_rounded_rect_styled() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();
        let brush = chain.create_solid_brush(ColorF::WHITE).unwrap();

        let style = device
            .create_stroke_style(
                &StrokeStyleBuilder::new()
                    .dash_style(DashStyle::DashDot)
                    .miter_limit(5.0),
            )
            .unwrap();

        let rrect = RoundedRect::new(Rect::new(5.0, 5.0, 55.0, 55.0), 10.0, 5.0);

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.draw_rounded_rect_styled(&rrect, &brush, 2.0, &style);
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn draw_path_styled() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();
        let brush = chain.create_solid_brush(ColorF::WHITE).unwrap();

        let style = device
            .create_stroke_style(
                &StrokeStyleBuilder::new()
                    .caps(CapStyle::Round)
                    .dash_style(DashStyle::Dot)
                    .dash_offset(0.5),
            )
            .unwrap();

        let path = PathBuilder::new(&device)
            .unwrap()
            .begin(Vector2::new(10.0, 10.0))
            .line_to(Vector2::new(54.0, 10.0))
            .line_to(Vector2::new(54.0, 54.0))
            .close()
            .build()
            .unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.draw_path_styled(&path, &brush, 2.0, &style);
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn stroke_style_builder_all_setters() {
        let device = GpuDevice::new_warp().unwrap();

        let style = device
            .create_stroke_style(
                &StrokeStyleBuilder::new()
                    .start_cap(CapStyle::Round)
                    .end_cap(CapStyle::Square)
                    .dash_cap(CapStyle::Triangle)
                    .line_join(LineJoin::Bevel)
                    .miter_limit(4.0)
                    .dash_style(DashStyle::DashDot)
                    .dash_offset(1.5),
            )
            .unwrap();

        // StrokeStyle implements Clone.
        let _clone = style.clone();
        drop(style);
    }

    #[test]
    fn bitmap_target_shadow_effect() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);

        let target = session.create_bitmap_target().unwrap();
        session.with_target(&target, || {
            let Ok(brush) = session.create_solid_brush(ColorF::WHITE) else {
                return;
            };
            session.clear(ColorF::TRANSPARENT);
            session.fill_ellipse(&Ellipse::circle(Vector2::new(32.0, 32.0), 10.0), &brush);
        });

        let shadow = session.create_shadow(&target).unwrap();
        session.draw_effect(&shadow);
        session.draw_image(&target);

        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn session_load_bitmap() {
        ensure_com_initialized();
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);

        let bitmap = session
            .load_bitmap(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test.png"))
            .unwrap();

        assert!(bitmap.width() > 0.0);
        assert!(bitmap.height() > 0.0);

        session.draw_bitmap(&bitmap, &Rect::new(0.0, 0.0, 64.0, 64.0), 1.0);

        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn create_bitmap_from_bytes() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);

        // A 2x2 premultiplied-BGRA image (opaque red, green, blue, white).
        let pixels: [u8; 16] = [
            0, 0, 255, 255, // red
            0, 255, 0, 255, // green
            255, 0, 0, 255, // blue
            255, 255, 255, 255, // white
        ];
        let bitmap = session.create_bitmap(&pixels, 2, 2).unwrap();

        assert_eq!(bitmap.width(), 2.0);
        assert_eq!(bitmap.height(), 2.0);

        session.draw_bitmap(&bitmap, &Rect::new(0.0, 0.0, 32.0, 32.0), 1.0);

        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn create_bitmap_with_alpha_modes() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let session = chain.begin_draw().unwrap();
        let pixels: [u8; 4] = [10, 20, 30, 128];

        for alpha in [AlphaMode::Premultiplied, AlphaMode::Ignore] {
            let bitmap = session
                .create_bitmap_with_alpha(&pixels, 1, 1, alpha)
                .unwrap();
            assert_eq!(bitmap.width(), 1.0);
            assert_eq!(bitmap.height(), 1.0);
        }

        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn create_bitmap_wrong_length_errors() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let session = chain.begin_draw().unwrap();
        // 3 bytes is not enough for a 1x1 BGRA pixel (needs 4).
        let pixels = [0u8, 0, 0];
        assert!(session.create_bitmap(&pixels, 1, 1).is_err());

        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn create_bitmap_zero_dimension_errors() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let session = chain.begin_draw().unwrap();
        assert!(session.create_bitmap(&[], 0, 4).is_err());
        assert!(session.create_bitmap(&[], 4, 0).is_err());

        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn ellipse_new() {
        let e = Ellipse::new(Vector2::new(10.0, 20.0), 30.0, 40.0);
        assert_eq!(e.center.x, 10.0);
        assert_eq!(e.center.y, 20.0);
        assert_eq!(e.radius_x, 30.0);
        assert_eq!(e.radius_y, 40.0);
    }

    #[test]
    fn rounded_rect_new() {
        let rr = RoundedRect::new(Rect::new(0.0, 0.0, 100.0, 50.0), 10.0, 20.0);
        assert_eq!(rr.radius_x, 10.0);
        assert_eq!(rr.radius_y, 20.0);
        assert_eq!(rr.rect.width(), 100.0);
    }

    #[test]
    fn text_format_with_weight_and_paragraph_alignment() {
        let format = TextFormat::new("Segoe UI", 18.0)
            .with_weight(windows_text::FontWeight::BOLD)
            .with_paragraph_alignment(ParagraphAlignment::Center);

        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(128, 64).unwrap();
        let brush = chain.create_solid_brush(ColorF::WHITE).unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.draw_text(
            "Centered",
            &format,
            &Rect::new(0.0, 0.0, 128.0, 64.0),
            &brush,
        );
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn path_bezier_to() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();
        let brush = chain.create_solid_brush(ColorF::WHITE).unwrap();

        let path = PathBuilder::new(&device)
            .unwrap()
            .begin(Vector2::new(0.0, 32.0))
            .bezier_to(
                Vector2::new(10.0, 0.0),
                Vector2::new(54.0, 0.0),
                Vector2::new(64.0, 32.0),
            )
            .close()
            .build()
            .unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.fill_path(&path, &brush);
        session.draw_path(&path, &brush, 1.0);
        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn color_constructors() {
        let c1 = ColorF::new(0.5, 0.6, 0.7, 0.8);
        assert_eq!(c1.r, 0.5);
        assert_eq!(c1.a, 0.8);

        let c2 = ColorF::rgb(1.0, 0.0, 0.0);
        assert_eq!(c2.a, 1.0);

        let c3 = ColorF::from_rgba8(255, 128, 0, 255);
        assert_eq!(c3.r, 1.0);
        assert!(c3.g > 0.49 && c3.g < 0.51);

        let c4 = ColorF::from_rgb8(0, 0, 0);
        assert_eq!(c4.r, 0.0);
        assert_eq!(c4.a, 1.0);
    }

    #[test]
    fn device_accessors() {
        let device = GpuDevice::new_warp().unwrap();
        let _d2d_factory = device.d2d_factory();
    }

    #[test]
    fn swap_chain_interop_and_load_bitmap() {
        ensure_com_initialized();
        let device = GpuDevice::new_warp().unwrap();
        let chain = device.create_swap_chain(64, 64).unwrap();

        let borrowed: &IDXGISwapChain1 = chain.as_interface();
        let queried: IDXGISwapChain1 = chain.to_interface().unwrap();
        assert_eq!(borrowed.as_raw(), queried.as_raw());
        let mut desc = DXGI_SWAP_CHAIN_DESC1::default();
        unsafe { borrowed.GetDesc1(&mut desc).unwrap() };
        assert_eq!((desc.Width, desc.Height), (64, 64));

        let bitmap = chain
            .load_bitmap(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test.png"))
            .unwrap();
        assert!(bitmap.width() > 0.0);
    }

    #[test]
    fn session_interop_and_create_brush() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let session = chain.begin_draw().unwrap();
        let context: &ID2D1DeviceContext = session.as_interface();
        let size = unsafe { context.GetPixelSize() };
        assert_eq!((size.width, size.height), (64, 64));

        // create_solid_brush on session (vs swap chain).
        let brush = session.create_solid_brush(ColorF::RED).unwrap();
        session.fill_rect(&Rect::new(0.0, 0.0, 64.0, 64.0), &brush);

        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn path_interop() {
        let device = GpuDevice::new_warp().unwrap();
        let path = PathBuilder::new(&device)
            .unwrap()
            .begin(Vector2::new(0.0, 0.0))
            .line_to(Vector2::new(10.0, 0.0))
            .close()
            .build()
            .unwrap();

        let geometry: &ID2D1PathGeometry1 = path.as_interface();
        let mut count = 0;
        unsafe { geometry.GetFigureCount(&mut count).unwrap() };
        assert_eq!(count, 1);
    }

    #[test]
    fn path_fill_contains_point() {
        let device = GpuDevice::new_warp().unwrap();

        // Triangle: apex at (32, 0), base from (0, 64) to (64, 64).
        let path = PathBuilder::new(&device)
            .unwrap()
            .begin(Vector2::new(32.0, 0.0))
            .line_to(Vector2::new(64.0, 64.0))
            .line_to(Vector2::new(0.0, 64.0))
            .close()
            .build()
            .unwrap();

        assert!(path.fill_contains_point(Vector2::new(32.0, 50.0)));
        assert!(!path.fill_contains_point(Vector2::new(5.0, 5.0)));
    }

    #[test]
    fn path_stroke_contains_point() {
        let device = GpuDevice::new_warp().unwrap();

        let path = PathBuilder::new(&device)
            .unwrap()
            .begin_hollow(Vector2::new(10.0, 32.0))
            .line_to(Vector2::new(54.0, 32.0))
            .end_open()
            .build()
            .unwrap();

        assert!(path.stroke_contains_point(Vector2::new(32.0, 32.0), 4.0));
        assert!(!path.stroke_contains_point(Vector2::new(32.0, 50.0), 4.0));
    }

    #[test]
    fn path_compute_bounds() {
        let device = GpuDevice::new_warp().unwrap();

        // Axis-aligned rectangle 10,20 -> 40,50.
        let path = PathBuilder::new(&device)
            .unwrap()
            .begin(Vector2::new(10.0, 20.0))
            .line_to(Vector2::new(40.0, 20.0))
            .line_to(Vector2::new(40.0, 50.0))
            .line_to(Vector2::new(10.0, 50.0))
            .close()
            .build()
            .unwrap();

        let bounds = path.compute_bounds();
        assert!((bounds.left - 10.0).abs() < 0.5);
        assert!((bounds.top - 20.0).abs() < 0.5);
        assert!((bounds.right - 40.0).abs() < 0.5);
        assert!((bounds.bottom - 50.0).abs() < 0.5);
    }

    #[test]
    fn path_polygon() {
        let device = GpuDevice::new_warp().unwrap();

        let path = PathBuilder::new(&device)
            .unwrap()
            .polygon([
                Vector2::new(32.0, 0.0),
                Vector2::new(64.0, 64.0),
                Vector2::new(0.0, 64.0),
            ])
            .unwrap();

        assert!(path.fill_contains_point(Vector2::new(32.0, 50.0)));
        assert!(!path.fill_contains_point(Vector2::new(5.0, 5.0)));
    }

    #[test]
    fn path_polygon_empty_errors() {
        let device = GpuDevice::new_warp().unwrap();
        let result = PathBuilder::new(&device).unwrap().polygon([]);
        assert!(result.is_err());
    }

    // Device-lost classification is pure logic (no GPU needed). The codes below
    // are the canonical DXGI/Direct2D HRESULTs, written independently of the
    // crate's own constants so the test isn't a tautology.
    #[test]
    fn is_device_lost_classifies_known_codes() {
        for code in [
            0x887A0005_u32, // DXGI_ERROR_DEVICE_REMOVED
            0x887A0007,     // DXGI_ERROR_DEVICE_RESET
            0x887A0006,     // DXGI_ERROR_DEVICE_HUNG
            0x887A0020,     // DXGI_ERROR_DRIVER_INTERNAL_ERROR
            0x8899000C,     // D2DERR_RECREATE_TARGET
        ] {
            let hr = windows_core::HRESULT(code as i32);
            assert!(is_device_lost(hr), "{code:#X} should be device-lost");
        }
    }

    #[test]
    fn is_device_lost_rejects_other_codes() {
        for code in [
            0x0000_0000_u32, // S_OK
            0x8007_0057,     // E_INVALIDARG
            0x8000_4005,     // E_FAIL
            0x8007_000E,     // E_OUTOFMEMORY
        ] {
            let hr = windows_core::HRESULT(code as i32);
            assert!(!is_device_lost(hr), "{code:#X} should not be device-lost");
        }
    }

    #[test]
    fn check_device_lost_handles_ok_and_err() {
        let ok: Result<i32> = Ok(7);
        assert!(!check_device_lost(&ok));

        let lost: Result<i32> = Err(windows_core::Error::from_hresult(windows_core::HRESULT(
            0x887A0005_u32 as i32, // DXGI_ERROR_DEVICE_REMOVED
        )));
        assert!(check_device_lost(&lost));

        let other: Result<i32> = Err(windows_core::Error::from_hresult(windows_core::HRESULT(
            0x8007_0057_u32 as i32, // E_INVALIDARG
        )));
        assert!(!check_device_lost(&other));
    }

    #[test]
    fn new_or_warp_produces_working_device() {
        let device = GpuDevice::new_or_warp().unwrap();
        let chain = device.create_swap_chain(64, 64).unwrap();
        let _swap_chain: &IDXGISwapChain1 = chain.as_interface();
    }

    #[test]
    fn set_dpi_recreates_target_and_still_draws() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();
        chain.set_dpi(192.0, 192.0);
        {
            let session = chain.begin_draw().unwrap();
            session.clear(ColorF::WHITE);
        }
        chain.present().unwrap();
    }

    #[test]
    fn set_composition_scale_is_applied() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();
        chain.set_composition_scale(2.0, 2.0);
        {
            let session = chain.begin_draw().unwrap();
            session.clear(ColorF::BLACK);
        }
        chain.present().unwrap();
    }

    #[test]
    fn borrowed_session_offset_composes() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();
        let outer = chain.begin_draw().unwrap();

        let offset = Matrix3x2::translation(10.0, 20.0);
        let session = DrawingSession::from_borrowed_context(
            outer.as_interface::<ID2D1DeviceContext>(),
            offset,
        );

        // The offset is applied to the context but hidden from callers: an
        // untouched session reports the identity transform even though the
        // context is really translated by the atlas offset.
        let seen = session.transform();
        assert_eq!((seen.m31, seen.m32), (0.0, 0.0));
        assert_eq!((seen.m11, seen.m22), (1.0, 1.0));

        // A caller transform composes on top of the offset and round-trips.
        session.set_transform(&Matrix3x2::translation(5.0, 0.0));
        let seen = session.transform();
        assert_eq!((seen.m31, seen.m32), (5.0, 0.0));

        // with_transform restores the previous caller transform (offset intact).
        session.with_transform(&Matrix3x2::translation(100.0, 100.0), || {});
        let seen = session.transform();
        assert_eq!((seen.m31, seen.m32), (5.0, 0.0));
    }

    // Dropping a borrowed session must not end drawing on the borrowed context;
    // the owning swap-chain session ends it and still presents successfully.
    #[test]
    fn borrowed_session_does_not_end_draw() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        {
            let outer = chain.begin_draw().unwrap();
            outer.clear(ColorF::BLACK);
            {
                let session = DrawingSession::from_borrowed_context(
                    outer.as_interface::<ID2D1DeviceContext>(),
                    Matrix3x2::translation(0.0, 0.0),
                );
                session.clear(ColorF::CORNFLOWER_BLUE);
            }
            // If the borrowed drop had called EndDraw, the outer session's own
            // EndDraw would fail; presenting below confirms it did not.
        }

        chain.present().unwrap();
    }

    #[test]
    fn render_target_readback_roundtrip() {
        let device = GpuDevice::new_warp().unwrap();
        let target = device.create_render_target(4, 4).unwrap();
        assert_eq!(target.width(), 4);
        assert_eq!(target.height(), 4);

        target
            .draw(|session| {
                session.clear(ColorF::RED);
                Ok(())
            })
            .unwrap();

        let pixels = target.read_pixels().unwrap();
        // 4x4 BGRA, tightly packed.
        assert_eq!(pixels.len(), 4 * 4 * 4);
        // Opaque red, premultiplied BGRA => B=0, G=0, R=255, A=255.
        #[allow(clippy::chunks_exact_to_as_chunks)]
        for pixel in pixels.chunks_exact(4) {
            assert_eq!(pixel, [0, 0, 255, 255]);
        }
    }

    #[test]
    fn render_target_readback_spatial() {
        let device = GpuDevice::new_warp().unwrap();
        let target = device.create_render_target(8, 8).unwrap();

        target
            .draw(|session| {
                session.clear(ColorF::BLACK);
                let brush = session.create_solid_brush(ColorF::GREEN)?;
                // Fill the left half with green, leaving the right half black.
                session.fill_rect(&Rect::new(0.0, 0.0, 4.0, 8.0), &brush);
                Ok(())
            })
            .unwrap();

        let pixels = target.read_pixels().unwrap();
        let pixel_at = |x: usize, y: usize| {
            let start = (y * 8 + x) * 4;
            &pixels[start..start + 4]
        };
        // Left half: opaque green => B=0, G=255, R=0, A=255.
        assert_eq!(pixel_at(1, 4), [0, 255, 0, 255]);
        // Right half: opaque black => B=0, G=0, R=0, A=255.
        assert_eq!(pixel_at(6, 4), [0, 0, 0, 255]);
    }

    #[test]
    fn render_target_zero_dimension_errors() {
        let device = GpuDevice::new_warp().unwrap();
        assert!(device.create_render_target(0, 4).is_err());
        assert!(device.create_render_target(4, 0).is_err());
    }

    #[test]
    fn text_layout_draws() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(256, 64).unwrap();

        let format = TextFormat::new("Segoe UI", 20.0).with_alignment(TextAlignment::Center);
        let layout = TextLayout::new("Layout", &format, 256.0, 64.0);
        let brush = chain.create_solid_brush(ColorF::WHITE).unwrap();

        let session = chain.begin_draw().unwrap();
        session.clear(ColorF::BLACK);
        session.draw_text_layout(Vector2::new(0.0, 0.0), &layout, &brush);
        drop(session);
        chain.present().unwrap();
    }

    // Pixels covered by white text drawn on black.
    struct Ink {
        width: usize,
        coverage: Vec<u8>,
    }

    impl Ink {
        fn render(width: u32, height: u32, layout: &TextLayout, origin: Vector2) -> Self {
            let device = GpuDevice::new_warp().unwrap();
            let target = device.create_render_target(width, height).unwrap();
            target
                .draw(|session| {
                    session.clear(ColorF::BLACK);
                    let brush = session.create_solid_brush(ColorF::WHITE)?;
                    session.draw_text_layout(origin, layout, &brush);
                    Ok(())
                })
                .unwrap();
            #[allow(clippy::chunks_exact_to_as_chunks)]
            let coverage = target
                .read_pixels()
                .unwrap()
                .chunks_exact(4)
                .map(|p| p[0].max(p[1]).max(p[2]))
                .collect();
            Self {
                width: width as usize,
                coverage,
            }
        }

        // The smallest pixel rectangle containing every lit pixel.
        fn bounds(&self) -> Rect {
            let mut bounds: Option<(usize, usize, usize, usize)> = None;
            for (i, _) in self.coverage.iter().enumerate().filter(|(_, c)| **c > 0) {
                let (x, y) = (i % self.width, i / self.width);
                let b = bounds.get_or_insert((x, y, x, y));
                *b = (b.0.min(x), b.1.min(y), b.2.max(x + 1), b.3.max(y + 1));
            }
            let (left, top, right, bottom) = bounds.unwrap();
            Rect::new(left as f32, top as f32, right as f32, bottom as f32)
        }

        fn total(&self) -> u32 {
            self.coverage.iter().map(|c| *c as u32).sum()
        }
    }

    fn text_layout(text: &str, format: &TextFormat, width: f32) -> TextLayout {
        TextLayout::new(text, format, width, 100.0)
    }

    #[test]
    fn text_ink_matches_layout_metrics() {
        let format = TextFormat::new("Segoe UI", 24.0);
        let layout = text_layout("Hello", &format, 200.0);
        let origin = Vector2::new(10.0, 5.0);
        let ink = Ink::render(220, 60, &layout, origin).bounds();
        let metrics = layout.metrics();
        let left = origin.x + metrics.left;
        let right = left + metrics.width;
        let top = origin.y + metrics.top;
        let bottom = top + metrics.height;

        // Glyphs stay inside the line box, apart from antialiasing at the edges.
        assert!(
            ink.left >= left - 1.0 && ink.right <= right + 1.0,
            "{ink:?}"
        );
        assert!(
            ink.top >= top - 1.0 && ink.bottom <= bottom + 1.0,
            "{ink:?}"
        );

        // Side bearings are small, so the ink spans most of the reported width.
        assert!(ink.right - ink.left > metrics.width - 6.0, "{ink:?}");

        // The trailing caret sits just past the last glyph.
        let caret = layout.caret_bounds(5, false);
        assert!(
            (origin.x + caret.left - ink.right).abs() <= 3.0,
            "{caret:?} {ink:?}"
        );
    }

    #[test]
    fn text_ink_follows_origin() {
        let format = TextFormat::new("Segoe UI", 20.0);
        let layout = text_layout("Move", &format, 100.0);
        let a = Ink::render(140, 60, &layout, Vector2::new(0.0, 0.0)).bounds();
        let b = Ink::render(140, 60, &layout, Vector2::new(20.0, 10.0)).bounds();
        assert_eq!(b.left - a.left, 20.0);
        assert_eq!(b.top - a.top, 10.0);
        assert_eq!(b.right - b.left, a.right - a.left);
        assert_eq!(b.bottom - b.top, a.bottom - a.top);
    }

    #[test]
    fn text_ink_follows_alignment() {
        let ink = |alignment| {
            let format = TextFormat::new("Segoe UI", 20.0).with_alignment(alignment);
            let layout = text_layout("Align", &format, 200.0);
            Ink::render(200, 40, &layout, Vector2::new(0.0, 0.0)).bounds()
        };
        let leading = ink(TextAlignment::Leading);
        let center = ink(TextAlignment::Center);
        let trailing = ink(TextAlignment::Trailing);

        assert!(leading.left < 4.0, "{leading:?}");
        assert!(trailing.right > 196.0, "{trailing:?}");
        let middle = (center.left + center.right) / 2.0;
        assert!((middle - 100.0).abs() <= 3.0, "{center:?}");
    }

    #[test]
    fn text_ink_wraps_onto_lines() {
        let format = TextFormat::new("Segoe UI", 20.0).with_word_wrapping(WordWrapping::Wrap);
        let wide = text_layout("one two three", &format, 300.0);
        let narrow = text_layout("one two three", &format, 60.0);
        assert_eq!(wide.metrics().line_count, 1);
        let lines = narrow.metrics().line_count;
        assert!(lines >= 2);

        let ink = Ink::render(100, 120, &narrow, Vector2::new(0.0, 0.0)).bounds();
        let metrics = narrow.metrics();
        assert!(ink.right <= 61.0, "{ink:?}");
        assert!(ink.bottom <= metrics.height + 1.0, "{ink:?}");

        // The last line has ink.
        let line_height = metrics.height / lines as f32;
        assert!(ink.bottom > metrics.height - line_height, "{ink:?}");
    }

    #[test]
    fn bold_text_has_more_ink() {
        let normal = TextFormat::new("Segoe UI", 24.0);
        let bold = TextFormat::new("Segoe UI", 24.0).with_weight(windows_text::FontWeight::BOLD);
        let origin = Vector2::new(0.0, 0.0);
        let normal = Ink::render(200, 40, &text_layout("Weight", &normal, 200.0), origin);
        let bold = Ink::render(200, 40, &text_layout("Weight", &bold, 200.0), origin);
        assert!(bold.total() > normal.total() * 5 / 4);
    }

    #[test]
    fn underline_and_strikethrough_have_ink() {
        let format = TextFormat::new("Segoe UI", 24.0);
        let origin = Vector2::new(0.0, 0.0);
        let plain = Ink::render(200, 40, &text_layout("aaaa", &format, 200.0), origin).bounds();

        let mut underlined = text_layout("aaaa", &format, 200.0);
        underlined.set_underline(.., true);
        let underlined = Ink::render(200, 40, &underlined, origin).bounds();
        assert!(underlined.bottom > plain.bottom, "{underlined:?} {plain:?}");

        // The spaces between the glyphs have no ink until they are struck through.
        let mut struck = text_layout("a    a", &format, 200.0);
        let spaces = struck.range_bounds(2..4)[0];
        let lit = |ink: &Ink| {
            let (left, right) = (spaces.left as usize, spaces.right as usize);
            ink.coverage
                .chunks_exact(ink.width)
                .enumerate()
                .filter(|(_, row)| row[left..right].iter().all(|c| *c > 0))
                .map(|(y, _)| y as f32)
                .collect::<Vec<_>>()
        };
        assert!(lit(&Ink::render(200, 40, &struck, origin)).is_empty());
        struck.set_strikethrough(1..5, true);
        let rows = lit(&Ink::render(200, 40, &struck, origin));
        assert!(!rows.is_empty());
        // The line crosses the glyphs above the baseline.
        let baseline = struck.line_metrics()[0].baseline;
        assert!(
            rows.iter().all(|y| *y > spaces.top && *y < baseline),
            "{rows:?} {baseline}"
        );
    }

    #[test]
    fn range_weight_has_more_ink() {
        let format = TextFormat::new("Segoe UI", 24.0);
        let origin = Vector2::new(0.0, 0.0);
        let ink = |bold: std::ops::Range<usize>| {
            let mut layout = text_layout("WeightWeight", &format, 300.0);
            layout.set_font_weight(bold, windows_text::FontWeight::BOLD);
            Ink::render(300, 40, &layout, origin).total()
        };
        let normal = ink(0..0);
        let half = ink(6..12);
        let bold = ink(0..12);
        assert!(normal < half && half < bold, "{normal} {half} {bold}");
    }
}
