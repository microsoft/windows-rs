use super::*;

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
