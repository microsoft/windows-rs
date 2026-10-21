use super::*;

/// TabView close requests report the tab key, Reactor's own tab inserts and moves do not report
/// a reorder, and closing after a move removes the right native tab.
#[test]
fn tab_close_keys_and_programmatic_changes() -> Result<()> {
    process::run("tab", |ui| {
        ui.name("order", "1,2,3")?;
        ui.invoke("add")?;
        ui.name("order", "1,4,2,3")?;
        ui.element("tab-4")?;
        ui.invoke("reverse")?;
        ui.name("order", "3,2,4,1")?;
        ui.name_stays("reorders", "0", Duration::from_millis(200))?;

        close(ui, "tab-2")?;
        ui.name("closed", "2")?;
        ui.name("order", "3,4,1")?;
        ui.removed_from_tree("tab-2")?;

        ui.invoke("reverse")?;
        ui.name("order", "1,4,3")?;
        close(ui, "tab-4")?;
        ui.name("closed", "2,4")?;
        ui.removed_from_tree("tab-4")?;
        close(ui, "tab-1")?;
        ui.name("closed", "2,4,1")?;
        ui.name("order", "3")?;
        ui.name_stays("reorders", "0", Duration::from_millis(200))
    })
}

/// Invokes the close button in the template of the tab with the given automation ID.
fn close(ui: &Automation, id: &str) -> Result<()> {
    let tab = ui.element(id)?;
    let button = ui.wait(
        &format!("find the close button of {id:?}"),
        || unsafe {
            let all = ui.client.CreateTrueCondition()?;
            let elements = tab.FindAll(TreeScope_Descendants, &all)?;
            for index in 0..elements.Length()? {
                let element = elements.GetElement(index)?;
                if element.CurrentAutomationId()? == "CloseButton" {
                    return Ok(Some(element));
                }
            }
            Ok(None)
        },
        Option::is_some,
    )?;
    eprintln!("close {id:?}");
    unsafe {
        button
            .unwrap()
            .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
            .Invoke()
            .ok()?;
    }
    Ok(())
}
