use windows_reactor::*;

struct MenuFlyoutSample {
    last_action: String,
}

impl Component for MenuFlyoutSample {
    type Message = String;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            last_action: "(none)".to_string(),
        }
    }

    fn update(&mut self, message: String, _context: &ComponentContext<Self>) {
        self.last_action = message;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("MenuFlyout");
        StackPanel::new().spacing(8.0).children((
            Button::new().content("Open Menu").menu(Menu::new(
                [
                    MenuItem::item("cut", "Cut"),
                    MenuItem::item("copy", "Copy"),
                    MenuItem::item("paste", "Paste"),
                    MenuItem::separator("separator"),
                    MenuItem::submenu(
                        "font-size",
                        "Font Size",
                        [
                            MenuItem::item("small", "Small"),
                            MenuItem::item("medium", "Medium"),
                            MenuItem::item("large", "Large"),
                        ],
                    ),
                ],
                context.forward(),
            )),
            format!("Last action: {}", self.last_action),
        ))
    }
}

fn main() {
    App::run_component::<MenuFlyoutSample>(()).unwrap();
}
