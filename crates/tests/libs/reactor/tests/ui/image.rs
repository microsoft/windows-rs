use super::*;

#[test]
fn image_decode_failure_and_recovery() -> Result<()> {
    process::run("image", |ui| {
        ui.name("events", "Opened 1; failed 0")?;
        ui.invoke("invalid")?;
        ui.name("events", "Opened 1; failed 1")?;
        ui.invoke("valid")?;
        ui.name("events", "Opened 2; failed 1")?;
        ui.name_stays("events", "Opened 2; failed 1", Duration::from_millis(250))
    })
}

#[test]
fn svg_images_load_on_live_and_mounting_elements() -> Result<()> {
    process::run("image", |ui| {
        ui.name("events", "Opened 1; failed 0")?;
        ui.invoke("svg")?;
        ui.name("events", "Opened 3; failed 0")?;
        ui.name_stays("events", "Opened 3; failed 0", Duration::from_millis(250))
    })
}
