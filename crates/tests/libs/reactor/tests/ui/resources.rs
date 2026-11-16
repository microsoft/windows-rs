use super::*;

#[test]
fn resource_overrides_apply_clear_and_reapply() -> Result<()> {
    process::run("resources", |ui| {
        ui.name("state", "Styled true; toggles 0")?;
        ui.invoke("toggle")?;
        ui.name("state", "Styled false; toggles 1")?;
        ui.invoke("toggle")?;
        ui.name("state", "Styled true; toggles 2")?;
        ui.invoke("target")
    })
}
