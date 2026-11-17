use super::*;

const LIST_ITEM: i32 = 50007;
const DATA_ITEM: i32 = 50029;
const VK_DOWN: i32 = 0x28;
const VK_SPACE: i32 = 0x20;

/// Raises each event from native input and checks the converted payload arrives once.
#[test]
fn native_events_report_converted_arguments() -> Result<()> {
    process::run("events", |ui| {
        ui.name("mode", "Mode Some(Expanded)")?;
        let mut events = Vec::new();
        let mut expect = |event: &'static str| -> Result<()> {
            events.push(event);
            ui.name("events", &format!("Events [{}]", events.join(", ")))
        };

        ui.invoke("PART_PaneToggleButton")?;
        expect("pane-toggle")?;
        ui.invoke("AddButton")?;
        expect("add-tab")?;
        ui.pointer_move("double-target", 0.5, 0.5)?;
        ui.pointer_clicks(2)?;
        expect("double")?;

        let calendar = ui.element("calendar")?;
        let day = ui.wait(
            "find calendar day 15",
            || unsafe {
                let all = ui.client.CreateTrueCondition()?;
                let items = calendar.FindAll(TreeScope_Descendants, &all)?;
                for index in 0..items.Length()? {
                    let item = items.GetElement(index)?;
                    if item.CurrentControlType()? == DATA_ITEM && item.CurrentName()? == "15" {
                        return Ok(Some(item));
                    }
                }
                Ok(None)
            },
            Option::is_some,
        )?;
        let day = day.unwrap();
        unsafe { day.SetFocus().ok()? };
        ui.press_keys_on(&day, "15", &[VK_SPACE])?;
        expect("dates")?;

        ui.type_text("TextBox", "a")?;
        ui.named(LIST_ITEM, "Alpha")?;
        ui.press_keys("TextBox", &[VK_DOWN])?;
        expect("chosen Alpha")?;

        ui.invoke("FlyoutButton")?;
        ui.invoke("AcceptButton")?;
        expect("time set")?;

        ui.invoke("minimal")?;
        ui.name("mode", "Mode Some(Minimal)")?;
        let expected = format!("Events [{}]", events.join(", "));
        ui.name_stays("events", &expected, Duration::from_millis(500))
    })
}
