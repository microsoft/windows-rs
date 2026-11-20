use super::*;

const VK_LEFT: i32 = 0x25;

#[test]
fn text_box_padding_and_corners_update_and_clear_in_place() -> Result<()> {
    process::run("text-appearance", |ui| {
        let _physical = PhysicalCoordinates::new()?;
        ui.focus("anchor")?;
        ui.pointer_move("anchor", 0.5, 0.5)?;
        let element = ui.element("sizing")?;
        let measure = || unsafe {
            let bounds = element.CurrentBoundingRectangle()?;
            let width = bounds.right - bounds.left;
            let height = bounds.bottom - bounds.top;
            let dc = GetDC(std::ptr::null_mut());
            assert!(!dc.is_null(), "GetDC failed");
            let pixel = GetPixel(dc, bounds.left + height / 10, bounds.top + height / 10);
            let interior = GetPixel(dc, bounds.left + width / 2, bounds.top + height / 10);
            assert_ne!(ReleaseDC(std::ptr::null_mut(), dc), 0);
            assert_ne!(pixel, u32::MAX, "GetPixel failed");
            assert_ne!(interior, u32::MAX, "GetPixel failed");
            Ok((width, height, pixel, interior))
        };
        let red = |pixel: u32| pixel & 0xff > 200 && pixel & 0xffff00 == 0;
        let baseline = ui.wait(
            "native padding and corner baseline",
            measure,
            |&(w, h, p, interior)| w > 0 && h > 0 && red(p) && red(interior),
        )?;
        ui.invoke("apply")?;
        let applied = ui.wait(
            "padding grows both axes and rounds the corner",
            measure,
            |&(w, h, p, interior)| {
                w > baseline.0 + 40 && h > baseline.1 + 20 && p == 0xffffff && red(interior)
            },
        )?;
        ui.invoke("change")?;
        ui.wait(
            "new padding shrinks both axes and zero radius squares the corner",
            measure,
            |&(w, h, p, interior)| w < applied.0 && h < applied.1 && red(p) && red(interior),
        )?;
        ui.invoke("apply")?;
        ui.wait(
            "restore large padding and corners before clearing",
            measure,
            |sample| *sample == applied,
        )?;
        ui.invoke("reset")?;
        ui.wait(
            "omitted properties restore native defaults",
            measure,
            |sample| *sample == baseline,
        )?;
        ui.press_keys("sizing", &[0x23])?;
        ui.type_text("sizing", "N")?;
        ui.value("sizing", "MMMMMMMMMMMMN")?;
        for style in ["1", "2", "0"] {
            ui.press_keys("sizing", &[0x11, 0x4e])?;
            ui.name("style", style)?;
            ui.focused("sizing")?;
            ui.value("sizing", "MMMMMMMMMMMMN")?;
        }
        ui.type_text("sizing", "O")?;
        ui.value("sizing", "MMMMMMMMMMMMNO")?;
        assert!(unsafe {
            ui.client
                .CompareElements(&element, &ui.element("sizing")?)?
                .as_bool()
        });
        Ok(())
    })
}

#[test]
fn decorative_overlay_hit_testing_updates_and_clears() -> Result<()> {
    process::run("text-appearance", |ui| {
        let input = ui.element("input")?;
        ui.pointer_enter("input", "entered", "true")?;
        let click_icon = || -> Result<()> {
            ui.focus("anchor")?;
            ui.pointer_move("input", 28.0 / 320.0, 0.5)?;
            ui.pointer_clicks(1)
        };
        click_icon()?;
        ui.focused("input")?;
        ui.type_text("input", "a")?;
        ui.name("echo", "a")?;
        ui.name("presses", "0")?;

        ui.invoke("block")?;
        click_icon()?;
        ui.name("presses", "1")?;
        assert!(!unsafe {
            ui.client
                .CompareElements(&ui.client.GetFocusedElement()?, &input)?
                .as_bool()
        });

        ui.invoke("pass")?;
        click_icon()?;
        ui.focused("input")?;
        ui.name("presses", "1")?;

        ui.invoke("omit-hit")?;
        click_icon()?;
        ui.name("presses", "2")?;
        assert!(!unsafe {
            ui.client
                .CompareElements(&ui.client.GetFocusedElement()?, &input)?
                .as_bool()
        });

        ui.invoke("pass")?;
        click_icon()?;
        ui.focused("input")?;
        ui.value("input", "a")?;
        assert!(unsafe {
            ui.client
                .CompareElements(&input, &ui.element("input")?)?
                .as_bool()
        });
        Ok(())
    })
}

