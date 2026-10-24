use super::*;

#[test]
fn repeater_updates_preserve_realized_component_state() -> Result<()> {
    process::run("repeater", |ui| {
        ui.name("row-0", "Row 0; value 0; clicks 0")?;
        let row = ui.element("row-0")?;
        ui.invoke("row-0")?;
        ui.name("row-0", "Row 0; value 0; clicks 1")?;

        ui.invoke("rerender")?;
        ui.name("revision", "Revision 1")?;
        ui.name("row-0", "Row 0; value 0; clicks 1")?;
        ui.invoke("change")?;
        ui.name("revision", "Revision 2")?;
        ui.name("row-0", "Row 0; value 1; clicks 1")?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&row, &ui.element("row-0")?)?
                    .as_bool(),
                "input update recreated the realized row"
            );
        }
        ui.invoke("row-0")?;
        ui.name("row-0", "Row 0; value 1; clicks 2")
    })
}

#[test]
fn repeater_scrolling_realizes_only_nearby_rows() -> Result<()> {
    const SCROLL_PATTERN: i32 = 10004;
    const NO_SCROLL: f64 = -1.0;
    process::run("repeater", |ui| {
        ui.name("row-0", "Row 0; value 0; clicks 0")?;
        unsafe {
            ui.element("rows")?
                .GetCurrentPatternAs::<IUIAutomationScrollPattern>(SCROLL_PATTERN)?
                .SetScrollPercent(NO_SCROLL, 50.0)
                .ok()?;
        }
        ui.name("row-128", "Row 128; value 0; clicks 0")?;
        let realized = ui.count("row-")?;
        assert!(realized < 64, "{realized} of 256 rows realized");
        Ok(())
    })
}

#[test]
fn repeater_realizes_rows_requested_during_an_update() -> Result<()> {
    process::run("repeater_layout", |ui| {
        ui.invoke("load")?;
        ui.name("row-0", "Row 0; value 0; clicks 0")?;
        ui.invoke("count")?;
        let count = ui.element("count")?;
        let mut attempts = 0;
        let created = loop {
            attempts += 1;
            assert!(attempts < 500, "row count was not reported");
            let name = String::from_utf16_lossy(&unsafe { count.CurrentName()? });
            let created: usize = name.strip_prefix("Created ").unwrap().parse().unwrap();
            if created != 0 {
                break created;
            }
            std::thread::sleep(Duration::from_millis(20));
        };
        assert!(created < 64, "{created} of 256 rows created");
        ui.invoke("row-0")?;
        ui.name("row-0", "Row 0; value 0; clicks 1")
    })
}
