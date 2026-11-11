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
