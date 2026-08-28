use crate::controls::*;
use windows_reactor::*;

pub struct CommandBarPage {
    last_command: String,
}

#[derive(Clone)]
pub enum Message {
    CommandClicked(String),
}

impl Component for CommandBarPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            last_command: "(none)".to_string(),
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::CommandClicked(label) => self.last_command = label,
        }
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
                            context.callback(Message::CommandClicked),
                        ),
                        TextBlock::new()
                            .text(format!("Last command: {}", self.last_command))
                            .opacity(0.6),
                    )),
                    r#"CommandBar::new().owned_commands(primary, secondary, |label| ...)"#,
                ),
            )],
        )
    }
}
