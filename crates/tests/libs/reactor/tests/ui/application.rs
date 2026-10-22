use super::*;

#[test]
fn application_window_activation_and_updates_preserve_identity() -> Result<()> {
    process::run("application", |ui| {
        ui.window_handle()?;
        let lifetime = ui.element("lifetime")?;
        ui.invoke("show")?;
        ui.name("lifetime", "Mounted 1; closed 0")?;
        ui.name("secondary-value", "Value 0")?;
        let original = ui.element("secondary-value")?;
        ui.invoke("show")?;
        ui.invoke("increment")?;
        ui.name("secondary-value", "Value 1")?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&original, &ui.element("secondary-value")?)?
                    .as_bool()
            );
        }
        ui.name_stays(
            "lifetime",
            "Mounted 1; closed 0",
            Duration::from_millis(250),
        )?;
        ui.invoke("close-secondary")?;
        ui.name_of(&lifetime, "Mounted 1; closed 1")
    })
}

#[test]
fn replacing_the_last_window_keeps_the_application_alive() -> Result<()> {
    process::run("window-replacement", |ui| {
        ui.invoke("replace-window")?;
        ui.gone("replace-window")?;
        ui.name("replacement", "Updates 0")?;
        ui.invoke("ping")?;
        ui.name("replacement", "Updates 1")
    })
}

#[test]
fn exit_with_open_window_returns() -> Result<()> {
    process::run_to_exit("exit-with-window")?;
    process::run_to_exit("error-with-window")
}

#[test]
fn application_startup_error_is_returned() -> Result<()> {
    process::run_to_exit("startup-error")
}
