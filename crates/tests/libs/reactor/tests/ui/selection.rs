use super::*;

/// A declared index is applied after its items and kept across Reactor's own item changes without
/// reporting those native moves; an undeclared index still reports them.
#[test]
fn declared_index_does_not_echo() -> Result<()> {
    const DECLARED: [&str; 7] = [
        "list-view",
        "grid-view",
        "combo-box",
        "radio-buttons",
        "pivot",
        "flip-view",
        "tab-view",
    ];
    process::run("selection-index", |ui| {
        let selected = |name| -> Result<()> {
            for id in DECLARED {
                ui.selected_name(id, name)?;
            }
            ui.name_stays("echoes", "Echoes []", Duration::from_millis(500))
        };
        selected("One")?;

        select(&ui.list_item("free-list", "Zero")?)?;
        ui.name("free", "Free Some(0)")?;
        ui.invoke("remove-first")?;
        ui.name("free", "Free None")?;
        selected("Two")?;

        ui.invoke("insert-first")?;
        selected("One")?;
        ui.invoke("rotate")?;
        selected("Front")
    })
}
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
