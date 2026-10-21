use super::*;

const VK_LEFT: i32 = 0x25;

#[test]
fn text_box_transform_keeps_caret_without_echo() -> Result<()> {
    process::run("text-transform", |ui| transform(ui, "input"))
}

#[test]
fn auto_suggest_box_transform_keeps_caret_without_echo() -> Result<()> {
    // The editable template part carries the value and keyboard focus.
    process::run("suggest-transform", |ui| transform(ui, "TextBox"))
}

/// The fixture uppercases each edit. Each keystroke must reach the app once, the caret must stay
/// where the user left it after the app rewrites the text, and header changes must not recreate
/// the control or disturb its value.
fn transform(ui: &Automation, edit: &str) -> Result<()> {
    ui.value(edit, "")?;
    ui.element("header")?;
    for (typed, expected) in [("a", "A"), ("b", "AB")] {
        ui.type_text(edit, typed)?;
        ui.value(edit, expected)?;
    }
    settled(ui, "Text AB; calls 2")?;

    ui.press_keys(edit, &[VK_LEFT])?;
    for (typed, expected) in [("x", "AXB"), ("y", "AXYB")] {
        ui.type_text(edit, typed)?;
        ui.value(edit, expected)?;
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
    ui.value(edit, "AXYB")?;
    settled(ui, "Text AXYB; calls 4")
}

fn settled(ui: &Automation, expected: &str) -> Result<()> {
    ui.name("echo", expected)?;
    ui.name_stays("echo", expected, Duration::from_millis(250))
}
