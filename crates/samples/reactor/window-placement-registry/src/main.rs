use std::io::{Error, ErrorKind};
use std::rc::Rc;
use windows_reactor::*;
use windows_registry::{CURRENT_USER, Key};

const SETTINGS_KEY: &str = r"Software\windows-rs\samples\window-placement";

#[derive(Clone)]
struct Settings {
    key: Rc<Key>,
    saved: Option<WindowPlacement>,
}

impl PartialEq for Settings {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.key, &other.key) && self.saved == other.saved
    }
}

struct PlacementSample {
    key: Rc<Key>,
    status: String,
}

impl Component for PlacementSample {
    type Input = Settings;
    type Message = WindowPlacement;

    fn create(settings: &Settings, _: &ComponentContext<Self>) -> Self {
        Self {
            key: Rc::clone(&settings.key),
            status: "Waiting for placement".to_string(),
        }
    }

    fn update(&mut self, placement: WindowPlacement, _: &ComponentContext<Self>) {
        self.status = match self.key.set_string("Placement", encode(placement)) {
            Ok(()) => format!(
                "Saved restored bounds: ({}, {}) {} x {} physical pixels\nMaximized: {}",
                placement.x, placement.y, placement.width, placement.height, placement.maximized,
            ),
            Err(error) => format!("Could not save placement: {error}"),
        };
    }

    fn view(&self, settings: &Settings, context: &mut ViewContext<Self>) -> View {
        let mut visuals = WindowVisuals::new().client_size(640.0, 360.0);
        if let Some(saved) = settings.saved {
            visuals = visuals.initial_placement(saved);
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

fn encode(placement: WindowPlacement) -> String {
    format!(
        "{} {} {} {} {}",
        placement.x,
        placement.y,
        placement.width,
        placement.height,
        i32::from(placement.maximized),
    )
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
    App::run_component::<PlacementSample>(Settings {
        key: Rc::new(key),
        saved,
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        for maximized in [false, true] {
            let placement = WindowPlacement {
                x: -1200,
                y: -200,
                width: 800,
                height: 600,
                maximized,
            };
            assert_eq!(decode(&encode(placement)).unwrap(), placement);
        }
    }

    #[test]
    fn reject_invalid_saved_values() {
        for value in [
            "",
            "0 0 800 600",
            "0 0 800 600 0 extra",
            "x 0 800 600 0",
            "0 0 0 600 0",
            "0 0 800 -1 0",
            "0 0 800 600 2",
            "2147483647 0 800 600 0",
            "0 2147483647 800 600 0",
        ] {
            assert_eq!(decode(value).unwrap_err().kind(), ErrorKind::InvalidData);
        }
    }
}
