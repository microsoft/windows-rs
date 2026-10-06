use std::io::{Error, ErrorKind};
use windows_reactor::*;
use windows_registry::CURRENT_USER;

const SETTINGS_KEY: &str = r"Software\windows-rs\samples\window-placement";

struct PlacementSample {
    status: String,
}

impl Component for PlacementSample {
    type Input = Option<WindowPlacement>;
    type Message = WindowPlacement;

    fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self {
            status: "Waiting for placement".to_string(),
        }
    }

    fn update(&mut self, placement: WindowPlacement, _: &ComponentContext<Self>) {
        let value = format!(
            "{} {} {} {} {}",
            placement.x,
            placement.y,
            placement.width,
            placement.height,
            i32::from(placement.maximized),
        );
        self.status = match CURRENT_USER
            .create(SETTINGS_KEY)
            .and_then(|key| key.set_string("Placement", &value))
        {
            Ok(()) => format!("Saved (x y width height maximized): {value}"),
            Err(error) => format!("Could not save placement: {error}"),
        };
    }

    fn view(&self, saved: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let mut visuals = WindowVisuals::new().client_size(640.0, 360.0);
        if let Some(saved) = saved {
            visuals = visuals.initial_placement(*saved);
        }
        context.window_visuals(visuals);
        context.on_window_placement(context.callback(|placement| placement));
        context.window_frame(
            "Persistent window placement",
            StackPanel::new().margin(24.0).spacing(12.0).children((
                "Move, resize, or maximize this window.",
                "Close it, then run this sample again to restore its placement.",
                TextBlock::new().text(self.status.clone()),
                TextBlock::new().text(format!(r"HKCU\{SETTINGS_KEY}")),
            )),
        )
    }
}

fn decode(value: &str) -> std::io::Result<WindowPlacement> {
    let invalid = || Error::new(ErrorKind::InvalidData, "invalid saved window placement");
    let fields = value
        .split_whitespace()
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| invalid())?;
    let [x, y, width, height, maximized] = fields.as_slice() else {
        return Err(invalid());
    };
    if *width <= 0
        || *height <= 0
        || x.checked_add(*width).is_none()
        || y.checked_add(*height).is_none()
        || !matches!(maximized, 0 | 1)
    {
        return Err(invalid());
    }
    Ok(WindowPlacement {
        x: *x,
        y: *y,
        width: *width,
        height: *height,
        maximized: *maximized == 1,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = CURRENT_USER.create(SETTINGS_KEY)?;
    let saved = match key.get_string("Placement") {
        Ok(value) => Some(decode(&value)?),
        // A missing value is the first-run case, not a registry access failure.
        Err(error) if error.code().0 == 0x8007_0002_u32 as i32 => None,
        Err(error) => return Err(error.into()),
    };
    App::run_component::<PlacementSample>(saved)?;
    Ok(())
}
