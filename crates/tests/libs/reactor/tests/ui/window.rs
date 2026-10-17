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
fn window_icon_set_and_clear() -> Result<()> {
    process::run("window", |ui| {
        ui.window_icon(false)?;
        ui.invoke("set-icon")?;
        ui.window_icon(true)?;
        ui.invoke("clear-icon")?;
        ui.window_icon(false)
    })
}
