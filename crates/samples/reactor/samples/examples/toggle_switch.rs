#![windows_subsystem = "windows"]

use windows_reactor::*;

struct ToggleSwitchSample {
    on: bool,
}

impl Component for ToggleSwitchSample {
    type Message = bool;
    type Input = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self { on: true }
    }

    fn update(&mut self, on: bool, _context: &ComponentContext<Self>) {
        self.on = on;
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title("ToggleSwitch");
        StackPanel::new().spacing(8.0).children((
            ToggleSwitch::new()
                .is_on(self.on)
                .on_toggled(context.callback(|on| on))
                .slots([
                    SlotView::new(ToggleSwitchSlot::Header, "Notifications"),
                    SlotView::new(ToggleSwitchSlot::OnContent, "On"),
                    SlotView::new(ToggleSwitchSlot::OffContent, "Off"),
                ]),
            if self.on {
                "Notifications enabled"
            } else {
                "Notifications muted"
            },
            ToggleSwitch::new()
                .is_on(true)
                .is_enabled(false)
                .slots([SlotView::new(
                    ToggleSwitchSlot::Header,
                    "Disabled (always on)",
                )]),
        ))
    }
}

fn main() {
    App::run_component::<ToggleSwitchSample>(()).unwrap();
}
