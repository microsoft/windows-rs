use super::*;

fn open(ui: &Automation, label: &str) -> Result<IUIAutomationElement> {
    ui.invoke("open")?;
    let item = ui.menu_item(label)?;
    ui.menu_item("More")?;
    Ok(item)
}

fn closed(ui: &Automation, label: &str) -> Result<()> {
    ui.menu_gone(label)?;
    ui.menu_gone("More")
}

fn invoke(item: &IUIAutomationElement) -> Result<()> {
    unsafe {
        item.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
            .Invoke()
            .ok()?;
    }
    Ok(())
}

fn same_item(ui: &Automation, item: &IUIAutomationElement, label: &str) -> Result<()> {
    unsafe {
        assert!(
            ui.client
                .CompareElements(item, &ui.menu_item(label)?)?
                .as_bool()
        );
    }
    Ok(())
}

#[test]
fn menu_callback_replacement_and_removal() -> Result<()> {
    process::run("menu", callback_replacement_and_removal)
}

#[test]
fn dropdown_callback_replacement_and_removal() -> Result<()> {
    process::run("menu-dropdown", callback_replacement_and_removal)
}

#[test]
fn menu_bar_callback_replacement_and_removal() -> Result<()> {
    process::run("menu-bar", callback_replacement_and_removal)
}

fn callback_replacement_and_removal(ui: &Automation) -> Result<()> {
    ui.focus("replace")?;
    ui.name("state", "Generation 0; attached true")?;
    let original = open(ui, "Command")?;
    ui.invoke("replace")?;
    ui.name("state", "Generation 1; attached true")?;
    same_item(ui, &original, "Command")?;
    invoke(&original)?;
    ui.name("selection", "Selected 1; calls 1")?;
    closed(ui, "Command")?;
    let stale = unsafe {
        open(ui, "Command")?.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
    };
    ui.invoke("remove")?;
    ui.name("state", "Generation 1; attached false")?;
    closed(ui, "Command")?;
    retired(ui, &stale, "Selected 1; calls 1")
}

#[test]
fn menu_content_replacement_retires_open_items() -> Result<()> {
    process::run("menu", content_replacement_retires_open_items)
}

#[test]
fn dropdown_content_replacement_retires_open_items() -> Result<()> {
    process::run("menu-dropdown", content_replacement_retires_open_items)
}

#[test]
fn menu_bar_content_replacement_retires_open_items() -> Result<()> {
    process::run("menu-bar", content_replacement_retires_open_items)
}

fn content_replacement_retires_open_items(ui: &Automation) -> Result<()> {
    ui.focus("replace")?;
    let owner = ui.element("open")?;
    let original = open(ui, "Command")?;
    let stale =
        unsafe { original.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)? };
    ui.invoke("change")?;
    ui.name("content", "Changed")?;
    closed(ui, "Command")?;
    retired(ui, &stale, "Selected 0; calls 0")?;
    let current = open(ui, "Changed command")?;
    unsafe {
        assert!(
            ui.client
                .CompareElements(&owner, &ui.element("open")?)?
                .as_bool()
        );
        assert!(!ui.client.CompareElements(&original, &current)?.as_bool());
    }
    retired(ui, &stale, "Selected 0; calls 0")?;
    same_item(ui, &current, "Changed command")?;
    invoke(&current)?;
    ui.name("selection", "Selected 0; calls 1")?;
    closed(ui, "Changed command")?;
    ui.name_stays(
        "selection",
        "Selected 0; calls 1",
        Duration::from_millis(250),
    )
}

#[test]
fn menu_reattachment_does_not_revive_retired_items() -> Result<()> {
    process::run("menu", reattachment_does_not_revive_retired_items)
}

#[test]
fn dropdown_reattachment_does_not_revive_retired_items() -> Result<()> {
    process::run("menu-dropdown", reattachment_does_not_revive_retired_items)
}

#[test]
fn menu_bar_reattachment_does_not_revive_retired_items() -> Result<()> {
    process::run("menu-bar", reattachment_does_not_revive_retired_items)
}

fn reattachment_does_not_revive_retired_items(ui: &Automation) -> Result<()> {
    ui.focus("replace")?;
    let owner = ui.element("open")?;
    let original = open(ui, "Command")?;
    let stale =
        unsafe { original.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)? };
    ui.invoke("remove")?;
    ui.name("state", "Generation 0; attached false")?;
    closed(ui, "Command")?;
    retired(ui, &stale, "Selected 0; calls 0")?;
    ui.invoke("attach")?;
    ui.name("state", "Generation 0; attached true")?;
    let current = open(ui, "Command")?;
    unsafe {
        assert!(
            ui.client
                .CompareElements(&owner, &ui.element("open")?)?
                .as_bool()
        );
        assert!(!ui.client.CompareElements(&original, &current)?.as_bool());
    }
    retired(ui, &stale, "Selected 0; calls 0")?;
    same_item(ui, &current, "Command")?;
    invoke(&current)?;
    ui.name("selection", "Selected 0; calls 1")?;
    closed(ui, "Command")?;
    ui.name_stays(
        "selection",
        "Selected 0; calls 1",
        Duration::from_millis(250),
    )
}

#[test]
fn menu_owner_replacement_retires_open_items() -> Result<()> {
    process::run("menu", owner_replacement_retires_open_items)
}

#[test]
fn dropdown_owner_replacement_retires_open_items() -> Result<()> {
    process::run("menu-dropdown", owner_replacement_retires_open_items)
}

