use super::*;

pub(super) fn start(ui: &Automation) -> Result<()> {
    ui.focus("baseline")?;
    ui.pointer_move("source", 0.5, 0.5)?;
    ui.invoke("start")?;
    ui.name("result", "Ready")?;
    ui.pointer_button(true)?;
    ui.pointer_move("source", 0.6, 0.5)?;
    ui.name("feedback", "None")
}

fn drop_copy(ui: &Automation) -> Result<()> {
    start(ui)?;
    ui.pointer_move("target", 0.5, 0.5)?;
    ui.name("feedback", "Copy")?;
    ui.pointer_button(false)?;
    ui.name("result", "Dropped Copy; text read true")
}

#[test]
fn drop_policy_at_mount() -> Result<()> {
    process::run("drag-mounted", drop_copy)
}

#[test]
fn drop_policy_added_to_existing_border() -> Result<()> {
    process::run("drag-added", |ui| {
        let original = ui.element("target")?;
        ui.name("policy", "None")?;
        start(ui)?;
        ui.pointer_move("target", 0.5, 0.5)?;
        ui.pointer_button(false)?;
        ui.name("result", "Not accepted")?;
        ui.invoke("copy")?;
        ui.name("policy", "Copy")?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&original, &ui.element("target")?)?
                    .as_bool()
            );
        }
        drop_copy(ui)
    })
}

#[test]
fn drop_policy_changes_during_drag() -> Result<()> {
    process::run("drag-mounted", |ui| {
        start(ui)?;
        ui.pointer_move("target", 0.5, 0.5)?;
        ui.name("feedback", "Copy")?;
        ui.invoke("move")?;
        ui.name("policy", "Move")?;
        ui.pointer_move("target", 0.75, 0.5)?;
        ui.name("feedback", "Move")?;
        ui.invoke("copy")?;
        ui.name("policy", "Copy")?;
        ui.pointer_button(false)?;
        ui.name("result", "Dropped Copy; text read true")
    })
}

#[test]
fn cancelled_drag_can_be_restarted() -> Result<()> {
    process::run("drag-mounted", |ui| {
        let mut source = None;
        for _ in 0..3 {
            start(ui)?;
            let current = source_window(ui)?;
            if let Some(previous) = &source {
                assert!(unsafe { ui.client.CompareElements(previous, &current)? }.as_bool());
            }
            source = Some(current);
            ui.pointer_move("target", 0.5, 0.5)?;
            ui.name("feedback", "Copy")?;
            ui.invoke("cancel")?;
            ui.pointer_move("target", 0.75, 0.5)?;
            ui.name("result", "Cancelled")?;
            assert_eq!(
                unsafe { source.as_ref().unwrap().CurrentName()? },
                "Drag text"
            );
            ui.pointer_button(false)?;
        }
        drop_copy(ui)
    })
}

fn source_window(ui: &Automation) -> Result<IUIAutomationElement> {
    unsafe {
        let window = ui
            .client
            .ElementFromHandle(ui.window_handle()? as UIA_HWND)?;
        let elements = window.FindAll(TreeScope_Descendants, &ui.client.CreateTrueCondition()?)?;
        let mut source = None;
        for index in 0..elements.Length()? {
            let element = elements.GetElement(index)?;
            if element.CurrentName()? == "Drag text" {
                assert!(source.is_none(), "more than one native drag source");
                source = Some(element);
            }
        }
        source.ok_or_else(|| "native drag source is missing".into())
    }
}
