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
