use super::*;

#[test]
fn scroll_view_reports_view_changes_and_scroll_completions() -> Result<()> {
    const SCROLL_PATTERN: i32 = 10004;
    const NO_SCROLL: f64 = -1.0;
    process::run("scroll", |ui| {
        ui.name("metrics", "Offset 0; viewport 200; extent 2000")?;

        ui.invoke("jump")?;
        ui.name("completions", "Completed jump 500")?;
        ui.name("metrics", "Offset 500; viewport 200; extent 2000")?;

        // A request for the current offset still completes.
        ui.invoke("jump")?;
        ui.name("completions", "Completed jump 500, jump 500")?;

        // Requests are clamped to the scrollable range.
        ui.invoke("end")?;
        ui.name("completions", "Completed jump 500, jump 500, end 1800")?;
        ui.name("metrics", "Offset 1800; viewport 200; extent 2000")?;

        // Scrolling that Reactor did not request is still observed. ScrollView has no automation
        // peer of its own; its template's ScrollPresenter provides the scroll pattern.
        unsafe {
            ui.element("PART_ScrollPresenter")?
                .GetCurrentPatternAs::<IUIAutomationScrollPattern>(SCROLL_PATTERN)?
                .SetScrollPercent(NO_SCROLL, 0.0)
                .ok()?;
        }
        ui.name("metrics", "Offset 0; viewport 200; extent 2000")?;

        // An interrupted request completes with the view where the interruption left it.
        ui.invoke("interrupt")?;
        ui.name(
            "completions",
            "Completed jump 500, jump 500, end 1800, animated 300, interrupt 300",
        )?;
        ui.name("metrics", "Offset 300; viewport 200; extent 2000")?;

        // WinUI takes no action without content, so the requests cannot complete normally.
        ui.invoke("show-empty")?;
        ui.invoke("empty")?;
        ui.name(
            "completions",
            "Completed jump 500, jump 500, end 1800, animated 300, interrupt 300, empty 1 Unavailable, empty 2 Unavailable",
        )?;

        // Removing the view before the request reaches it reports that it is unavailable.
        ui.invoke("remove")?;
        ui.name(
            "completions",
            "Completed jump 500, jump 500, end 1800, animated 300, interrupt 300, empty 1 Unavailable, empty 2 Unavailable, removed Unavailable",
        )?;
        Ok(())
    })
}