#[test]
fn menu_bar_owner_replacement_retires_open_items() -> Result<()> {
    process::run("menu-bar", owner_replacement_retires_open_items)
}

fn owner_replacement_retires_open_items(ui: &Automation) -> Result<()> {
    ui.focus("replace")?;
    let owner = ui.element("open")?;
    let original = open(ui, "Command")?;
    let stale =
        unsafe { original.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)? };
    ui.invoke("replace-owner")?;
    ui.name("owner", "Owner 1")?;
    ui.name("state", "Generation 1; attached true")?;
    closed(ui, "Command")?;
    unsafe {
        assert!(
            !ui.client
                .CompareElements(&owner, &ui.element("open")?)?
                .as_bool()
        );
    }

    retired(ui, &stale, "Selected 0; calls 0")?;
    let current = open(ui, "Command")?;
    unsafe {
        assert!(!ui.client.CompareElements(&original, &current)?.as_bool());
    }
    ui.invoke("replace")?;
    ui.name("state", "Generation 2; attached true")?;
    same_item(ui, &current, "Command")?;
    retired(ui, &stale, "Selected 0; calls 0")?;
    same_item(ui, &current, "Command")?;
    invoke(&current)?;
    ui.name("selection", "Selected 2; calls 1")?;
    closed(ui, "Command")?;
    ui.name_stays(
        "selection",
        "Selected 2; calls 1",
        Duration::from_millis(250),
    )
}

#[test]
fn command_callback_replacement_and_removal() -> Result<()> {
    process::run("command-bar", |ui| {
        ui.focus("replace")?;
        ui.invoke("open")?;
        let original = ui.menu_item("Command")?;
        let stale =
            unsafe { original.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)? };
        ui.invoke("replace")?;
        ui.name("state", "Generation 1; attached true")?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&original, &ui.menu_item("Command")?)?
                    .as_bool()
            );
        }
        invoke(&original)?;
        ui.name("selection", "Selected 1; calls 1")?;
        ui.invoke("remove")?;
        ui.name("state", "Generation 1; attached false")?;
        ui.menu_gone("Command")?;
        retired(ui, &stale, "Selected 1; calls 1")
    })
}

#[test]
fn command_content_replacement_retires_open_items() -> Result<()> {
    process::run("command-bar", |ui| {
        ui.focus("replace")?;
        let owner = ui.element("open")?;
        ui.invoke("open")?;
        let original = ui.menu_item("Command")?;
        let stale =
            unsafe { original.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)? };
        ui.invoke("change")?;
        ui.name("content", "Changed")?;
        ui.menu_gone("Command")?;
        retired(ui, &stale, "Selected 0; calls 0")?;
        ui.invoke("open")?;
        let current = ui.menu_item("Changed command")?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&owner, &ui.element("open")?)?
                    .as_bool()
            );
            assert!(!ui.client.CompareElements(&original, &current)?.as_bool());
        }
        retired(ui, &stale, "Selected 0; calls 0")?;
        unsafe {
            assert!(
                ui.client
                    .CompareElements(&current, &ui.menu_item("Changed command")?)?
                    .as_bool()
            );
        }
        invoke(&current)?;
        ui.name("selection", "Selected 0; calls 1")?;
        ui.name_stays(
            "selection",
            "Selected 0; calls 1",
            Duration::from_millis(250),
        )
    })
}

#[test]
fn tray_callback_replacement_and_icon_removal() -> Result<()> {
    process::run("tray", |ui| {
        ui.focus("replace")?;
        ui.open_tray_menu()?;
        let original = ui.menu_item("Command")?;
        ui.menu_item("More")?;
        ui.activate_tray_icon()?;
        ui.name("state", "Generation 1; attached true")?;
        same_item(ui, &original, "Command")?;
        invoke(&original)?;
        ui.name("selection", "Selected 1; calls 1")?;
        closed(ui, "Command")?;
        ui.invoke("remove")?;
        ui.name("activation", "Remove icon")?;
        ui.open_tray_menu()?;
        let stale = unsafe {
            ui.menu_item("Command")?
                .GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)?
        };
        ui.activate_tray_icon()?;
        ui.name("state", "Generation 1; attached false")?;
        ui.tray_gone()?;
        closed(ui, "Command")?;
        retired(ui, &stale, "Selected 1; calls 1")
    })
}

#[test]
fn tray_icon_replacement_retires_open_items() -> Result<()> {
    process::run("tray", |ui| {
        ui.focus("replace")?;
        ui.invoke("replace-owner")?;
        ui.name("activation", "Replace icon")?;
        ui.open_tray_menu()?;
        let original = ui.menu_item("Command")?;
        let stale =
            unsafe { original.GetCurrentPatternAs::<IUIAutomationInvokePattern>(INVOKE_PATTERN)? };
        ui.activate_tray_icon()?;
        ui.name("owner", "Owner 1")?;
        ui.name("state", "Generation 1; attached true")?;
        closed(ui, "Command")?;
        retired(ui, &stale, "Selected 0; calls 0")?;
        ui.open_tray_menu()?;
        let current = ui.menu_item("Command")?;
        unsafe {
            assert!(!ui.client.CompareElements(&original, &current)?.as_bool());
        }
        retired(ui, &stale, "Selected 0; calls 0")?;
        same_item(ui, &current, "Command")?;
        invoke(&current)?;
        ui.name("selection", "Selected 1; calls 1")?;
        closed(ui, "Command")?;
        ui.name_stays(
            "selection",
            "Selected 1; calls 1",
            Duration::from_millis(250),
        )
    })
}
