use super::*;

#[test]
fn flyout_content_replacement_and_owner_removal() -> Result<()> {
    process::run("flyout", |ui| {
        let owner = ui.focus("open")?;
        ui.gone("action")?;
        ui.invoke("open")?;
        ui.name("action", "Action 0")?;
        let original = ui.element("action")?;
        let stale =
            unsafe { original.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)? };
        ui.invoke("action")?;
        ui.name("selection", "Selected 0; calls 1")?;

        ui.invoke("replace")?;
        ui.name("action", "Action 1")?;
        let current = ui.element("action")?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&owner, &ui.element("open")?)?
                    .as_bool(),
                "content replacement replaced the flyout owner"
            );
            assert!(
                !ui.client.CompareElements(&original, &current)?.as_bool(),
                "keyed content replacement retained the old action"
            );
        }
        retired(ui, &stale, "Selected 0; calls 1")?;
        ui.invoke("action")?;
        ui.name("selection", "Selected 1; calls 2")?;
        let current =
            unsafe { current.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)? };

        ui.invoke("remove")?;
        ui.gone("open")?;
        ui.gone("action")?;
        ui.gone("replace")?;
        ui.gone("remove")?;
        retired(ui, &stale, "Selected 1; calls 2")?;
        retired(ui, &current, "Selected 1; calls 2")
    })
}
