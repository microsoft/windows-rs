use super::*;

#[test]
fn initial_inline_hyperlink_focus_preserves_keyboard_navigation() -> Result<()> {
    const HYPERLINK: i32 = 50005;
    process::run("window-focus-hyperlink", |ui| {
        let link = ui.named(HYPERLINK, "Initial link")?;
        let focused = || unsafe {
            ui.client
                .CompareElements(&ui.client.GetFocusedElement()?, &link)
        };
        let _ = ui.wait("inline hyperlink has initial focus", focused, |same| {
            same.as_bool()
        })?;
        ui.name("first-focuses", "0")?;
        ui.press_keys_on(&link, "Initial link", &[0x09])?;
        ui.name("focus", "First Keyboard")?;
        ui.name("first-focuses", "1")?;
        ui.focused("First")?;
        ui.press_keys_on(&ui.element("First")?, "First", &[0x10, 0x09])?;
        let _ = ui.wait(
            "Shift+Tab returns to the inline hyperlink",
            focused,
            |same| same.as_bool(),
        )?;
        Ok(())
    })
}

#[test]
fn initial_focus_preserves_keyboard_navigation_and_reactivation() -> Result<()> {
    process::run("window-focus", |ui| {
        ui.name("focus", "First Pointer")?;
        ui.name("first-focuses", "1")?;
        ui.focused("First")?;
        let first = ui.element("First")?;
        ui.press_keys_on(&first, "First", &[0x09])?;
        ui.name("focus", "Second Keyboard")?;
        ui.focused("Second")?;
        ui.name("activations", "1")?;
        ui.name_stays("focus", "Second Keyboard", Duration::from_millis(250))?;
        ui.focused("Second")?;
        let second = ui.element("Second")?;
        ui.press_keys_on(&second, "Second", &[0x10, 0x09])?;
        ui.name("focus", "First Keyboard")
    })?;
    process::run("window-focus-empty", |ui| ui.name("focus", "Empty"))
}

#[test]
fn window_title_and_client_size_updates() -> Result<()> {
    process::run("window", |ui| {
        let original = ui.element("resize")?;
        ui.window_title("Reactor UI fixture: window")?;
        ui.window_size(480.0, 320.0)?;
        ui.invoke("resize")?;
        ui.window_title("Reactor UI fixture: window updated")?;
        ui.window_size(640.0, 400.0)?;
        ui.invoke("reset")?;
        ui.window_title("Reactor UI fixture: window")?;
        ui.window_size(480.0, 320.0)?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&original, &ui.element("resize")?)?
                    .as_bool(),
                "window updates replaced the native content"
            );
        }
        Ok(())
    })
}

#[test]
fn custom_title_bar_client_size_updates() -> Result<()> {
    for fixture in ["window-frame", "window-frame-policy"] {
        process::run(fixture, |ui| {
            let original = ui.element("resize")?;
            ui.window_size(480.0, 320.0)?;
            ui.invoke("resize")?;
            ui.window_size(640.0, 400.0)?;
            ui.invoke("reset")?;
            ui.window_size(480.0, 320.0)?;
            unsafe {
                assert!(
                    ui.client
                        .CompareElements(&original, &ui.element("resize")?)?
                        .as_bool(),
                    "client sizing replaced the custom title-bar content"
                );
            }
            Ok(())
        })?;
    }
    Ok(())
}

#[test]
fn window_presentation_updates_native_styles() -> Result<()> {
    process::run("window", |ui| {
        let window = ui.window_handle()? as HWND;
        for (index, restricted) in [false, true, false].into_iter().enumerate() {
            if index != 0 {
                ui.invoke("presentation")?;
            }
            ui.wait(
                "native topmost and minimization styles match the declaration",
                || unsafe {
                    Ok((
                        GetWindowLongW(window, GWL_EXSTYLE) & WS_EX_TOPMOST != 0,
                        GetWindowLongW(window, GWL_STYLE) & WS_MINIMIZEBOX != 0,
                    ))
                },
                |styles| *styles == (restricted, !restricted),
            )?;
        }
        Ok(())
    })
}

