use super::*;

#[test]
fn window_close_discards_pending_dialog_update() -> Result<()> {
    process::run_closing("dialog", |ui| {
        ui.element("dialog-content")?;
        ui.invoke("CloseButton")?;
        ui.name("closed", "Closed 1; result None")?;
        ui.gone("dialog-content")?;
        ui.invoke("close-with-pending-open")
    })
}

#[test]
fn application_exit_closes_visible_title_bar_dialog() -> Result<()> {
    process::run_closing("dialog-application", |ui| {
        ui.invoke("open-window")?;
        ui.element("queue-dialog")?;
        ui.invoke("exit-application")
    })
}

#[test]
fn window_close_cancels_queued_dialog_with_another_window_alive() -> Result<()> {
    process::run("dialog-application", |ui| {
        ui.invoke("open-window")?;
        ui.element("queue-dialog")?;
        ui.invoke("queue-dialog")?;
        ui.name("shutdown-state", "Queued true; closed false; drained false")?;
        ui.gone("queued-dialog-content")?;
        ui.invoke("close-dialog-window")?;
        ui.name("shutdown-state", "Queued true; closed true; drained true")?;
        ui.gone("queue-dialog")?;
        ui.gone("queued-dialog-content")
    })
}

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
