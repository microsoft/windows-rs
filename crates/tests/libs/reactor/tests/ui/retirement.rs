use super::*;

#[test]
fn nested_retirement_preserves_cleanup_drop_order_and_generation() -> Result<()> {
    process::run("retirement-nested", |ui| {
        ui.name("mounted", "Mounted 1")?;
        let retained = ui.element("nested-5")?;
        check_nested(ui, 1, "Created 6; cleaned 0; dropped 0")?;
        ui.invoke("toggle")?;
        ui.name("retired", "Retired 1")?;
        check_nested(ui, 2, "Created 6; cleaned 5; dropped 5")?;
        ui.invoke("toggle")?;
        ui.name("mounted", "Mounted 2")?;
        check_retired_senders(ui, &retained, 3, 11, 5)
    })
}

#[test]
fn virtual_key_removal_preserves_nested_cleanup_and_drop_order() -> Result<()> {
    process::run("retirement-virtual", |ui| {
        ui.name("nested-4", "Node 4")?;
        let retained = ui.element("nested-5")?;
        ui.invoke("check")?;
        ui.name("checks", "Checks 1")?;
        // WinUI can realize and recycle the first row during startup. Audit every lifetime.
        let mounted = nested_count(ui, "mounted", "Mounted ")?;
        let retired = nested_count(ui, "retired", "Retired ")?;
        assert_eq!(mounted, retired + 1);
        nested_snapshot(
            ui,
            1,
            &format!(
                "Created {}; cleaned {}; dropped {}",
                mounted * 5 + 1,
                retired * 5,
                retired * 5
            ),
        )?;
        let scroll = unsafe {
            ui.element("rows")?
                .GetCurrentPatternAs::<IUIAutomationScrollPattern>(10004)?
        };
        unsafe { scroll.SetScrollPercent(-1.0, 100.0).ok()? };
        ui.name("row-199", "Row 199")?;
        // Offscreen rows can remain realized. Removing the key requires retirement regardless.
        ui.invoke("toggle")?;
        ui.name("retired", &format!("Retired {}", retired + 1))?;
        check_nested(
            ui,
            2,
            &format!(
                "Created {}; cleaned {}; dropped {}",
                mounted * 5 + 1,
                (retired + 1) * 5,
                (retired + 1) * 5
            ),
        )?;
        unsafe { scroll.SetScrollPercent(-1.0, 0.0).ok()? };
        ui.name("row-1", "Row 1")?;
        ui.invoke("toggle")?;
        ui.name("nested-4", "Node 4")?;
        ui.invoke("check")?;
        ui.name("checks", "Checks 3")?;
        let remounted = nested_count(ui, "mounted", "Mounted ")?;
        let retired = nested_count(ui, "retired", "Retired ")?;
        assert!(remounted > mounted);
        assert_eq!(remounted, retired + 1);
        nested_snapshot(
            ui,
            3,
            &format!(
                "Created {}; cleaned {}; dropped {}",
                remounted * 5 + 1,
                retired * 5,
                retired * 5
            ),
        )?;
        check_retired_senders(ui, &retained, 4, remounted * 5 + 1, retired * 5)
    })
}

fn nested_count(ui: &Automation, id: &str, prefix: &str) -> Result<usize> {
    let value = String::from_utf16(&unsafe { ui.element(id)?.CurrentName()? })?;
    Ok(value.strip_prefix(prefix).unwrap().parse()?)
}

fn check_nested(ui: &Automation, check: usize, expected: &str) -> Result<()> {
    ui.invoke("check")?;
    nested_snapshot(ui, check, expected)
}

fn nested_snapshot(ui: &Automation, check: usize, expected: &str) -> Result<()> {
    ui.name("checks", &format!("Checks {check}"))?;
    assert_eq!(unsafe { ui.element("snapshot")?.CurrentName()? }, expected);
    Ok(())
}

fn check_retired_senders(
    ui: &Automation,
    retained: &IUIAutomationElement,
    check: usize,
    created: usize,
    retired: usize,
) -> Result<()> {
    ui.invoke("send-retired")?;
    nested_snapshot(
        ui,
        check,
        &format!("Created {created}; cleaned {retired}; dropped {retired}"),
    )?;
    assert!(unsafe {
        ui.client
            .CompareElements(retained, &ui.element("nested-5")?)?
            .as_bool()
    });
    Ok(())
}

#[test]
fn removal_transition_retires_callbacks_and_removes_native_child() -> Result<()> {
    process::run("retirement", |ui| {
        ui.name(
            "publication",
            "Callback released false; reference queued true",
        )?;
        let original = ui.element("old")?;
        let first = ui.element("first")?;
        let second = ui.element("second")?;
        let retained = ui.element("retained-scope")?;
        ui.name("scope-lifetimes", "Cleanups 0; drops 0")?;
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
        // WinUI may still render and expose the removed child while it animates out.
        retired(ui, &invoke, "Selected 0; calls 1")?;
        ui.removed_from_tree("old")?;
        ui.invoke("swap")?;
        for (key, element) in [("first", &first), ("second", &second)] {
            unsafe {
                assert!(
                    ui.client
                        .CompareElements(element, &ui.element(key)?)?
                        .as_bool(),
                    "{key:?} lost its native identity"
                );
            }
        }
        stacked(ui, &[&second, &first, &replacement])?;
        ui.invoke("replacement")?;
        ui.name("selection", "Selected 1; calls 2")?;
        ui.name("scope-lifetimes", "Cleanups 0; drops 0")?;

        // Replacing the Border's content retires the old page's callbacks and native subtree.
        let page = unsafe {
            replacement.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
        };
        ui.invoke("page")?;
        ui.name("page-text", "Page B")?;
        ui.name("scope-lifetimes", "Cleanups 2; drops 2")?;
        retired(ui, &page, "Selected 1; calls 2")?;
        ui.removed_from_tree("replacement")?;
        ui.invoke("page-action")?;
        ui.name("selection", "Selected 2; calls 3")?;

        ui.invoke("page")?;
        let restored = ui.element("replacement")?;
        ui.name("scope-lifetimes", "Cleanups 2; drops 2")?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&retained, &ui.element("retained-scope")?)?
                    .as_bool(),
                "page swap replaced the retained component's native child"
            );
            assert!(
                !ui.client
                    .CompareElements(&replacement, &restored)?
                    .as_bool(),
                "page swap reused the old native child"
            );
        }
        ui.removed_from_tree("page-action")?;
        ui.invoke("replacement")?;
        ui.name("selection", "Selected 1; calls 4")
    })
}

/// Waits until the elements are laid out top to bottom in the given order.
fn stacked(ui: &Automation, elements: &[&IUIAutomationElement]) -> Result<()> {
    ui.wait(
        "elements are stacked in the expected native order",
        || {
            elements
                .iter()
                .map(|element| Ok(unsafe { element.CurrentBoundingRectangle()?.top }))
                .collect::<windows_core::Result<Vec<_>>>()
        },
        |tops| tops.windows(2).all(|pair| pair[0] < pair[1]),
    )?;
    Ok(())
}
