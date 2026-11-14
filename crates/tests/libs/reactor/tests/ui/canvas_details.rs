use super::*;

#[test]
fn custom_details_navigate_sort_resize_and_remove_during_updates() -> Result<()> {
    process::run("canvas-details", |ui| {
        ui.press_keys("custom-details", &[0x23])?;
        ui.name("selected", "9999")?;
        ui.pointer_move("custom-details", 30.0 / 600.0, 0.03)?;
        ui.pointer_clicks(1)?;
        ui.name("sorted", "Some(0)")?;
        ui.name("selected", "9999")?;
        ui.press_keys("custom-details", &[0x28])?;
        ui.name("selected", "9998")?;
        ui.pointer_move("custom-details", 182.0 / 600.0, 0.03)?;
        cursor(ui, windows_core::PCWSTR(IDC_SIZEWE.0.cast()))?;
        ui.pointer_button(true)?;
        ui.pointer_move("custom-details", 242.0 / 600.0, 0.03)?;
        ui.pointer_button(false)?;
        ui.pointer_move("custom-details", 210.0 / 600.0, 0.03)?;
        ui.pointer_clicks(1)?;
        ui.name("sorted", "Some(0)")?;
        ui.press_keys("custom-details", &[0x24])?;
        ui.name("selected", "0")?;
        ui.pointer_move("custom-details", 0.1, 0.12)?;
        cursor(ui, windows_core::PCWSTR(IDC_ARROW.0.cast()))?;
        ui.pointer_wheel(-120)?;
        ui.pointer_clicks(1)?;
        let mut lines = 0u32;
        if !unsafe {
            SystemParametersInfoW(
                SPI_GETWHEELSCROLLLINES as u32,
                0,
                (&mut lines as *mut u32).cast(),
                0,
            )
        }
        .as_bool()
        {
            return Err(windows_core::Error::from_thread().into());
        }
        let body_height = 388.0 - 32.0 - 14.0;
        let offset = if lines == u32::MAX {
            body_height
        } else {
            f64::from(lines) * 26.0
        };
        let offset = offset.min(10_000.0 * 26.0 - body_height);
        let selected = ((offset + 388.0 * 0.12 - 32.0) / 26.0).floor() as usize;
        ui.name("selected", &selected.to_string())?;
        ui.invoke("remove")?;
        ui.name("selected", "None")?;
        ui.press_keys("custom-details", &[0x28])?;
        ui.name("selected", if selected == 0 { "1" } else { "0" })?;
        ui.focused("custom-details")
    })
}

#[test]
fn custom_details_keep_selection_and_drag_ownership_during_key_churn() -> Result<()> {
    process::run("canvas-details", |ui| {
        ui.press_keys("custom-details", &[0x24])?;
        ui.name("selected", "0")?;
        ui.invoke("churn")?;
        ui.name_stays("selected", "0", Duration::from_millis(350))?;
        ui.pointer_move("custom-details", 182.0 / 600.0, 0.03)?;
        cursor(ui, windows_core::PCWSTR(IDC_SIZEWE.0.cast()))?;
        ui.pointer_button(true)?;
        ui.name_stays("selected", "0", Duration::from_millis(350))?;
        ui.pointer_move("custom-details", 242.0 / 600.0, 0.03)?;
        ui.pointer_button(false)?;
        ui.pointer_move("custom-details", 210.0 / 600.0, 0.03)?;
        ui.pointer_clicks(1)?;
        ui.name("sorted", "Some(0)")?;
        ui.move_resize_window(100, 100, 800, 600)?;
        ui.pointer_move("custom-details", 0.1, 0.2)?;
        ui.pointer_wheel(-120)?;
        ui.name_stays("selected", "0", Duration::from_millis(350))?;
        ui.invoke("churn")?;
        ui.name("churn", "Start churn")?;
        let last = String::from_utf16(&unsafe { ui.element("last")?.CurrentName()? })?;
        ui.press_keys("custom-details", &[0x23])?;
        ui.name("selected", &last)?;
        ui.invoke("remove")?;
        ui.name("selected", "None")
    })
}

fn cursor(ui: &Automation, shape: windows_core::PCWSTR) -> Result<()> {
    let expected = unsafe { LoadCursorW(std::ptr::null_mut(), shape) };
    if expected.is_null() {
        return Err(windows_core::Error::from_thread().into());
    }
    ui.wait(
        "pointer shows the expected system cursor",
        || {
            let mut info = CURSORINFO {
                cbSize: size_of::<CURSORINFO>() as u32,
                ..Default::default()
            };
            if !unsafe { GetCursorInfo(&mut info) }.as_bool() {
                return Err(windows_core::Error::from_thread());
            }
            Ok(info.hCursor == expected)
        },
        |matches| *matches,
    )?;
    Ok(())
}
