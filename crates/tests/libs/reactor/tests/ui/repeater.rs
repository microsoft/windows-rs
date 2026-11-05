use super::*;

#[test]
fn repeater_creation_and_rendering_follow_visible_changes() -> Result<()> {
    for fixture in ["repeater_items", "repeater_virtual"] {
        process::run(fixture, |ui| check_row_work(ui, 128))?;
    }
    Ok(())
}

#[test]
fn repeater_short_and_mixed_rows_keep_realization_bounded() -> Result<()> {
    for fixture in ["repeater_short", "repeater_mixed"] {
        process::run(fixture, |ui| check_row_work(ui, 1_000))?;
    }
    Ok(())
}

fn row_work(ui: &Automation, sample: usize) -> Result<[usize; 3]> {
    let counter = ui.element("count")?;
    ui.invoke("count")?;
    let counts = ui.wait(
        "row work snapshot",
        || unsafe { Ok(String::from_utf16_lossy(&counter.CurrentName()?)) },
        |name| name.starts_with(&format!("{sample}:")),
    )?;
    let counts = counts
        .split(':')
        .skip(1)
        .map(str::parse)
        .collect::<std::result::Result<Vec<usize>, _>>()?;
    Ok(counts.try_into().unwrap())
}

fn check_row_work(ui: &Automation, bound: usize) -> Result<()> {
    ui.name("row-0", "Row 0; value 0; clicks 0")?;
    let baseline = row_work(ui, 1)?;
    eprintln!("startup [created, rendered, dropped]: {baseline:?}");
    assert!(baseline[0] > 0 && baseline[0] < bound, "{baseline:?}");
    ui.invoke("rerender")?;
    ui.name("revision", "Revision 1")?;
    let unchanged = row_work(ui, 2)?;
    eprintln!("unchanged: {unchanged:?}");
    assert_eq!(unchanged, baseline);
    ui.invoke("change")?;
    ui.name("row-0", "Row 0; value 1; clicks 0")?;
    let changed = row_work(ui, 3)?;
    eprintln!("changed: {changed:?}");
    assert_eq!(changed, [baseline[0], baseline[1] + 1, baseline[2]]);
    Ok(())
}

#[test]
fn repeater_short_and_mixed_rows_realize_during_selection() -> Result<()> {
    for fixture in ["repeater_layout_short", "repeater_layout_mixed"] {
        process::run(fixture, |ui| {
            ui.invoke("load")?;
            ui.name("row-0", "Row 0; value 0; clicks 0")?;
            let counter = ui.element("count")?;
            ui.invoke("count")?;
            let name = ui.wait(
                "created rows are counted",
                || unsafe { Ok(String::from_utf16_lossy(&counter.CurrentName()?)) },
                |name| name != "Created 0",
            )?;
            let created: usize = name.strip_prefix("Created ").unwrap().parse()?;
            eprintln!("selection created: {created}");
            assert!(created > 0 && created < 1_000, "{created} rows created");
            Ok(())
        })?;
    }
    Ok(())
}

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
fn repeater_reorder_preserves_realized_state_and_focus() -> Result<()> {
    const SPACE: i32 = 0x20;
    process::run("repeater", |ui| {
        let original = ui.element("row-0")?;
        let moved = ui.element("row-1")?;
        ui.invoke("row-0")?;
        ui.name("row-0", "Row 0; value 0; clicks 1")?;
        for revision in 1..=3 {
            // Row 1 requests the swap itself, without moving focus to an external trigger.
            ui.press_keys("row-1", &[SPACE])?;
            ui.name("revision", &format!("Revision {revision}"))?;
            ui.name("row-0", "Row 0; value 0; clicks 1")?;
            ui.name("row-1", &format!("Row 1; value 0; clicks {revision}"))?;
            for (id, before) in [("row-0", &original), ("row-1", &moved)] {
                unsafe {
                    assert!(
                        ui.client
                            .CompareElements(before, &ui.element(id)?)?
                            .as_bool(),
                        "reordering replaced {id}"
                    );
                }
            }
            ui.wait(
                "rows take their new positions",
                || unsafe {
                    Ok((
                        original.CurrentBoundingRectangle()?.top,
                        moved.CurrentBoundingRectangle()?.top,
                    ))
                },
                |(first, second)| {
                    if revision % 2 == 1 {
                        second < first
                    } else {
                        first < second
                    }
                },
            )?;
            ui.focused("row-1")?;
        }
        Ok(())
    })
}

#[test]
fn repeater_removal_retires_only_removed_rows() -> Result<()> {
    process::run("repeater", |ui| {
        let removed = ui.element("row-0")?;
        let retained = ui.element("row-1")?;
        ui.invoke("row-0")?;
        ui.name("row-0", "Row 0; value 0; clicks 1")?;
        ui.invoke("row-1")?;
        ui.name("row-1", "Row 1; value 0; clicks 1")?;

        ui.invoke("toggle-first")?;
        ui.gone("row-0")?;
        ui.invoke("rerender")?;
        ui.name("drops", "Clicked row 0 drops 1")?;
        ui.name("row-1", "Row 1; value 0; clicks 1")?;
        ui.invoke("toggle-first")?;
        ui.name("row-0", "Row 0; value 0; clicks 0")?;
        ui.name("row-1", "Row 1; value 0; clicks 1")?;
        unsafe {
            assert!(
                !ui.client
                    .CompareElements(&removed, &ui.element("row-0")?)?
                    .as_bool()
            );
            assert!(
                ui.client
                    .CompareElements(&retained, &ui.element("row-1")?)?
                    .as_bool()
            );
        }

        ui.invoke("row-0")?;
        ui.name("row-0", "Row 0; value 0; clicks 1")?;
        ui.invoke("toggle-empty")?;
        ui.gone("row-0")?;
        ui.gone("row-1")?;
        ui.invoke("rerender")?;
        ui.name("drops", "Clicked row 0 drops 2")?;
        ui.invoke("toggle-empty")?;
        ui.name("row-0", "Row 0; value 0; clicks 0")?;
        ui.name("row-1", "Row 1; value 0; clicks 0")?;
        unsafe {
            assert!(
                !ui.client
                    .CompareElements(&retained, &ui.element("row-1")?)?
                    .as_bool()
            );
        }
        Ok(())
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
    rows_requested_during_an_update("repeater_layout")
}

#[test]
fn repeater_realizes_rows_requested_by_a_flip_view_selection() -> Result<()> {
    rows_requested_during_an_update("repeater_layout_flip_view")
}

#[test]
fn repeater_realizes_rows_requested_by_a_pivot_selection() -> Result<()> {
    rows_requested_during_an_update("repeater_layout_pivot")
}

#[test]
fn repeater_keyed_reset_preserves_pending_realizations() -> Result<()> {
    process::run("repeater_layout_keyed_reset", |ui| {
        ui.invoke("load")?;
        ui.name("row-5", "Row 5; value 0; clicks 0")?;
        ui.name("row-4", "Row 4; value 0; clicks 0")?;
        let first = ui.element("row-5")?;
        let second = ui.element("row-4")?;
        ui.wait(
            "pending rows take their keyed positions",
            || unsafe {
                Ok((
                    first.CurrentBoundingRectangle()?.top,
                    second.CurrentBoundingRectangle()?.top,
                ))
            },
            |(first, second)| first < second,
        )?;
        ui.invoke("row-5")?;
        ui.name("row-5", "Row 5; value 0; clicks 1")
    })
}

fn rows_requested_during_an_update(fixture: &'static str) -> Result<()> {
    process::run(fixture, |ui| {
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
