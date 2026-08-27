use windows_reactor::*;

struct CommandBarFlyoutSample {
    last_action: String,
}

impl Component for CommandBarFlyoutSample {
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
        context.window_title("CommandBarFlyout");
        StackPanel::new().spacing(8.0).children((
            Button::new()
                .content("Show Commands")
                .command_bar_flyout(CommandBarFlyout::new(
                    [
                        CommandBarCommand::button_with_icon("cut", "Cut", Symbol::Cut),
                        CommandBarCommand::button_with_icon("copy", "Copy", Symbol::Copy),
                        CommandBarCommand::button_with_icon("paste", "Paste", Symbol::Paste),
                    ],
                    [
                        CommandBarCommand::button("select-all", "Select All"),
                        CommandBarCommand::button("print", "Print"),
                    ],
                    context.callback(std::convert::identity),
                )),
            format!("Last action: {}", self.last_action),
        ))
    }
}

fn main() {
    App::run_component::<CommandBarFlyoutSample>(()).unwrap();
}
