use windows_reactor::*;
use windows_registry::CURRENT_USER;

const SETTINGS_KEY: &str = r"Software\windows-rs\samples\window-placement";

struct PlacementSample;

impl Component for PlacementSample {
    type Input = Option<WindowPlacement>;
    type Message = WindowPlacement;

    fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self
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
        if let Err(error) = CURRENT_USER
            .create(SETTINGS_KEY)
            .and_then(|key| key.set_string("Placement", &value))
        {
            eprintln!("Could not save placement: {error}");
        }
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
            )),
        )
    }
}

fn load() -> Result<WindowPlacement, Box<dyn std::error::Error>> {
    let value = CURRENT_USER.open(SETTINGS_KEY)?.get_string("Placement")?;
    let fields = value
        .split_whitespace()
        .map(str::parse::<i32>)
        .collect::<Result<Vec<_>, _>>()?;
    match fields.as_slice() {
        &[x, y, width, height, maximized @ (0 | 1)]
            if width > 0
                && height > 0
                && x.checked_add(width).is_some()
                && y.checked_add(height).is_some() =>
        {
            Ok(WindowPlacement {
                x,
                y,
                width,
                height,
                maximized: maximized == 1,
            })
        }
        _ => Err("invalid saved window placement".into()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let saved = load()
        .inspect_err(|error| eprintln!("Using default window placement: {error}"))
        .ok();
    App::run_component::<PlacementSample>(saved)?;
    Ok(())
}
