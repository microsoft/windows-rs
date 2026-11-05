use crate::controls::*;
use windows_reactor::*;

pub struct CommandBarPage {
    last_command: String,
}

impl Component for CommandBarPage {
    type Message = String;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            last_command: "(none)".to_string(),
        }
    }

    fn update(&mut self, command: String, _context: &ComponentContext<Self>) {
        self.last_command = command;
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "CommandBar",
            "A toolbar for app commands and actions.",
            [KeyedView::new(
                "interactive-command-bar",
                sample_card(
                    "Interactive CommandBar",
                    StackPanel::new().spacing(8.0).children((
                        CommandBar::new().owned_commands(
                            [
                                CommandBarCommand::button_with_icon("add", "Add", Symbol::Add),
                                CommandBarCommand::button_with_icon("edit", "Edit", Symbol::Edit),
                                CommandBarCommand::separator("separator"),
                                CommandBarCommand::button_with_icon(
                                    "delete",
                                    "Delete",
                                    Symbol::Delete,
                                ),
                            ],
                            [
                                CommandBarCommand::button("select-all", "Select All"),
                                CommandBarCommand::button("share", "Share"),
                            ],
                            context.callback(|key: Key| key_label(&key)),
                        ),
                        TextBlock::new()
                            .text(format!("Last command: {}", self.last_command))
                            .opacity(0.6),
                    )),
                ),
            )],
        )
    }
}
