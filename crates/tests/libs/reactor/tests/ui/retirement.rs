use super::*;

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

        // Replacing the Border's content retires the old page's callbacks and native subtree.
        let page = unsafe {
            replacement.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
        };
        ui.invoke("page")?;
        ui.name("page-text", "Page B")?;
        retired(ui, &page, "Selected 1; calls 2")?;
        ui.removed_from_tree("replacement")?;
        ui.invoke("page-action")?;
        ui.name("selection", "Selected 2; calls 3")?;

        ui.invoke("page")?;
        let restored = ui.element("replacement")?;
        unsafe {
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