#[test]
fn window_maximizable_updates_and_restores_native_default() -> Result<()> {
    for fixture in ["window-maximizable", "window-maximizable-frame"] {
        process::run(fixture, |ui| {
            let window = ui.window_handle()? as HWND;
            let original = ui.element("enable-maximize")?;
            for (action, declaration, enabled) in [
                (None, "Some(false)", false),
                (Some("enable-maximize"), "Some(true)", true),
                (Some("disable-maximize"), "Some(false)", false),
                (Some("clear-maximize"), "None", true),
            ] {
                if let Some(action) = action {
                    ui.invoke(action)?;
                }
                ui.name("maximizable", &format!("Maximizable {declaration}"))?;
                ui.wait(
                    "native maximize style matches the declaration without disabling resize or minimize",
                    || unsafe { Ok(GetWindowLongW(window, GWL_STYLE)) },
                    |style| {
                        (style & WS_MAXIMIZEBOX != 0) == enabled
                            && style & WS_THICKFRAME != 0
                            && style & WS_MINIMIZEBOX != 0
                    },
                )?;
                let double_click = || unsafe {
                    // Wait for native caption handling, including the disabled no-op case.
                    let mut result = 0;
                    assert_ne!(
                        SendMessageTimeoutW(
                            window,
                            WM_NCLBUTTONDBLCLK as u32,
                            HTCAPTION as usize,
                            0,
                            SMTO_ABORTIFHUNG as u32 | SMTO_BLOCK as u32,
                            5000,
                            &mut result,
                        ),
                        0,
                        "native caption double-click timed out"
                    );
                };
                assert!(!unsafe { IsZoomed(window) }.as_bool());
                double_click();
                assert_eq!(
                    unsafe { IsZoomed(window) }.as_bool(),
                    enabled,
                    "caption double-click ignored the maximizable setting"
                );
                if enabled {
                    double_click();
                    assert!(!unsafe { IsZoomed(window) }.as_bool());
                }
                assert_eq!(ui.window_handle()? as HWND, window);
                assert!(
                    unsafe {
                        ui.client
                            .CompareElements(&original, &ui.element("enable-maximize")?)?
                    }
                    .as_bool(),
                    "changing maximizable replaced the native content"
                );
            }
            Ok(())
        })?;
    }
    Ok(())
}

#[test]
fn window_icon_set_and_clear() -> Result<()> {
    process::run("window", |ui| {
        ui.window_icon(false)?;
        ui.invoke("set-icon")?;
        ui.window_icon(true)?;
        ui.invoke("clear-icon")?;
        ui.window_icon(false)
    })
}

#[test]
fn title_bar_existing_control_tracks_layout_changes() -> Result<()> {
    process::run("window-titlebar-initial", |ui| {
        drag_title_bar_slider(ui)?;
        let slider = ui.element("slider")?;
        let original_left = unsafe { slider.CurrentBoundingRectangle()?.left };
        ui.invoke("move")?;
        ui.wait(
            "existing slider moves within the title bar",
            || unsafe { Ok(slider.CurrentBoundingRectangle()?.left) },
            |left| *left != original_left,
        )?;
        assert!(
            unsafe { ui.client.CompareElements(&slider, &ui.element("slider")?)? }.as_bool(),
            "moving the slider replaced its native control"
        );
        drag_title_bar_slider(ui)
    })
}

#[test]
fn title_bar_refreshes_hit_testing_after_content_changes() -> Result<()> {
    process::run("window-titlebar-dynamic", |ui| {
        ui.element("drag-region")?;
        for _ in 0..2 {
            ui.invoke("show")?;
            drag_title_bar_slider(ui)?;
            ui.invoke("hide")?;
            let empty = ui.element("drag-region")?;
            let _physical = PhysicalCoordinates::new()?;
            let window = ui.window_handle()? as HWND;
            let mut before = RECT::default();
            unsafe { GetWindowRect(window, &mut before).ok()? };
            let thread = unsafe { GetWindowThreadProcessId(window, std::ptr::null_mut()) };
            ui.pointer_move_to(&empty, "drag-region", 0.5, 0.5)?;
            ui.pointer_button(true)?;
            ui.wait(
                "title-bar press enters the native window-move loop",
                || unsafe {
                    let mut info = GUITHREADINFO {
                        cbSize: size_of::<GUITHREADINFO>() as u32,
                        ..Default::default()
                    };
                    GetGUIThreadInfo(thread, &mut info).ok()?;
                    Ok((info.flags, info.hwndMoveSize))
                },
                |(flags, moving)| flags & GUI_INMOVESIZE as u32 != 0 && *moving == window,
            )?;
            ui.pointer_move_to(&empty, "drag-region", 0.8, 0.5)?;
            ui.pointer_button(false)?;
            // Outline-only window dragging commits the new bounds on mouse release.
            ui.wait(
                "removed slider leaves a draggable title-bar region",
                || unsafe {
                    let mut rect = RECT::default();
                    GetWindowRect(window, &mut rect).ok()?;
                    Ok((rect.left, rect.top))
                },
                |position| *position != (before.left, before.top),
            )?;
        }
        Ok(())
    })
}

fn drag_title_bar_slider(ui: &Automation) -> Result<()> {
    ui.name("slider-value", "50")?;
    let slider = ui.element("slider")?;
    let _physical = PhysicalCoordinates::new()?;
    let window = ui.window_handle()? as HWND;
    let mut before = RECT::default();
    unsafe { GetWindowRect(window, &mut before).ok()? };
    let value = ui.element("slider-value")?;
    ui.pointer_move_to(&slider, "slider", 0.5, 0.5)?;
    ui.pointer_button(true)?;
    ui.pointer_move_to(&slider, "slider", 0.8, 0.5)?;
    let dragged = ui.wait(
        "slider drag changes its value rather than moving the window",
        || unsafe {
            let mut rect = RECT::default();
            GetWindowRect(window, &mut rect).ok()?;
            Ok(((rect.left, rect.top), value.CurrentName()?))
        },
        |(position, text)| {
            *position != (before.left, before.top)
                || String::from_utf16_lossy(text)
                    .parse::<f64>()
                    .is_ok_and(|value| value > 60.0)
        },
    );
    ui.pointer_button(false)?;
    let (position, _) = dragged?;
    assert_eq!(
        position,
        (before.left, before.top),
        "dragging the slider moved the window"
    );
    Ok(())
}

