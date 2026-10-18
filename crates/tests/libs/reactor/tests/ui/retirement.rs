use super::*;

#[test]
fn exit_fade_retires_callbacks_and_removes_native_child() -> Result<()> {
    process::run("retirement", |ui| {
        ui.name(
            "publication",
            "Callback released false; reference queued true",
        )?;
        let original = ui.element("old")?;
        let invoke =
            unsafe { original.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)? };
        ui.invoke("old")?;
        ui.name("selection", "Selected 0; calls 1")?;

        ui.invoke("remove")?;
        ui.name(
            "publication",
            "Callback released true; reference queued false",
        )?;
        let replacement = ui.element("replacement")?;
        unsafe {
            assert!(
                !ui.client
                    .CompareElements(&original, &replacement)?
                    .as_bool(),
                "retirement reused the old native child"
            );
        }
        retired(ui, &invoke, "Selected 0; calls 1")?;
        ui.removed_from_tree("old")?;
        ui.invoke("replacement")?;
        ui.name("selection", "Selected 1; calls 2")
    })
}
