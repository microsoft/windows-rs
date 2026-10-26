use super::*;

const DECLARED: [&str; 7] = [
    "list-view",
    "grid-view",
    "combo-box",
    "radio-buttons",
    "pivot",
    "flip-view",
    "tab-view",
];

/// A declared index is applied after its items and kept across Reactor's own item changes without
/// reporting those native moves; an undeclared index still reports them.
#[test]
fn declared_index_does_not_echo() -> Result<()> {
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

/// A user selection on a declared-index selector is reported once, and the declared index is
/// restored when the app keeps it.
#[test]
fn declared_index_rejects_user_selection() -> Result<()> {
    process::run("selection-index", |ui| {
        let mut echoes = Vec::new();
        for id in DECLARED {
            ui.selected_name(id, "One")?;
            user_select_next(ui, id)?;
            echoes.push(format!("({id:?}, Some(2))"));
            let expected = format!("Echoes [{}]", echoes.join(", "));
            ui.name("echoes", &expected)?;
            ui.selected_name(id, "One")?;
            ui.name_stays("echoes", &expected, Duration::from_millis(500))?;
        }
        Ok(())
    })
}

/// Selects "Two", the item after the declared one, as a user would.
fn user_select_next(ui: &Automation, id: &str) -> Result<()> {
    let expand_collapse = || -> Result<IUIAutomationExpandCollapsePattern> {
        Ok(unsafe {
            ui.element(id)?
                .GetCurrentPatternAs(EXPAND_COLLAPSE_PATTERN)?
        })
    };
    match id {
        // FlipView realizes only its current item; a user hovers it and clicks its Next button.
        "flip-view" => {
            ui.pointer_move(id, 0.5, 0.5)?;
            ui.pointer_move("NextButtonHorizontal", 0.5, 0.5)?;
            ui.pointer_clicks(1)
        }
        // ComboBox items are in the automation tree only while its drop-down is open.
        "combo-box" => {
            unsafe { expand_collapse()?.Expand().ok()? };
            select_item(ui, id, "Two")?;
            unsafe { expand_collapse()?.Collapse().ok()? };
            Ok(())
        }
        _ => select_item(ui, id, "Two"),
    }
}
/// Item-flag selectors report only user selections. Declared flags drive all three controls,
/// including across insertion, reordering, clearing, and removal of the selected item.
#[test]
fn declared_item_flags_do_not_echo() -> Result<()> {
    // SelectorBar has no automation peer of its own, so it is found through its items view.
    const SELECTOR_BAR: &str = "PART_ItemsView";
    const CONTROLS: [&str; 3] = ["list-box", SELECTOR_BAR, "navigation-view"];
    process::run("selection-items", |ui| {
        let all_selected = |name: Option<&str>, echoes: &str| -> Result<()> {
            for id in CONTROLS {
                selected_item(ui, id, name)?;
            }
            ui.name_stays("echoes", echoes, Duration::from_millis(500))
        };
        all_selected(Some("One"), "Echoes []")?;

        select_item(ui, "list-box", "Zero")?;
        all_selected(Some("Zero"), r#"Echoes [("list-box", Some("Zero"))]"#)?;
        select_item(ui, SELECTOR_BAR, "Two")?;
        all_selected(
            Some("Two"),
            r#"Echoes [("list-box", Some("Zero")), ("selector-bar", Some("Two"))]"#,
        )?;
        select_item(ui, "navigation-view", "One")?;
        let echoes = r#"Echoes [("list-box", Some("Zero")), ("selector-bar", Some("Two")), ("navigation-view", Some("One"))]"#;
        all_selected(Some("One"), echoes)?;

        ui.invoke("select-last")?;
        all_selected(Some("Two"), echoes)?;
        ui.invoke("insert-first")?;
        all_selected(Some("Two"), echoes)?;
        ui.invoke("rotate")?;
        all_selected(Some("Two"), echoes)?;
        ui.invoke("clear")?;
        all_selected(None, echoes)?;
        ui.invoke("select-last")?;
        all_selected(Some("One"), echoes)?;
        ui.invoke("remove-selected")?;
        all_selected(None, echoes)
    })
}

const SELECTION_ITEM_PATTERN: i32 = 10010;

/// Selectable descendants of `control` with their names, taken from the element or its first named
/// descendant.
fn selectable_items(
    ui: &Automation,
    control: &IUIAutomationElement,
) -> windows_core::Result<Vec<(String, IUIAutomationSelectionItemPattern)>> {
    let mut items = Vec::new();
    unsafe {
        let all = ui.client.CreateTrueCondition()?;
        let elements = control.FindAll(TreeScope_Descendants, &all)?;
        for index in 0..elements.Length()? {
            let element = elements.GetElement(index)?;
            let Ok(pattern) = element
                .GetCurrentPatternAs::<IUIAutomationSelectionItemPattern>(SELECTION_ITEM_PATTERN)
            else {
                continue;
            };
            let mut name = element.CurrentName()?.display().to_string();
            let content = element.FindAll(TreeScope_Descendants, &all)?;
            for index in 0..content.Length()? {
                if !name.is_empty() {
                    break;
                }
                name = content
                    .GetElement(index)?
                    .CurrentName()?
                    .display()
                    .to_string();
            }
            items.push((name, pattern));
        }
    }
    Ok(items)
}

fn selected_item(ui: &Automation, id: &str, expected: Option<&str>) -> Result<()> {
    let control = ui.element(id)?;
    ui.wait(
        &format!("{id:?} selection becomes {expected:?}"),
        || {
            let mut selected = Vec::new();
            for (name, pattern) in selectable_items(ui, &control)? {
                if unsafe { pattern.CurrentIsSelected()? }.as_bool() {
                    selected.push(name);
                }
            }
            Ok(selected)
        },
        |selected| selected.iter().map(String::as_str).eq(expected),
    )?;
    Ok(())
}

fn select_item(ui: &Automation, id: &str, name: &str) -> Result<()> {
    eprintln!("select {name:?} in {id:?}");
    let control = ui.element(id)?;
    let (_, pattern) = ui
        .wait(
            &format!("find selectable {name:?} in {id:?}"),
            || {
                Ok(selectable_items(ui, &control)?
                    .into_iter()
                    .find(|(item, _)| item == name))
            },
            Option::is_some,
        )?
        .unwrap();
    unsafe { pattern.Select().ok()? };
    Ok(())
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
    unsafe {
        item.GetCurrentPatternAs::<IUIAutomationSelectionItemPattern>(SELECTION_ITEM_PATTERN)?
            .Select()
            .ok()?;
    }
    Ok(())
}
