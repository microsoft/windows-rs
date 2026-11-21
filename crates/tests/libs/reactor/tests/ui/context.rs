use super::*;

#[test]
fn virtual_rows_keep_context_snapshots_and_provider_isolation() -> Result<()> {
    process::run("context-virtual", |ui| {
        ui.name("context-row-0", "Value 1/10; renders 1; clicks 0")?;
        ui.name("context-row-1", "Value 1/10; renders 1; clicks 0")?;
        let row = ui.element("context-row-0")?;
        ui.invoke("context-row-0")?;
        ui.name("context-row-0", "Value 1/10; renders 2; clicks 1")?;

        ui.invoke("bump")?;
        ui.name("context-row-0", "Value 2/10; renders 3; clicks 1")?;
        ui.name("context-row-1", "Value 2/10; renders 2; clicks 0")?;
        ui.invoke("same")?;
        ui.name("root", "Root renders 3")?;
        ui.name("context-row-0", "Value 2/10; renders 3; clicks 1")?;
        ui.name("context-row-1", "Value 2/10; renders 2; clicks 0")?;

        ui.invoke("shadow")?;
        ui.name("context-row-0", "Value 50/10; renders 4; clicks 1")?;
        ui.name("context-row-1", "Value 2/10; renders 2; clicks 0")?;
        ui.invoke("bump")?;
        ui.name("list-context", "Value 3/10")?;
        ui.name("context-row-0", "Value 50/10; renders 4; clicks 1")?;
        ui.name("context-row-1", "Value 3/10; renders 3; clicks 0")?;
        ui.invoke("other")?;
        ui.name("context-row-0", "Value 50/11; renders 5; clicks 1")?;
        ui.name("context-row-1", "Value 3/11; renders 4; clicks 0")?;

        ui.invoke("provide")?;
        ui.name("context-row-0", "Value 50/11; renders 5; clicks 1")?;
        ui.name("context-row-1", "Value 100/11; renders 5; clicks 0")?;
        ui.invoke("shadow")?;
        ui.name("context-row-0", "Value 100/11; renders 6; clicks 1")?;
        assert!(unsafe {
            ui.client
                .CompareElements(&row, &ui.element("context-row-0")?)?
                .as_bool()
        });

        unsafe {
            ui.element("rows")?
                .GetCurrentPatternAs::<IUIAutomationScrollPattern>(10004)?
                .SetScrollPercent(-1.0, 100.0)
                .ok()?;
        }
        ui.name("context-row-199", "Value 100/11; renders 1; clicks 0")?;
        ui.invoke("provide")?;
        ui.name("context-row-199", "Value 3/11; renders 2; clicks 0")?;
        ui.invoke("context-row-199")?;
        ui.name("context-row-199", "Value 3/11; renders 3; clicks 1")
    })
}

#[test]
fn context_values_reach_consumers_without_rendering_intermediates() -> Result<()> {
    process::run("context", |ui| {
        ui.name("a-value", "Value 1; renders 1")?;
        ui.name("b-value", "Value 1; renders 1")?;
        let a = ui.element("a-value")?;
        let b = ui.element("b-value")?;

        ui.invoke("bump")?;
        ui.name("a-value", "Value 2; renders 2")?;
        ui.name("b-value", "Value 2; renders 2")?;

        // An equal value does not render consumers.
        ui.invoke("same")?;
        ui.name("root", "Root renders 3")?;
        ui.name("a-value", "Value 2; renders 2")?;
        ui.name("b-value", "Value 2; renders 2")?;

        // A nested provider shadows the outer value for its subtree only.
        ui.invoke("shadow")?;
        ui.name("b-value", "Value 50; renders 3")?;
        ui.invoke("bump")?;
        ui.name("a-value", "Value 3; renders 3")?;
        ui.name("root", "Root renders 5")?;
        ui.name("b-value", "Value 50; renders 3")?;

        // Without a provider, consumers read the context default.
        ui.invoke("provide")?;
        ui.name("a-value", "Value 100; renders 4")?;
        ui.name("b-value", "Value 50; renders 3")?;
        ui.invoke("shadow")?;
        ui.name("b-value", "Value 100; renders 4")?;
        ui.invoke("provide")?;
        ui.name("a-value", "Value 3; renders 5")?;
        ui.name("b-value", "Value 3; renders 5")?;

        // Providers are transparent: none of this recreated a component or native element.
        ui.name("a-middle", "Middle renders 1")?;
        ui.name("b-middle", "Middle renders 1")?;
        for (before, id) in [(a, "a-value"), (b, "b-value")] {
            unsafe {
                assert!(
                    ui.client
                        .CompareElements(&before, &ui.element(id)?)?
                        .as_bool(),
                    "{id} was recreated"
                );
            }
        }
        Ok(())
    })
}
