use windows_reactor::*;

struct TrayApp;

impl Application for TrayApp {
    type Input = ();
    type Message = ();

    fn create(_: &(), _: &ApplicationContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: (), context: &ApplicationContext<Self>) {
        context.exit();
    }

    fn view(&self, _: &(), context: &ApplicationViewContext<Self>) -> ApplicationView {
        ApplicationView::new().notify_icon(
            "tray",
            NotifyIcon::new(concat!(env!("CARGO_MANIFEST_DIR"), "\\icon.ico"))
                .tooltip("Right-click to exit")
                .menu(Menu::new(
                    [MenuItem::item("exit", "Exit")],
                    context.callback(|_| ()),
                )),
        )
    }
}

fn main() {
    App::run_application::<TrayApp>(()).unwrap();
}
