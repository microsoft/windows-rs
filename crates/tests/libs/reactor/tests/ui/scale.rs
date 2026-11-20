use super::*;

#[derive(Debug)]
struct Pixels {
    width: i32,
    height: i32,
    horizontal: Option<(i32, i32)>,
    vertical: Option<(i32, i32)>,
}

impl Pixels {
    fn matches(&self, x: f64, y: f64) -> bool {
        fn axis(actual: Option<(i32, i32)>, length: i32, scale: f64) -> bool {
            if scale == 0.0 {
                return actual.is_none();
            }
            let Some((first, last)) = actual else {
                return false;
            };
            let first_expected = f64::from(length) * (1.0 - scale) / 2.0;
            let end_expected = f64::from(length) * (1.0 + scale) / 2.0;
            (f64::from(first) - first_expected).abs() <= 2.0
                && (f64::from(last + 1) - end_expected).abs() <= 2.0
        }
        if x == 0.0 || y == 0.0 {
            return self.horizontal.is_none() && self.vertical.is_none();
        }
        axis(self.horizontal, self.width, x) && axis(self.vertical, self.height, y)
    }
}

fn pixels(element: &IUIAutomationElement) -> windows_core::Result<Pixels> {
    let (width, height, buffer) = snapshot(element)?;
    let red = |x, y| {
        let pixel = buffer[(y * width + x) as usize];
        (pixel >> 16) & 0xff > 200 && (pixel >> 8) & 0xff < 60 && pixel & 0xff < 60
    };
    let range = |values: Vec<i32>| values.first().copied().zip(values.last().copied());
    let horizontal = range((0..width).filter(|x| red(*x, height / 2)).collect());
    let vertical = range((0..height).filter(|y| red(width / 2, *y)).collect());
    Ok(Pixels {
        width,
        height,
        horizontal,
        vertical,
    })
}

fn scaled(ui: &Automation, x: f64, y: f64, resized: bool) -> Result<()> {
    let element = ui.element("bounds")?;
    let (width, height) = if resized {
        (1.0, 1.0)
    } else {
        (2.0 / 3.0, 0.75)
    };
    ui.wait(
        &format!("centered scale ({x}, {y})"),
        || pixels(&element),
        |sample| sample.matches(x * width, y * height),
    )?;
    Ok(())
}

#[test]
fn scale_is_centered_on_initial_layout_and_resize() -> Result<()> {
    process::run("scale", |ui| {
        let _physical = PhysicalCoordinates::new()?;
        scaled(ui, 0.5, 0.5, false)?;
        ui.invoke("resize")?;
        scaled(ui, 0.5, 0.5, true)
    })
}

#[test]
fn scale_stays_centered_when_only_layout_changes() -> Result<()> {
    process::run("scale", |ui| {
        let _physical = PhysicalCoordinates::new()?;
        ui.invoke("full")?;
        scaled(ui, 1.0, 1.0, false)?;
        ui.invoke("half")?;
        scaled(ui, 0.5, 0.5, false)?;
        ui.invoke("resize")?;
        scaled(ui, 0.5, 0.5, true)
    })
}

#[test]
fn scale_axes_clear_and_remount_independently() -> Result<()> {
    process::run("scale", |ui| {
        let _physical = PhysicalCoordinates::new()?;
        scaled(ui, 0.5, 0.5, false)?;
        for (action, x, y) in [
            ("vertical", 1.0, 0.5),
            ("horizontal", 0.5, 1.0),
            ("zero", 1.0, 0.0),
            ("zero-x", 0.0, 1.0),
            ("clear", 1.0, 1.0),
        ] {
            ui.invoke(action)?;
            scaled(ui, x, y, false)?;
        }
        ui.invoke("vertical")?;
        scaled(ui, 1.0, 0.5, false)?;
        ui.invoke("attach")?;
        ui.name("attached-state", "false")?;
        scaled(ui, 0.0, 0.0, false)?;
        ui.invoke("resize")?;
        ui.invoke("attach")?;
        ui.name("attached-state", "true")?;
        scaled(ui, 1.0, 0.5, true)
    })
}

fn vertical_transition(ui: &Automation, resized: bool, minimum: f64, maximum: f64) -> Result<()> {
    let element = ui.element("bounds")?;
    let (width, height) = if resized {
        (1.0, 1.0)
    } else {
        (2.0 / 3.0, 0.75)
    };
    ui.wait(
        "centered intermediate vertical scale with unchanged width",
        || pixels(&element),
        |sample| {
            let Some((first, last)) = sample.vertical else {
                return false;
            };
            let fraction = f64::from(last + 1 - first) / f64::from(sample.height);
            fraction > minimum * height
                && fraction < maximum * height
                && sample.matches(width, fraction)
        },
    )?;
    Ok(())
}

#[test]
fn scale_transition_stays_centered_through_clear_resize_and_removal() -> Result<()> {
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
        "scale transition coverage requires Windows animations"
    );
    process::run("scale", |ui| {
        let _physical = PhysicalCoordinates::new()?;
        ui.invoke("zero")?;
        scaled(ui, 1.0, 0.0, false)?;
        ui.invoke("transition")?;
        ui.name("transition-state", "true")?;
        ui.invoke("vertical")?;
        vertical_transition(ui, false, 0.05, 0.45)?;

        ui.invoke("clear")?;
        ui.invoke("resize")?;
        vertical_transition(ui, true, 0.55, 0.95)?;
        scaled(ui, 1.0, 1.0, true)?;

        ui.invoke("vertical")?;
        vertical_transition(ui, true, 0.55, 0.9)?;
        ui.invoke("attach")?;
        ui.name("attached-state", "false")?;
        scaled(ui, 0.0, 0.0, true)?;
        ui.invoke("attach")?;
        ui.name("attached-state", "true")?;
        scaled(ui, 1.0, 0.5, true)?;
        ui.invoke("full")?;
        vertical_transition(ui, true, 0.6, 0.9)?;
        scaled(ui, 1.0, 1.0, true)
    })
}
