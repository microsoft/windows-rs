use windows_reactor::*;

struct CommandBarSample {
    last_click: String,
}

impl Component for CommandBarSample {
    type Message = String;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            last_click: "(none)".to_string(),
        }
    }

    fn update(&mut self, message: String, _context: &ComponentContext<Self>) {
        self.last_click = message;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("CommandBar");
        StackPanel::new().spacing(12.0).children((
            CommandBar::new().owned_commands(
                [
                    CommandBarCommand::button_with_icon("add", "Add", Symbol::Add),
                    CommandBarCommand::button_with_icon("edit", "Edit", Symbol::Edit),
                    CommandBarCommand::separator("separator"),
                    CommandBarCommand::button_with_icon("save", "Save", Symbol::Save),
                    CommandBarCommand::button_with_icon("delete", "Delete", Symbol::Delete),
                ],
                [
                    CommandBarCommand::button("select-all", "Select All"),
                    CommandBarCommand::button("share", "Share"),
                ],
                context.callback(std::convert::identity),
            ),
            TextBlock::new().text(format!("Last clicked: {}", self.last_click)),
        ))
    }
}

fn main() {
    App::run_component::<CommandBarSample>(()).unwrap();
}