#[test]
fn initial_window_placement_and_native_updates() -> Result<()> {
    process::run("window-placement", |ui| {
        ui.window_bounds(80, 100, 640, 480)?;
        ui.name("placement", "Placement 80 100; 640 x 480; maximized false")?;
        let original = ui.element("change-initial")?;

        ui.move_resize_window(140, 160, 700, 520)?;
        ui.window_bounds(140, 160, 700, 520)?;
        ui.name("placement", "Placement 140 160; 700 x 520; maximized false")?;

        ui.invoke("change-initial")?;
        ui.name("state", "Changed initial placement: true")?;
        ui.name_stays(
            "placement",
            "Placement 140 160; 700 x 520; maximized false",
            Duration::from_millis(250),
        )?;
        ui.window_bounds(140, 160, 700, 520)?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&original, &ui.element("change-initial")?)?
                    .as_bool(),
                "placement updates replaced the native content"
            );
        }
        Ok(())
    })
}

#[test]
fn window_visuals_constraints_position_theme_and_backdrop() -> Result<()> {
    process::run("window-visuals", |ui| {
        let window = ui.window_handle()? as HWND;
        let origin = || -> Result<(i32, i32)> {
            let _physical = PhysicalCoordinates::new()?;
            let mut rect = RECT::default();
            unsafe { GetWindowRect(window, &mut rect).ok()? };
            Ok((rect.left, rect.top))
        };
        assert_eq!(origin()?, (120, 90));
        ui.window_size(480.0, 320.0)?;
        background(ui, true)?;

        // Constraints bound native resizing in client DIPs.
        ui.move_resize_window(120, 90, 50, 50)?;
        ui.window_size(400.0, 300.0)?;
        ui.move_resize_window(120, 90, 4000, 4000)?;
        ui.window_size(640.0, 480.0)?;
        ui.invoke("constraints")?;
        ui.name("state", "Constrained false; dark true; None; moved false")?;
        ui.move_resize_window(120, 90, 4000, 4000)?;
        let physical = PhysicalCoordinates::new()?;
        ui.wait(
            "unconstrained window exceeds the previous maximum",
            || {
                let mut rect = RECT::default();
                unsafe { GetClientRect(window, &mut rect).ok()? };
                let dpi = unsafe { GetDpiForWindow(window) };
                Ok(f64::from(rect.right) * 96.0 / f64::from(dpi))
            },
            |width| *width > 640.0,
        )?;
        drop(physical);

        // Initial position is read only from the first declaration.
        ui.move_resize_window(120, 90, 700, 500)?;
        ui.invoke("move")?;
        ui.name("state", "Constrained false; dark true; None; moved true")?;
        ui.name_stays(
            "state",
            "Constrained false; dark true; None; moved true",
            Duration::from_millis(250),
        )?;
        assert_eq!(origin()?, (120, 90));

        ui.invoke("theme")?;
        background(ui, false)?;
        // A backdrop replaces the solid background; without one the cleared surface shows black.
        // Clearing between backdrops keeps a previous backdrop from masking a missing one.
        for (id, name) in [
            ("mica", "Mica"),
            ("no-backdrop", "None"),
            ("mica-alt", "MicaAlt"),
            ("no-backdrop", "None"),
            ("acrylic", "Acrylic"),
            ("no-backdrop", "None"),
        ] {
            ui.invoke(id)?;
            ui.name(
                "state",
                &format!("Constrained false; dark false; {name}; moved true"),
            )?;
            background(ui, false)?;
        }
        ui.invoke("theme")?;
        background(ui, true)
    })
}

fn background(ui: &Automation, dark: bool) -> Result<()> {
    let _physical = PhysicalCoordinates::new()?;
    let swatch = ui.element("swatch")?;
    ui.wait(
        &format!(
            "window background is {}",
            if dark { "dark" } else { "light" }
        ),
        || unsafe {
            let bounds = swatch.CurrentBoundingRectangle()?;
            let dc = GetDC(std::ptr::null_mut());
            assert!(!dc.is_null(), "GetDC failed");
            let pixel = GetPixel(
                dc,
                (bounds.left + bounds.right) / 2,
                (bounds.top + bounds.bottom) / 2,
            );
            assert_ne!(ReleaseDC(std::ptr::null_mut(), dc), 0);
            assert_ne!(pixel, u32::MAX, "GetPixel failed");
            Ok([pixel & 0xff, (pixel >> 8) & 0xff, (pixel >> 16) & 0xff])
        },
        |channels| {
            channels
                .iter()
                .all(|&channel| if dark { channel < 64 } else { channel > 192 })
        },
    )?;
    Ok(())
}
