use super::*;

#[test]
fn scroll_view_content_orientation_controls_measurement_and_resets() -> Result<()> {
    process::run("scroll-orientation", |ui| {
        ui.name("metrics", "Offset 0,0; viewport 300,200; extent 200,200")?;
        ui.invoke("vertical")?;
        ui.name("metrics", "Offset 0,100; viewport 300,200; extent 300,300")?;
        ui.invoke("horizontal")?;
        ui.name("metrics", "Offset 0,0; viewport 300,200; extent 200,200")?;
        ui.invoke("vertical")?;
        ui.name("metrics", "Offset 0,100; viewport 300,200; extent 300,300")?;
        ui.invoke("neither")?;
        ui.name("metrics", "Offset 0,0; viewport 300,200; extent 200,200")?;
        ui.invoke("default-orientation")?;
        ui.name("metrics", "Offset 0,100; viewport 300,200; extent 300,300")?;
        Ok(())
    })
}

#[test]
fn scroll_view_far_edge_anchoring_follows_content_only_at_the_edge() -> Result<()> {
    process::run("scroll-layout", |ui| {
        ui.name("metrics", "Offset 0,0; viewport 200,200; extent 400,400")?;
        ui.invoke("end")?;
        ui.name("completed", "Completed 1: 200,200")?;
        ui.invoke("grow")?;
        ui.name(
            "metrics",
            "Offset 280,280; viewport 200,200; extent 480,480",
        )?;
        ui.invoke("shrink")?;
        ui.name(
            "metrics",
            "Offset 200,200; viewport 200,200; extent 400,400",
        )?;

        ui.invoke("middle")?;
        ui.name("completed", "Completed 2: 100,100")?;
        ui.invoke("grow")?;
        ui.name(
            "metrics",
            "Offset 100,100; viewport 200,200; extent 480,480",
        )?;

        ui.invoke("end")?;
        ui.name("completed", "Completed 3: 280,280")?;
        ui.invoke("disabled")?;
        ui.name("anchor", "Some(NaN)")?;
        ui.invoke("grow")?;
        ui.name(
            "metrics",
            "Offset 280,280; viewport 200,200; extent 560,560",
        )?;

        ui.invoke("far")?;
        ui.name("anchor", "Some(1.0)")?;
        ui.invoke("end")?;
        ui.name("completed", "Completed 4: 360,360")?;
        ui.invoke("default-anchor")?;
        ui.name("anchor", "None")?;
        ui.invoke("grow")?;
        ui.name(
            "metrics",
            "Offset 360,360; viewport 200,200; extent 640,640",
        )?;
        Ok(())
    })
}

#[test]
fn scroll_view_observes_viewport_and_extent_changes() -> Result<()> {
    process::run("scroll", |ui| {
        ui.name("metrics", "Offset 0; viewport 200; extent 2000")?;
        ui.invoke("resize")?;
        ui.name("metrics", "Offset 0; viewport 240; extent 2000")?;
        ui.invoke("grow")?;
        ui.name("metrics", "Offset 0; viewport 240; extent 2400")?;
        Ok(())
    })
}

#[test]
fn removing_scroll_view_cancels_active_request() -> Result<()> {
    let mut animations = 0i32;
    unsafe {
        SystemParametersInfoW(
            SPI_GETCLIENTAREAANIMATION as u32,
            0,
            std::ptr::from_mut(&mut animations).cast(),
            Default::default(),
        )
        .ok()?;
    }
    assert_ne!(
        animations, 0,
        "active-scroll coverage requires Windows animations"
    );
    process::run("scroll", |ui| {
        ui.name("metrics", "Offset 0; viewport 200; extent 2000")?;
        ui.invoke("remove-active")?;
        ui.name("active-started", "true")?;
        ui.name("completions", "Completed active Unavailable")?;

        ui.invoke("attach")?;
        ui.name("active-started", "false")?;
        ui.name("metrics", "Offset 0; viewport 200; extent 2000")?;
        ui.invoke("remove-active")?;
        ui.name("active-started", "true")?;
        ui.name(
            "completions",
            "Completed active Unavailable, active Unavailable",
        )?;
        Ok(())
    })
}

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

#[test]
fn items_repeater_keeps_visible_row_when_items_are_prepended() -> Result<()> {
    process::run("scroll-prepend", |ui| {
        ui.invoke("middle")?;
        ui.name("metrics", "Offset 400; extent 2000; first Row 1010")?;
        let row = ui.element("row-1010")?;
        let top = unsafe { row.CurrentBoundingRectangle()?.top };
        ui.invoke("prepend")?;
        ui.name("metrics", "Offset 600; extent 2200; first Row 1010")?;
        let after = ui.element("row-1010")?;
        unsafe {
            assert!(
                ui.client.CompareElements(&row, &after)?.as_bool(),
                "prepending replaced Row 1010"
            );
            assert_eq!(after.CurrentBoundingRectangle()?.top, top);
        }
        ui.invoke("top")?;
        ui.name("metrics", "Offset 0; extent 2200; first Row 995")?;
        ui.invoke("prepend")?;
        ui.name("metrics", "Offset 0; extent 2400; first Row 990")
    })
}
