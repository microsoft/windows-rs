use super::*;

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
            ui.pointer_move_to(&empty, "drag-region", 0.5, 0.5)?;
            ui.pointer_button(true)?;
            ui.pointer_move_to(&empty, "drag-region", 0.8, 0.5)?;
            let moved = ui.wait(
                "removed slider leaves a draggable title-bar region",
                || unsafe {
                    let mut rect = RECT::default();
                    GetWindowRect(window, &mut rect).ok()?;
                    Ok((rect.left, rect.top))
                },
                |position| *position != (before.left, before.top),
            );
            ui.pointer_button(false)?;
            moved?;
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
