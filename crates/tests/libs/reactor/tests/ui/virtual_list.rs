use super::*;

#[test]
fn virtual_list_row_root_can_change_control_type() -> Result<()> {
    for fixture in ["virtual_list_items", "virtual_list_virtual"] {
        process::run(fixture, |ui| {
            let retained = ui.element("row-1")?;
            ui.invoke("row-0")?;
            ui.name("row-0", "Row 0; value 0; clicks 1")?;
            ui.invoke("toggle-editor")?;
            ui.gone("row-0")?;
            ui.value("row-editor", "")?;
            ui.set_value("row-editor", "draft")?;
            ui.invoke("rerender")?;
            ui.value("row-editor", "draft")?;
            ui.name("drops", "Clicked row 0 drops 1")?;
            ui.invoke("toggle-editor")?;
            ui.gone("row-editor")?;
            ui.name("row-0", "Row 0; value 0; clicks 0")?;
            ui.invoke("row-0")?;
            ui.name("row-0", "Row 0; value 0; clicks 1")?;
            assert!(unsafe {
                ui.client
                    .CompareElements(&retained, &ui.element("row-1")?)?
                    .as_bool()
            });
            Ok(())
        })?;
    }
    Ok(())
}

#[test]
fn virtual_list_items_take_their_row_names() -> Result<()> {
    process::run("virtual_list_virtual", |ui| {
        ui.name("row-0", "Row 0; value 0; clicks 0")?;
        ui.invoke("toggle-editor")?;
        let item = ui.named(LIST_ITEM, "Edit row 0")?;
        ui.invoke("toggle-editor")?;
        ui.name("row-0", "Row 0; value 0; clicks 0")?;
        let name = ui.wait(
            "list item drops the editor name",
            || unsafe { Ok(String::from_utf16_lossy(&item.CurrentName()?)) },
            |name| name != "Edit row 0",
        )?;
        assert_eq!(name, "Row 0; value 0; clicks 0");
        Ok(())
    })
}

#[test]
fn virtual_list_rows_stretch_to_the_viewport() -> Result<()> {
    process::run("virtual_list_virtual", |ui| {
        let viewport = ui.element("rows")?;
        let row = ui.element("row-0")?;
        ui.wait(
            "stretched row fills the viewport",
            || unsafe {
                Ok((
                    viewport.CurrentBoundingRectangle()?,
                    row.CurrentBoundingRectangle()?,
                ))
            },
            |(viewport, row)| {
                let viewport_width = viewport.right - viewport.left;
                let row_width = row.right - row.left;
                // ListViewItem padding insets the row from the viewport edges.
                viewport_width > 0 && row_width >= viewport_width * 9 / 10
            },
        )?;
        Ok(())
    })
}

#[test]
fn virtual_list_creation_and_rendering_follow_visible_changes() -> Result<()> {
    for fixture in ["virtual_list_items", "virtual_list_virtual"] {
        process::run(fixture, |ui| check_row_work(ui, 128))?;
    }
    Ok(())
}

