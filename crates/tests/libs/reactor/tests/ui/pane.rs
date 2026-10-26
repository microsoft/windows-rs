use super::*;

const QUIET: Duration = Duration::from_millis(500);
const ESCAPE: i32 = 0x1B;

#[test]
fn split_view_pane_is_controlled() -> Result<()> {
    process::run("pane", |ui| {
        ui.name("state", "Split false; navigation true")?;
        ui.gone("split-pane")?;

        ui.invoke("open-split")?;
        ui.element("split-pane")?;
        ui.invoke("close-split")?;
        ui.gone("split-pane")?;
        ui.name_stays("events", "Events []", QUIET)?;

        // A reopen during the close animation is Reactor's own write, not a user close.
        ui.invoke("open-split")?;
        ui.element("split-pane")?;
        ui.invoke("close-split")?;
        ui.invoke("open-split")?;
        ui.name_stays("events", "Events []", QUIET)?;
        ui.name("state", "Split true; navigation true")?;
        ui.element("split-pane")?;
        ui.invoke("close-split")?;
        ui.gone("split-pane")?;

        ui.invoke("open-split")?;
        ui.press_keys("split-pane", &[ESCAPE])?;
        ui.name("events", "Events [Split false]")?;
        ui.name("state", "Split false; navigation true")?;
        ui.gone("split-pane")?;

        ui.invoke("open-split")?;
        ui.invoke("reject")?;
        ui.press_keys("split-pane", &[ESCAPE])?;
        ui.name("events", "Events [Split false, Split false]")?;
        ui.name("state", "Split true; navigation true")?;
        ui.element("split-pane")?;
        ui.name_stays("events", "Events [Split false, Split false]", QUIET)
    })
}

#[test]
fn navigation_view_pane_is_controlled() -> Result<()> {
    process::run("pane", |ui| {
        ui.name("state", "Split false; navigation true")?;
        ui.name_stays("events", "Events []", QUIET)?;
        pane_open(ui, true)?;

        ui.invoke("close-navigation")?;
        pane_open(ui, false)?;
        ui.invoke("open-navigation")?;
        pane_open(ui, true)?;
        ui.name_stays("events", "Events []", QUIET)?;

        ui.invoke("TogglePaneButton")?;
        ui.name("events", "Events [Navigation false]")?;
        ui.name("state", "Split false; navigation false")?;
        pane_open(ui, false)?;
        ui.invoke("TogglePaneButton")?;
        ui.name("events", "Events [Navigation false, Navigation true]")?;
        pane_open(ui, true)?;

        // WinUI leaves the pane visually compact when it reopens during the close animation, so
        // only the reported event and the restored state are checked.
        ui.invoke("reject")?;
        ui.invoke("TogglePaneButton")?;
        let events = "Events [Navigation false, Navigation true, Navigation false]";
        ui.name("events", events)?;
        ui.name("state", "Split false; navigation true")?;
        ui.name_stays("events", events, QUIET)
    })
}

/// An open left pane shows menu items at full width; a closed one shows them compact.
fn pane_open(ui: &Automation, open: bool) -> Result<()> {
    let item = ui.element("navigation-item")?;
    ui.wait(
        &format!("navigation pane open {open}"),
        || unsafe { item.CurrentBoundingRectangle() },
        |bounds| (bounds.right - bounds.left > 150) == open,
    )?;
    Ok(())
}
