use super::*;

const QUIET: Duration = Duration::from_millis(500);
const BUTTON: i32 = 50000;
const TOOL_TIP: i32 = 50022;

#[test]
fn action_controls_report_each_invocation() -> Result<()> {
    process::run("smoke", |ui| {
        let mut events = Vec::new();
        for (id, event) in [
            ("hyperlink", "hyperlink"),
            ("repeat", "repeat"),
            ("split", "split"),
            ("app-bar", "app-bar"),
            ("PART_BackButton", "back"),
        ] {
            ui.invoke(id)?;
            events.push(event);
            ui.name("events", &format!("Events [{}]", events.join(", ")))?;
        }
        unsafe {
            ui.named(BUTTON, "Leaf")?
                .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
                .Invoke()
                .ok()?;
        }
        events.push("crumb Leaf");
        let expected = format!("Events [{}]", events.join(", "));
        ui.name("events", &expected)?;
        ui.name_stays("events", &expected, QUIET)
    })
}

#[test]
fn display_objects_update_remove_and_remount() -> Result<()> {
    process::run("smoke", |ui| {
        displayed(ui, "Gen 0", 30.0)?;
        ui.invoke("advance")?;
        displayed(ui, "Gen 1", 60.0)?;
        ui.invoke("hide")?;
        for id in DISPLAYED {
            ui.gone(id)?;
        }
        ui.invoke("advance")?;
        ui.invoke("show")?;
        displayed(ui, "Gen 2", 30.0)
    })
}

#[test]
fn tooltip_shows_and_updates_declared_text() -> Result<()> {
    process::run("smoke", |ui| {
        // Enter from another element: a move that ends where the cursor already rests does not
        // raise PointerEntered, so the tooltip would never open.
        ui.pointer_move("advance", 0.5, 0.5)?;
        ui.pointer_move("tipped", 0.5, 0.5)?;
        ui.named(TOOL_TIP, "Gen 0")?;
        // WinUI owns how long an open tooltip stays up, so update while away and hover again.
        ui.pointer_move("advance", 0.5, 0.5)?;
        ui.invoke("advance")?;
        ui.pointer_move("tipped", 0.5, 0.5)?;
        ui.named(TOOL_TIP, "Gen 1")?;
        Ok(())
    })
}

const DISPLAYED: [&str; 12] = [
    "viewbox-text",
    "scroll-text",
    "wrap-text",
    "relative-text",
    "list-item",
    "grid-item",
    "rich",
    "progress-bar",
    "progress-ring",
    "date",
    "time",
    "tipped",
];

/// Waits for the display objects to show `label` and `value`.
fn displayed(ui: &Automation, label: &str, value: f64) -> Result<()> {
    for id in &DISPLAYED[..6] {
        ui.name(id, label)?;
    }
    let rich = ui.element("rich")?;
    ui.wait(
        &format!("\"rich\" text becomes {label:?}"),
        || unsafe { rich.CurrentName() },
        |name| String::from_utf16_lossy(name).trim_end() == label,
    )?;
    for id in ["progress-bar", "progress-ring"] {
        let element = ui.element(id)?;
        ui.wait(
            &format!("{id:?} value becomes {value}"),
            || unsafe {
                element
                    .GetCurrentPatternAs::<IUIAutomationRangeValuePattern>(RANGE_VALUE_PATTERN)?
                    .CurrentValue()
            },
            |current| *current == value,
        )?;
    }
    for id in ["date", "time", "tipped"] {
        ui.element(id)?;
    }
    Ok(())
}
