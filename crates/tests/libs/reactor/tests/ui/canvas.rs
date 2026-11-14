use super::*;

#[test]
fn canvas_input_follows_submission_not_pending_drawing() -> Result<()> {
    process::run("canvas-submitted-input", |ui| {
        ui.name("submitted", "Submitted Some(0)")?;
        ui.invoke("stage")?;
        ui.name("requested", "Requested 1")?;
        let click = |expected| -> Result<()> {
            ui.pointer_move("canvas-input", 0.5, 0.25)?;
            ui.pointer_clicks(1)?;
            ui.name("pressed", expected)
        };
        click("Pressed 1 Some((0, \"red\"))")?;
        ui.invoke("fail-native")?;
        ui.name("errors", "Errors 1")?;
        ui.name("submitted", "Submitted Some(0)")?;
        click("Pressed 2 Some((0, \"red\"))")?;
        ui.invoke("fail-callback")?;
        ui.name("errors", "Errors 2")?;
        ui.name("submitted", "Submitted Some(0)")?;
        ui.invoke("submit")?;
        ui.name("submitted", "Submitted Some(1)")?;
        click("Pressed 3 Some((1, \"blue\"))")?;
        ui.invoke("stage")?;
        ui.name("requested", "Requested 2")?;
        ui.invoke("lose-device")?;
        ui.name("submitted", "Submitted Some(2)")?;
        click("Pressed 4 Some((2, \"red\"))")?;
        ui.invoke("again")?;
        ui.name("submitted", "Submitted Some(3)")?;
        click("Pressed 5 Some((3, \"blue\"))")?;
        ui.invoke("remove")?;
        ui.name("released", "Released true")
    })
}

#[test]
fn canvas_draws_resizes_and_releases_callbacks_on_removal() -> Result<()> {
    process::run("canvas", |ui| {
        ui.name("frame", "Canvas 0: 240 x 160; changed true; device 1")?;
        ui.name("dropped", "Dropped []")?;
        ui.invoke("resize")?;
        ui.name("frame", "Canvas 0: 320 x 200; changed true; device 1")?;
        ui.name("dropped", "Dropped []")?;

        ui.invoke("remove")?;
        ui.name("dropped", "Dropped [0]")?;
        ui.invoke("attach")?;
        ui.name("frame", "Canvas 1: 240 x 160; changed true; device 2")?;
        ui.name("dropped", "Dropped [0]")?;
        ui.invoke("remove")?;
        ui.name("dropped", "Dropped [0, 1]")
    })
}

#[test]
fn demand_canvas_recovers_and_releases_native_devices_on_removal() -> Result<()> {
    process::run("canvas-demand", |ui| {
        ui.name("frame", "Canvas 0: 240 x 160; changed true; device 1")?;
        ui.invoke("fail-draw")?;
        ui.name("draw-errors", "Draw errors 1")?;
        devices(ui, 1, 0)?;
        ui.invoke("invalidate")?;
        ui.name("repaint", "Revision 1; draws 1")?;
        ui.name_stays("repaint", "Revision 1; draws 1", Duration::from_millis(250))?;
        ui.invoke("invalidate-again")?;
        ui.name("repaint", "Revision 2; draws 2")?;
        ui.name_stays("repaint", "Revision 2; draws 2", Duration::from_millis(250))?;
        ui.invoke("lose-device")?;
        ui.name("frame", "Canvas 0: 240 x 160; changed true; device 2")?;
        devices(ui, 2, 1)?;
        for generation in 0..16 {
            ui.invoke("resize")?;
            ui.name(
                "frame",
                &format!(
                    "Canvas {generation}: 320 x 200; changed true; device {}",
                    generation + 2,
                ),
            )?;
            ui.invoke("remove")?;
            devices(ui, generation + 2, generation + 2)?;
            ui.name(
                "dropped",
                &format!("Dropped {:?}", (0..=generation).collect::<Vec<_>>()),
            )?;
            if generation < 15 {
                ui.invoke("attach")?;
                ui.name(
                    "frame",
                    &format!(
                        "Canvas {}: 240 x 160; changed true; device {}",
                        generation + 1,
                        generation + 3,
                    ),
                )?;
            }
        }
        Ok(())
    })
}

fn devices(ui: &Automation, created: u32, released: u32) -> Result<()> {
    ui.name(
        "devices",
        &format!("Devices created {created}; released {released}"),
    )
}

#[test]
fn canvas_can_be_a_window_view() -> Result<()> {
    process::run_to_exit("canvas-root")
}

#[test]
fn canvas_image_source_draws_and_attaches() -> Result<()> {
    process::run("canvas-image", |ui| {
        ui.name("status", "Drawn Some(true); attached Some(true)")
    })
}
