use super::*;

fn rendered(ui: &Automation, visible: bool, rounded: bool, resized: bool) -> Result<()> {
    let bounds = ui.element("bounds")?;
    ui.wait(
        "backdrop bounds and native rounded clipping",
        || {
            let (width, height, pixels) = snapshot(&bounds)?;
            let pixel = |x: f64, y: f64| {
                pixels[((f64::from(height) * y) as i32 * width + (f64::from(width) * x) as i32)
                    as usize]
                    & 0xffffff
            };
            Ok([
                pixel(0.25, 0.15),
                pixel(0.015, 0.025),
                pixel(0.75, 0.6),
                pixel(0.95, 0.5),
                pixel(0.5, 0.95),
            ])
        },
        |sample| {
            let red = 0xff0000;
            (sample[0] != red) == visible
                && (sample[1] == red) == (!visible || rounded)
                && (sample[2] != red) == (visible && resized)
                && sample[3] == red
                && sample[4] == red
        },
    )?;
    Ok(())
}

#[test]
fn backdrop_material_corners_bounds_and_removal_follow_native_updates() -> Result<()> {
    process::run("backdrop", |ui| {
        let _physical = PhysicalCoordinates::new()?;
        rendered(ui, true, true, false)?;
        let foreground = ui.element("foreground")?;
        ui.pointer_move("foreground", 0.5, 0.5)?;
        ui.pointer_clicks(1)?;
        ui.name("clicks", "1")?;

        for material in ["mica", "mica-alt", "acrylic"] {
            ui.invoke(material)?;
            rendered(ui, true, true, false)?;
            ui.invoke("none")?;
            rendered(ui, false, true, false)?;
            ui.invoke(material)?;
            rendered(ui, true, true, false)?;
        }
        ui.invoke("square")?;
        rendered(ui, true, false, false)?;
        ui.invoke("round")?;
        rendered(ui, true, true, false)?;
        ui.invoke("clear-corners")?;
        rendered(ui, true, false, false)?;
        ui.invoke("round")?;
        ui.invoke("resize")?;
        rendered(ui, true, true, true)?;

        for clear in ["none", "omit"] {
            ui.invoke(clear)?;
            rendered(ui, false, true, true)?;
            ui.invoke("acrylic")?;
            rendered(ui, true, true, true)?;
        }
        for _ in 0..3 {
            ui.invoke("attach")?;
            rendered(ui, false, true, true)?;
            ui.invoke("attach")?;
            rendered(ui, true, true, true)?;
        }
        assert!(unsafe {
            ui.client
                .CompareElements(&foreground, &ui.element("foreground")?)?
                .as_bool()
        });
        ui.pointer_move("foreground", 0.5, 0.5)?;
        ui.pointer_clicks(1)?;
        ui.name("clicks", "2")
    })
}
