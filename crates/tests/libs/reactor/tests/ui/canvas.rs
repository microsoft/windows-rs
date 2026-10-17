use super::*;

#[test]
fn canvas_draws_resizes_and_releases_callbacks_on_removal() -> Result<()> {
    process::run("canvas", |ui| {
        ui.name("frame", "Canvas 0: 240 x 160; changed true")?;
        ui.name("dropped", "Dropped []")?;
        ui.invoke("resize")?;
        ui.name("frame", "Canvas 0: 320 x 200; changed true")?;
        ui.name("dropped", "Dropped []")?;

        ui.invoke("remove")?;
        ui.name("dropped", "Dropped [0]")?;
        ui.invoke("attach")?;
        ui.name("frame", "Canvas 1: 240 x 160; changed true")?;
        ui.name("dropped", "Dropped [0]")?;
        ui.invoke("remove")?;
        ui.name("dropped", "Dropped [0, 1]")
    })
}
