use super::*;

const QUIET: Duration = Duration::from_millis(500);
const SETTLE: Duration = Duration::from_millis(250);

#[test]
fn info_bar_open_state_is_controlled() -> Result<()> {
    process::run("surface", |ui| {
        ui.name("state", "Info false; tip false")?;
        ui.gone("CloseButton")?;

        ui.invoke("show-info")?;
        ui.element("CloseButton")?;
        ui.invoke("hide-info")?;
        ui.gone("CloseButton")?;
        ui.name_stays("events", "Events []", QUIET)?;

        ui.invoke("show-info")?;
        ui.invoke("CloseButton")?;
        ui.name("events", "Events [Info closed]")?;
        ui.name("state", "Info false; tip false")?;
        ui.gone("CloseButton")?;
        ui.invoke("show-info")?;
        ui.element("CloseButton")?;

        ui.invoke("reject")?;
        ui.invoke("CloseButton")?;
        ui.name("events", "Events [Info closed, Info closed]")?;
        ui.name("state", "Info true; tip false")?;
        ui.element("CloseButton")?;
        ui.name_stays("events", "Events [Info closed, Info closed]", QUIET)
    })
}

#[test]
fn teaching_tip_open_state_is_controlled() -> Result<()> {
    process::run("surface", |ui| {
        ui.name("state", "Info false; tip false")?;
        ui.gone("ActionButton")?;

        ui.invoke("show-tip")?;
        ui.settled("ActionButton", SETTLE)?;
        ui.invoke("hide-tip")?;
        ui.gone("ActionButton")?;
        ui.name_stays("events", "Events []", QUIET)?;

        ui.invoke("show-tip")?;
        ui.settled("ActionButton", SETTLE)?;
        ui.invoke("ActionButton")?;
        ui.name("events", "Events [Tip action]")?;
        ui.invoke("CloseButton")?;
        ui.name("events", "Events [Tip action, Tip closed]")?;
        ui.name("state", "Info false; tip false")?;
        ui.gone("ActionButton")?;

        ui.invoke("show-tip")?;
        ui.settled("ActionButton", SETTLE)?;
        ui.invoke("reject")?;
        ui.invoke("CloseButton")?;
        ui.name("events", "Events [Tip action, Tip closed, Tip closed]")?;
        ui.name("state", "Info false; tip true")?;
        ui.element("ActionButton")?;
        ui.name_stays(
            "events",
            "Events [Tip action, Tip closed, Tip closed]",
            QUIET,
        )
    })
}

#[test]
fn teaching_tip_programmatic_close_is_not_reported() -> Result<()> {
    process::run("surface", |ui| {
        ui.invoke("show-tip")?;
        ui.settled("ActionButton", SETTLE)?;

        // WinUI ignores the reopen during the close animation, then raises `Closed`.
        ui.invoke("hide-tip")?;
        ui.invoke("show-tip")?;
        ui.gone("ActionButton")?;
        ui.name_stays("events", "Events []", QUIET)?;
        ui.name("state", "Info false; tip true")?;

        // The next render restores the declared open state.
        ui.invoke("show-tip")?;
        ui.element("ActionButton")?;
        ui.name_stays("events", "Events []", QUIET)
    })
}
