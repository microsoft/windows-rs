use super::*;

#[test]
fn list_selection_feedback_and_handler_lifetimes() -> Result<()> {
    process::run("selection", |ui| {
        let list = ui.element("list")?;
        let alpha = ui.list_item("list", "Alpha")?;
        let beta = ui.list_item("list", "Beta")?;
        let gamma = ui.list_item("list", "Gamma")?;
        ui.selected_list_item("list", None)?;
        ui.name("events", "Calls 0; received None")?;

        select(&beta)?;
        ui.selected_list_item("list", Some(&beta))?;
        ui.name("selection", "Selected Some(1)")?;
        ui.name("events", "Calls 1; received Some((0, Some(1)))")?;

        ui.invoke("select-last")?;
        ui.selected_list_item("list", Some(&gamma))?;
        ui.name("selection", "Selected Some(2)")?;
        ui.name_stays(
            "events",
            "Calls 1; received Some((0, Some(1)))",
            Duration::from_millis(250),
        )?;
        ui.invoke("clear")?;
        ui.selected_list_item("list", None)?;
        ui.name("selection", "Selected None")?;
        ui.name_stays(
            "events",
            "Calls 1; received Some((0, Some(1)))",
            Duration::from_millis(250),
        )?;

        ui.invoke("replace")?;
        ui.name("handler", "Handler 1; attached true")?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&list, &ui.element("list")?)?
                    .as_bool(),
                "handler replacement recreated the list"
            );
            assert!(
                ui.client
                    .CompareElements(&alpha, &ui.list_item("list", "Alpha")?)?
                    .as_bool(),
                "handler replacement recreated the item"
            );
        }
        select(&alpha)?;
        ui.selected_list_item("list", Some(&alpha))?;
        ui.name("selection", "Selected Some(0)")?;
        ui.name("events", "Calls 2; received Some((1, Some(0)))")?;

        ui.invoke("remove")?;
        ui.name("handler", "Handler 1; attached false")?;
        select(&beta)?;
        ui.selected_list_item("list", Some(&beta))?;
        ui.name_stays(
            "events",
            "Calls 2; received Some((1, Some(0)))",
            Duration::from_millis(250),
        )
    })
}

fn select(item: &IUIAutomationElement) -> Result<()> {
    const SELECTION_ITEM_PATTERN: i32 = 10010;
    unsafe {
        item.GetCurrentPatternAs::<IUIAutomationSelectionItemPattern>(SELECTION_ITEM_PATTERN)?
            .Select()
            .ok()?;
    }
    Ok(())
}
