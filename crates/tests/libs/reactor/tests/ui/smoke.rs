use super::*;

const QUIET: Duration = Duration::from_millis(500);
const BUTTON: i32 = 50000;
const TOOL_TIP: i32 = 50022;

fn icon_bounds(
    element: &IUIAutomationElement,
) -> windows_core::Result<Option<(i32, i32, i32, i32)>> {
    let (width, height, pixels) = snapshot(element)?;
    let (mut left, mut top, mut right, mut bottom) = (width, height, -1, -1);
    for (index, pixel) in pixels.into_iter().enumerate() {
        if (pixel >> 16) & 0xff > 200 && (pixel >> 8) & 0xff < 60 && pixel & 0xff < 60 {
            let x = index as i32 % width;
            let y = index as i32 / width;
            left = left.min(x);
            right = right.max(x);
            top = top.min(y);
            bottom = bottom.max(y);
        }
    }
    Ok((right >= left).then_some((left, top, right, bottom)))
}

#[test]
fn button_padding_unclips_content_updates_and_clears() -> Result<()> {
    process::run("button-padding", |ui| {
        let _physical = PhysicalCoordinates::new()?;
        ui.pointer_move("apply", 0.5, 0.5)?;
        let button = ui.element("padded")?;
        let reference = ui.element("reference")?;
        let full = ui
            .wait(
                "reference icon is drawn",
                || icon_bounds(&reference),
                Option::is_some,
            )?
            .unwrap();
        let baseline = ui
            .wait(
                "button icon is drawn",
                || icon_bounds(&button),
                Option::is_some,
            )?
            .unwrap();
        eprintln!("default button icon {baseline:?}; unclipped reference {full:?}");
        assert!(baseline.2 - baseline.0 < full.2 - full.0);
        let unclipped = |sample: &Option<(i32, i32, i32, i32)>| {
            sample.is_some_and(|(left, top, right, bottom)| {
                (right - left - (full.2 - full.0)).abs() <= 1
                    && (bottom - top - (full.3 - full.1)).abs() <= 1
            })
        };
        let initial = ui.element("initial")?;
        ui.wait(
            "initial padding reveals the complete icon",
            || icon_bounds(&initial),
            unclipped,
        )?;
        ui.invoke("apply")?;
        let applied = ui
            .wait(
                "padding reveals the complete icon",
                || icon_bounds(&button),
                unclipped,
            )?
            .unwrap();
        ui.invoke("padded")?;
        ui.name("clicks", "1")?;
        ui.invoke("asymmetric")?;
        let width = unsafe { button.CurrentBoundingRectangle()? };
        let dpi = f64::from(width.right - width.left) / 40.0;
        ui.wait(
            "asymmetric padding moves the complete icon",
            || icon_bounds(&button),
            |sample| {
                unclipped(sample)
                    && sample.is_some_and(|(left, top, _, _)| {
                        (f64::from(left - applied.0) - 6.0 * dpi).abs() <= 1.0
                            && (f64::from(top - applied.1) - 5.0 * dpi).abs() <= 1.0
                    })
            },
        )?;
        ui.invoke("zero")?;
        ui.wait(
            "zero padding recenters the complete icon",
            || icon_bounds(&button),
            |sample| *sample == Some(applied),
        )?;
        ui.invoke("reset")?;
        ui.wait(
            "omitting padding restores the native default",
            || icon_bounds(&button),
            |sample| *sample == Some(baseline),
        )?;
        ui.invoke("padded")?;
        ui.name("clicks", "2")?;
        assert!(unsafe {
            ui.client
                .CompareElements(&button, &ui.element("padded")?)?
                .as_bool()
        });
        Ok(())
    })
}

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
        displayed(ui, "Gen 2", 30.0)?;
        // The image-backed icon element and icon source both render.
        shows_red(ui, "AppBarButton image icon", "image-icon", |button| {
            let height = button.bottom - button.top;
            (button.top + height * 47 / 100, button.left, button.right)
        })?;
        let back = unsafe { ui.element("PART_BackButton")?.CurrentBoundingRectangle()? };
        shows_red(ui, "TitleBar SVG icon", "PART_TitleText", |title| {
            ((title.top + title.bottom) / 2, back.right, title.left)
        })
    })
}

/// Waits for a red pixel in the row `region` derives from the element's physical bounds.
fn shows_red(
    ui: &Automation,
    description: &str,
    id: &str,
    region: impl Fn(RECT) -> (i32, i32, i32),
) -> Result<()> {
    let _physical = PhysicalCoordinates::new()?;
    let element = ui.element(id)?;
    ui.wait(
        &format!("{description} shows red"),
        || unsafe {
            let (y, left, right) = region(element.CurrentBoundingRectangle()?);
            let dc = GetDC(std::ptr::null_mut());
            assert!(!dc.is_null(), "GetDC failed");
            let red = (left..right).any(|x| {
                let pixel = GetPixel(dc, x, y);
                pixel != u32::MAX
                    && pixel & 0xff > 200
                    && (pixel >> 8) & 0xff < 60
                    && (pixel >> 16) & 0xff < 60
            });
            assert_ne!(ReleaseDC(std::ptr::null_mut(), dc), 0);
            Ok(red)
        },
        |red| *red,
    )?;
    Ok(())
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

const DISPLAYED: [&str; 15] = [
    "viewbox-text",
    "scroll-text",
    "wrap-text",
    "relative-text",
    "list-item",
    "grid-item",
    "weighted-text",
    "styled",
    "uri-link",
    "rich",
    "progress-bar",
    "progress-ring",
    "date",
    "time",
    "tipped",
];

/// Waits for the display objects to show `label` and `value`.
fn displayed(ui: &Automation, label: &str, value: f64) -> Result<()> {
    for id in &DISPLAYED[..9] {
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