#[test]
fn virtual_list_short_and_mixed_rows_keep_realization_bounded() -> Result<()> {
    for fixture in ["virtual_list_short", "virtual_list_mixed"] {
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
    // WinUI may realize more viewport/cache rows between snapshots. Each new row renders once.
    assert!(unchanged[0] < bound, "{unchanged:?}");
    assert_eq!(unchanged[1] - baseline[1], unchanged[0] - baseline[0]);
    ui.invoke("change")?;
    ui.name("row-0", "Row 0; value 1; clicks 0")?;
    let changed = row_work(ui, 3)?;
    eprintln!("changed: {changed:?}");
    assert!(changed[0] < bound, "{changed:?}");
    assert_eq!(changed[1] - unchanged[1], changed[0] - unchanged[0] + 1);
    Ok(())
}

#[test]
fn virtual_list_short_and_mixed_rows_realize_during_selection() -> Result<()> {
    for fixture in ["virtual_list_layout_short", "virtual_list_layout_mixed"] {
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
fn virtual_list_updates_preserve_realized_component_state() -> Result<()> {
    process::run("virtual_list", |ui| {
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
fn virtual_list_reorder_preserves_realized_state_and_scroll_offset() -> Result<()> {
    const SPACE: i32 = 0x20;
    process::run("virtual_list", |ui| {
        let original = ui.element("row-0")?;
        let moved = ui.element("row-1")?;
        ui.invoke("row-0")?;
        ui.name("row-0", "Row 0; value 0; clicks 1")?;
        let top = unsafe { original.CurrentBoundingRectangle()?.top };
        for revision in 1..=3 {
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
                // The list keeps its scroll offset rather than following a moved row.
                |(first, second)| {
                    *first.min(second) == top
                        && if revision % 2 == 1 {
                            second < first
                        } else {
                            first < second
                        }
                },
            )?;
        }
        Ok(())
    })
}

#[test]
fn virtual_list_removal_retires_only_removed_rows() -> Result<()> {
    process::run("virtual_list", |ui| {
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
fn virtual_list_reset_keeps_a_scrolled_viewport() -> Result<()> {
    process::run("virtual_list", |ui| check_scrolled_reset(ui, 256))
}

#[test]
fn virtual_list_mixed_reset_keeps_a_scrolled_viewport() -> Result<()> {
    process::run("virtual_list_mixed", |ui| check_scrolled_reset(ui, 10_000))
}

fn check_scrolled_reset(ui: &Automation, count: usize) -> Result<()> {
    ui.name("row-0", "Row 0; value 0; clicks 0")?;
    let list = ui.element("rows")?;
    let scroll = unsafe { list.GetCurrentPatternAs::<IUIAutomationScrollPattern>(10004)? };
    let offset = || unsafe {
        let viewport = list.CurrentBoundingRectangle()?;
        let extent =
            f64::from(viewport.bottom - viewport.top) * 100.0 / scroll.CurrentVerticalViewSize()?;
        Ok((extent - f64::from(viewport.bottom - viewport.top))
            * scroll.CurrentVerticalScrollPercent()?
            / 100.0)
    };
    let visible = || unsafe {
        let viewport = list.CurrentBoundingRectangle()?;
        let all = ui.client.CreateTrueCondition()?;
        let items = list.FindAll(TreeScope_Descendants, &all)?;
        let mut rows = Vec::new();
        for index in 0..items.Length()? {
            let item = items.GetElement(index)?;
            let id = String::from_utf16_lossy(&item.CurrentAutomationId()?);
            if let Some(key) = id.strip_prefix("row-") {
                let rect = item.CurrentBoundingRectangle()?;
                if rect.bottom > viewport.top && rect.top < viewport.bottom {
                    rows.push((key.parse::<usize>().unwrap(), rect.top));
                }
            }
        }
        rows.sort_unstable_by_key(|(_, top)| *top);
        Ok(rows)
    };
    ui.invoke("select-first")?;
    ui.selected_name("rows", "Row 0; value 0; clicks 0")?;
    unsafe { scroll.SetScrollPercent(-1.0, 40.0).ok()? };
    let before = ui.wait("rows are visible away from the top", visible, |rows| {
        rows.len() >= 2 && rows[0].0 > 20
    })?;
    let original_offset = offset()?;
    eprintln!("before reverse: {before:?}; offset={original_offset}");
    for revision in 2..=4 {
        ui.invoke("reverse")?;
        ui.name("revision", &format!("Revision {revision}"))?;
        let reversed = revision % 2 == 0;
        let after = ui.wait(
            "reordered rows keep the scrolled viewport",
            visible,
            |rows| {
                rows.len() >= 2 && (rows[0].0 > rows[1].0) == reversed && {
                    let index = if reversed {
                        count - 1 - rows[0].0
                    } else {
                        rows[0].0
                    };
                    index > count / 4 && index < count / 2
                }
            },
        )?;
        eprintln!("after reorder: {after:?}");
        ui.wait("reset preserves pixel scroll offset", offset, |offset| {
            (offset - original_offset).abs() < 1.0
        })?;
        if count == 256 {
            let index = if reversed {
                count - 1 - after[0].0
            } else {
                after[0].0
            };
            assert_eq!(index, before[0].0);
            assert_eq!(after[0].1, before[0].1);
        }
    }
    ui.invoke("shorten")?;
    ui.name("revision", "Revision 5")?;
    ui.wait("short source clamps the scroll offset", offset, |offset| {
        offset.abs() < 1.0
    })?;
    ui.name(
        &format!("row-{}", count - 1),
        &format!("Row {}; value 0; clicks 0", count - 1),
    )?;
    ui.invoke("toggle-empty")?;
    ui.wait("empty source releases its rows", visible, Vec::is_empty)?;
    ui.invoke("toggle-empty")?;
    ui.name("row-0", "Row 0; value 0; clicks 0")?;
    Ok(())
}

#[test]
fn virtual_list_drag_reorders_keyed_rows() -> Result<()> {
    process::run("virtual_list_virtual", |ui| {
        ui.invoke("row-0")?;
        ui.name("row-0", "Row 0; value 0; clicks 1")?;
        let original = ui.element("row-0")?;
        let first = ui.list_item("rows", "Row 0; value 0; clicks 1")?;
        let list = ui.element("rows")?;
        let start = unsafe { first.CurrentBoundingRectangle()? };
        let bounds = unsafe { list.CurrentBoundingRectangle()? };
        ui.pointer_move_to(&first, "first row", 0.02, 0.5)?;
        ui.pointer_button(true)?;
        for step in 1..=8 {
            let y = f64::from(start.top - bounds.top)
                + f64::from(start.bottom - start.top) * (0.5 + f64::from(step) * 0.4);
            ui.pointer_move_to(
                &list,
                "drag target",
                0.02,
                y / f64::from(bounds.bottom - bounds.top),
            )?;
            std::thread::sleep(Duration::from_millis(50));
        }
        ui.pointer_button(false)?;
        ui.name("revision", "Revision 1")?;
        ui.name("row-0", "Row 0; value 0; clicks 1")?;
        let moved = ui.element("row-0")?;
        let second = ui.element("row-1")?;
        ui.wait(
            "dragged row follows row 1",
            || unsafe {
                Ok((
                    moved.CurrentBoundingRectangle()?.top,
                    second.CurrentBoundingRectangle()?.top,
                ))
            },
            |(first, second)| first > second,
        )?;
        assert!(unsafe {
            ui.client
                .CompareElements(&original, &ui.element("row-0")?)?
                .as_bool()
        });
        Ok(())
    })
}

#[test]
fn virtual_list_scrolling_realizes_only_nearby_rows() -> Result<()> {
    const SCROLL_PATTERN: i32 = 10004;
    const NO_SCROLL: f64 = -1.0;
    process::run("virtual_list", |ui| {
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
fn virtual_list_realizes_rows_requested_during_an_update() -> Result<()> {
    rows_requested_during_an_update("virtual_list_layout")
}

#[test]
fn virtual_list_realizes_rows_requested_by_a_flip_view_selection() -> Result<()> {
    rows_requested_during_an_update("virtual_list_layout_flip_view")
}

#[test]
fn virtual_list_realizes_rows_requested_by_a_pivot_selection() -> Result<()> {
    rows_requested_during_an_update("virtual_list_layout_pivot")
}

#[test]
fn virtual_list_keyed_reset_preserves_pending_realizations() -> Result<()> {
    process::run("virtual_list_layout_keyed_reset", |ui| {
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
