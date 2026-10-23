use super::*;

const FOREGROUND_COLOR: i32 = 40008;

fn foreground(range: &IUIAutomationTextRange) -> windows_core::Result<i32> {
    unsafe {
        let mut value = range.GetAttributeValue(FOREGROUND_COLOR)?;
        let color = if value.Anonymous.Anonymous.vt == VT_I4 as u16 {
            Ok(value.Anonymous.Anonymous.Anonymous.lVal)
        } else {
            Err(windows_core::Error::new(
                E_UNEXPECTED,
                "Text provider did not return a foreground color",
            ))
        };
        VariantClear(&mut value).ok()?;
        color
    }
}

fn follows_owner(ui: &Automation, requested: &str) -> Result<IUIAutomationElement> {
    let element = ui.element("scheme")?;
    let range = unsafe {
        element
            .GetCurrentPatternAs::<IUIAutomationTextPattern>(TEXT_PATTERN)?
            .DocumentRange()?
    };
    ui.wait(
        "dialog text color follows the window's observed color scheme",
        || Ok((unsafe { element.CurrentName()? }, foreground(&range)?)),
        |(name, color)| {
            if requested != "System" {
                name == &format!("Requested {requested}; owner Some({requested})")
                    && *color == if requested == "Light" { 0 } else { 0xffffff }
            } else {
                (name == "Requested System; owner Some(Light)" && *color == 0)
                    || (name == "Requested System; owner Some(Dark)" && *color == 0xffffff)
            }
        },
    )?;
    Ok(element)
}

fn changes_and_reopens(ui: &Automation, initial: &str) -> Result<()> {
    let original = follows_owner(ui, initial)?;
    for theme in ["Dark", "Light", "System"] {
        ui.invoke(&format!("dialog-{}", theme.to_lowercase()))?;
        let current = follows_owner(ui, theme)?;
        assert!(unsafe { ui.client.CompareElements(&original, &current)?.as_bool() });
    }
    for (generation, theme) in [(1, "Dark"), (0, "Light")] {
        ui.invoke("replace-owner")?;
        ui.name("owner-generation", &format!("Owner {generation}"))?;
        ui.invoke(&format!("dialog-{}", theme.to_lowercase()))?;
        let current = follows_owner(ui, theme)?;
        assert!(unsafe { ui.client.CompareElements(&original, &current)?.as_bool() });
    }
    for theme in ["Dark", "Light"] {
        close(ui)?;
        ui.invoke(&format!("window-{}", theme.to_lowercase()))?;
        ui.invoke("open")?;
        let current = follows_owner(ui, theme)?;
        assert!(unsafe { ui.client.CompareElements(&original, &current)?.as_bool() });
    }
    close(ui)
}

// UIA drops the dialog content when the close animation starts, but `on_closed` runs after it
// ends. Wait for the app to observe the close so a later `open` is not absorbed by the old state.
fn close(ui: &Automation) -> Result<()> {
    ui.invoke("CloseButton")?;
    ui.gone("scheme")?;
    ui.name("dialog-state", "Closed")
}

#[test]
fn dialog_follows_light_window_at_mount_and_theme_changes() -> Result<()> {
    process::run("dialog-theme-light", |ui| changes_and_reopens(ui, "Light"))
}

#[test]
fn dialog_follows_dark_window_at_mount_and_theme_changes() -> Result<()> {
    process::run("dialog-theme-dark", |ui| changes_and_reopens(ui, "Dark"))
}

#[test]
fn dialog_created_after_window_theme_is_applied() -> Result<()> {
    process::run("dialog-theme-late", |ui| {
        ui.invoke("open")?;
        changes_and_reopens(ui, "Light")
    })
}
