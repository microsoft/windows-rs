use super::*;

/// Keyed DataItem edits keep the native item order and text, and a selected index applied at mount
/// or with a selection mode change lands after the items and mode it depends on.
#[test]
fn list_items_and_selection() -> Result<()> {
    process::run("list", |ui| {
        let second = ui.list_item("mounted", "Second")?;
        ui.selected_list_item("mounted", Some(&second))?;

        order(ui, &["Alpha", "Beta", "Rich content"])?;
        ui.invoke("reverse")?;
        order(ui, &["Rich content", "Beta", "Alpha"])?;
        ui.invoke("insert")?;
        order(ui, &["Rich content", "Delta", "Beta", "Alpha"])?;
        ui.invoke("remove")?;
        order(ui, &["Rich content", "Delta", "Alpha"])?;
        // The new item may reuse the removed item's container and must show its own text.
        ui.invoke("insert-first")?;
        order(ui, &["Gamma", "Rich content", "Delta", "Alpha"])?;

        // Selection mode and index change in one update; the index must apply after the mode.
        ui.invoke("enable-selection")?;
        let three = ui.list_item("selectable", "Three")?;
        ui.selected_list_item("selectable", Some(&three))
    })
}

/// Waits until the list items in "items" have the given names, in order. A DataItem's text, or
/// its rich content, belongs to the first named descendant of the item container.
fn order(ui: &Automation, expected: &[&str]) -> Result<()> {
    let list = ui.element("items")?;
    ui.wait(
        &format!("\"items\" order becomes {expected:?}"),
        || unsafe {
            let all = ui.client.CreateTrueCondition()?;
            let items = list.FindAll(TreeScope_Children, &all)?;
            let mut names = Vec::new();
            for index in 0..items.Length()? {
                let item = items.GetElement(index)?;
                if item.CurrentControlType()? != LIST_ITEM {
                    continue;
                }
                let content = item.FindAll(TreeScope_Descendants, &all)?;
                let mut name = String::new();
                for index in 0..content.Length()? {
                    let candidate = content.GetElement(index)?.CurrentName()?;
                    if !candidate.is_empty() {
                        name = String::from_utf16_lossy(&candidate);
                        break;
                    }
                }
                names.push(name);
            }
            Ok(names)
        },
        |names| {
            names
                .iter()
                .map(String::as_str)
                .eq(expected.iter().copied())
        },
    )?;
    Ok(())
}
