use super::*;

#[test]
fn element_ref_focus_survives_replacement_and_rejects_stale_requests() -> Result<()> {
    process::run("reference", |ui| {
        let original = ui.element("target")?;
        // UIA focuses only the trigger; Reactor must focus the target.
        ui.focus("focus")?;
        ui.invoke("focus")?;
        ui.name(
            "result",
            "Queued true; completions 1; result Some(Ok(true))",
        )?;
        ui.focused("target")?;

        ui.focus("replace")?;
        ui.invoke("replace")?;
        ui.name("state", "Generation 1; attached true")?;
        ui.name(
            "result",
            "Queued true; completions 2; result Some(Err(Unavailable))",
        )?;
        ui.focused("replace")?;
        let replacement = ui.element("target")?;
        unsafe {
            assert!(
                !ui.client
                    .CompareElements(&original, &replacement)?
                    .as_bool(),
                "keyed replacement retained the old native target"
            );
        }
        ui.invoke("focus")?;
        ui.name(
            "result",
            "Queued true; completions 3; result Some(Ok(true))",
        )?;
        ui.focused("target")?;

        ui.focus("remove")?;
        ui.invoke("remove")?;
        ui.name("state", "Generation 1; attached false")?;
        ui.name(
            "result",
            "Queued true; completions 4; result Some(Err(Unavailable))",
        )?;
        ui.focused("remove")?;
        ui.focus("focus")?;
        ui.invoke("focus")?;
        ui.name("result", "Queued false; completions 4; result None")?;
        ui.focused("focus")?;

        ui.invoke("attach")?;
        ui.name("state", "Generation 2; attached true")?;
        ui.invoke("focus")?;
        ui.name(
            "result",
            "Queued true; completions 5; result Some(Ok(true))",
        )?;
        ui.focused("target")
    })
}
