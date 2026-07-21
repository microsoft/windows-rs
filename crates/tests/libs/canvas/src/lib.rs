//! Integration tests for windows-canvas.
//!
//! These tests create real D3D11/D2D devices and verify the core API surface.
//! They require a GPU (hardware or WARP) to run.

#[cfg(test)]
mod tests {
    use windows_canvas::*;

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
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let brush = chain.create_solid_brush(ColorF::RED).unwrap();

        for _ in 0..3 {
            let session = chain.begin_draw().unwrap();
            session.clear(ColorF::BLACK);
            session.fill_ellipse(&Ellipse::circle(Vector2::new(32.0, 32.0), 16.0), &brush);
            drop(session);
            chain.present().unwrap();
        }
    }

    #[test]
    fn text_format_and_draw_text() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(128, 64).unwrap();

        let format = TextFormat::new("Segoe UI", 16.0)
            .unwrap()
            .with_alignment(TextAlignment::Center);
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

        // Build a triangle path.
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

        // Normal present should return Ok(true) — device is fine.
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

        // Set a translation.
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

        // Use gradient with various draw methods (same as solid brush).
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

        // Build a hollow open path (stroke-only, not closed).
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
    fn text_format_new_bold() {
        let format = TextFormat::new_bold("Segoe UI", 20.0).unwrap();

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

        // Use radial gradient with fill_ellipse (not just fill_rect).
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
    fn rect_from_xywh_and_accessors() {
        let rect = Rect::from_xywh(10.0, 20.0, 30.0, 40.0);
        assert_eq!(rect.left, 10.0);
        assert_eq!(rect.top, 20.0);
        assert_eq!(rect.right, 40.0);
        assert_eq!(rect.bottom, 60.0);
        assert_eq!(rect.width(), 30.0);
        assert_eq!(rect.height(), 40.0);
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
        let format = TextFormat::with_weight("Segoe UI", 18.0, FontWeight::BOLD)
            .unwrap()
            .with_paragraph_alignment(ParagraphAlignment::Center);

        let _raw = format.raw();

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
        let _dwrite = device.dwrite_factory();
    }

    #[test]
    fn swap_chain_raw_and_load_bitmap() {
        ensure_com_initialized();
        let device = GpuDevice::new_warp().unwrap();
        let chain = device.create_swap_chain(64, 64).unwrap();

        let _raw = chain.raw_swap_chain();

        let bitmap = chain
            .load_bitmap(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test.png"))
            .unwrap();
        assert!(bitmap.width() > 0.0);
    }

    #[test]
    fn session_raw_and_create_brush() {
        let device = GpuDevice::new_warp().unwrap();
        let mut chain = device.create_swap_chain(64, 64).unwrap();

        let session = chain.begin_draw().unwrap();
        let _raw = session.raw();

        // create_solid_brush on session (vs swap chain).
        let brush = session.create_solid_brush(ColorF::RED).unwrap();
        session.fill_rect(&Rect::new(0.0, 0.0, 64.0, 64.0), &brush);

        drop(session);
        chain.present().unwrap();
    }

    #[test]
    fn path_raw_accessor() {
        let device = GpuDevice::new_warp().unwrap();
        let path = PathBuilder::new(&device)
            .unwrap()
            .begin(Vector2::new(0.0, 0.0))
            .line_to(Vector2::new(10.0, 0.0))
            .close()
            .build()
            .unwrap();

        let _raw = path.raw();
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

        // Open horizontal segment from (10, 32) to (54, 32).
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

        // Same triangle as path_fill_contains_point, built via the polygon helper.
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
        let _raw = chain.raw_swap_chain();
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
        let session = DrawingSession::from_borrowed_context(outer.raw(), offset);

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
                    outer.raw(),
                    Matrix3x2::translation(0.0, 0.0),
                );
                session.clear(ColorF::CORNFLOWER_BLUE);
            }
            // If the borrowed drop had called EndDraw, the outer session's own
            // EndDraw would fail; presenting below confirms it did not.
        }

        chain.present().unwrap();
    }
}
