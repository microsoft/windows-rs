use super::*;

#[test]
fn title_bar_dialog_primary_button_closes_window() -> Result<()> {
    process::run_closing("dialog-shutdown", |ui| {
        ui.element("dialog-content")?;
        ui.invoke("PrimaryButton")
    })
}

#[test]
fn title_bar_dialog_close_button_closes_window() -> Result<()> {
    process::run_closing("dialog-shutdown", |ui| {
        ui.element("dialog-content")?;
        ui.invoke("CloseButton")
    })
}

#[test]
fn title_bar_dialog_open_during_window_shutdown() -> Result<()> {
    process::run("dialog-shutdown", |ui| {
        ui.element("dialog-content")?;
        Ok(())
    })
}

#[test]
fn dialog_open_during_window_shutdown() -> Result<()> {
    process::run("dialog", |ui| {
        ui.element("dialog-content")?;
        Ok(())
    })
}

#[test]
fn dialog_requested_at_mount_closes_and_reopens() -> Result<()> {
    process::run("dialog", |ui| {
        ui.element("dialog-content")?;
        ui.invoke("PrimaryButton")?;
        ui.name("closed", "Closed 1; result Primary")?;
        ui.gone("dialog-content")?;
        ui.invoke("open")?;
        ui.element("dialog-content")?;
        ui.invoke("CloseButton")?;
        ui.name("closed", "Closed 2; result None")?;
        ui.gone("dialog-content")?;
        ui.name_stays(
            "closed",
            "Closed 2; result None",
            Duration::from_millis(250),
        )
    })
}
