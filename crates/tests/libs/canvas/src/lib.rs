//! Integration tests for windows-canvas.
//!
//! These tests create real D3D11/D2D devices and verify the core API surface.
//! They require a GPU (hardware or WARP) to run.

#[cfg(test)]
mod tests {
    use windows_canvas::*;

    fn ensure_com_initialized() {
        unsafe {
            windows_core::link!("combase.dll" "system" fn CoIncrementMTAUsage(pcookie: *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
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
        let identity = session.get_transform();
        assert_eq!(identity.M11, 1.0);
        assert_eq!(identity.M22, 1.0);
        assert_eq!(identity.M31, 0.0);

        // Set a translation.
        let translated = Matrix3x2 {
            M11: 1.0,
            M12: 0.0,
            M21: 0.0,
            M22: 1.0,
            M31: 10.0,
            M32: 20.0,
        };
        session.set_transform(&translated);
        let got = session.get_transform();
        assert_eq!(got.M31, 10.0);
        assert_eq!(got.M32, 20.0);

        // with_transform restores original.
        let scaled = Matrix3x2 {
            M11: 2.0,
            M12: 0.0,
            M21: 0.0,
            M22: 2.0,
            M31: 0.0,
            M32: 0.0,
        };
        session.with_transform(&scaled, || {
            let inside = session.get_transform();
            assert_eq!(inside.M11, 2.0);
        });
        let after = session.get_transform();
        assert_eq!(after.M31, 10.0); // restored to translated

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
}