#[test]
fn multiline_text_is_applied_after_its_mode() -> Result<()> {
    process::run("text-multiline", |ui| {
        ui.value("initial", "First\rSecond")?;
        ui.value("changing", "First")?;
        ui.value("retained", "First")?;
        let input = ui.element("changing")?;
        for expected in ["First\rSecond", "First", "First\rSecond"] {
            ui.invoke("toggle")?;
            ui.value("changing", expected)?;
            ui.value("retained", expected)?;
            ui.name_stays("calls", "Calls 0", Duration::from_millis(250))?;
        }
        ui.invoke("omit-mode")?;
        ui.value("changing", "First")?;
        ui.value("retained", "First")?;
        ui.name_stays("calls", "Calls 0", Duration::from_millis(250))?;
        ui.invoke("toggle")?;
        ui.value("changing", "First\rSecond")?;
        ui.value("retained", "First\rSecond")?;
        ui.name_stays("calls", "Calls 0", Duration::from_millis(250))?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&input, &ui.element("changing")?)?
                    .as_bool(),
                "changing multiline mode recreated the input"
            );
        }
        Ok(())
    })
}

#[test]
fn text_box_transform_keeps_caret_without_echo() -> Result<()> {
    process::run("text-transform", |ui| {
        transform(ui, "input", Automation::value)
    })
}

#[test]
fn auto_suggest_box_transform_keeps_caret_without_echo() -> Result<()> {
    // The editable template part carries the value and keyboard focus.
    process::run("suggest-transform", |ui| {
        transform(ui, "TextBox", Automation::value)
    })
}

#[test]
fn rich_edit_box_transform_keeps_caret_without_echo() -> Result<()> {
    // RichEditBox exposes its document through the text pattern rather than a value.
    process::run("rich-transform", |ui| {
        transform(ui, "input", Automation::text)
    })
}

/// The fixture uppercases each edit. Each keystroke must reach the app once, the caret must stay
/// where the user left it after the app rewrites the text, and header changes must not recreate
/// the control or disturb its value.
fn transform(
    ui: &Automation,
    edit: &str,
    value: fn(&Automation, &str, &str) -> Result<()>,
) -> Result<()> {
    value(ui, edit, "")?;
    ui.element("header")?;
    for (typed, expected) in [("a", "A"), ("b", "AB")] {
        ui.type_text(edit, typed)?;
        value(ui, edit, expected)?;
    }
    settled(ui, "Text AB; calls 2")?;

    ui.press_keys(edit, &[VK_LEFT])?;
    for (typed, expected) in [("x", "AXB"), ("y", "AXYB")] {
        ui.type_text(edit, typed)?;
        value(ui, edit, expected)?;
    }
    settled(ui, "Text AXYB; calls 4")?;

    let input = ui.element("input")?;
    ui.invoke("toggle-header")?;
    ui.gone("header")?;
    ui.invoke("toggle-header")?;
    ui.element("header")?;
    unsafe {
        assert!(
            ui.client
                .CompareElements(&input, &ui.element("input")?)?
                .as_bool(),
            "header change recreated the input"
        );
    }
    value(ui, edit, "AXYB")?;
    settled(ui, "Text AXYB; calls 4")
}

fn settled(ui: &Automation, expected: &str) -> Result<()> {
    ui.name("echo", expected)?;
    ui.name_stays("echo", expected, Duration::from_millis(250))
}
