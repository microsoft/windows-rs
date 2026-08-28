use windows_reactor::*;

struct WindowIcon;

impl Component for WindowIcon {
    type Message = ();
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Window Icon");
        context.window_visuals(
            WindowVisuals::new()
                .icon(concat!(env!("CARGO_MANIFEST_DIR"), "\\examples\\icon.ico"))
                .client_size(560.0, 240.0),
        );
        TextBlock::new()
            .text("Check the title bar and taskbar - the window uses icon.ico.")
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center)
            .into()
    }
}

fn main() {
    App::run_component::<WindowIcon>(()).unwrap();
}
