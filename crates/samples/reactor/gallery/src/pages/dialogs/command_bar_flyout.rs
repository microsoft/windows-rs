use crate::controls::*;
use windows_reactor::*;

pub struct CommandBarFlyoutPage {
    last_action: String,
}

impl Component for CommandBarFlyoutPage {
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
        page_content(
            "CommandBarFlyout",
            "A flyout that provides quick access to common commands.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic CommandBarFlyout",
                        StackPanel::new().spacing(8.0).children((
                            Button::new().content("Show Commands").command_bar_flyout(
                                CommandBarFlyout::new(
                                    [
                                        CommandBarCommand::button_with_icon(
                                            "cut",
                                            "Cut",
                                            Symbol::Cut,
                                        ),
                                        CommandBarCommand::button_with_icon(
                                            "copy",
                                            "Copy",
                                            Symbol::Copy,
                                        ),
                                        CommandBarCommand::button_with_icon(
                                            "paste",
                                            "Paste",
                                            Symbol::Paste,
                                        ),
                                    ],
                                    [],
                                    context.callback(std::convert::identity),
                                ),
                            ),
                            TextBlock::new()
                                .text(format!("Last action: {}", self.last_action))
                                .opacity(0.6),
                        )),
                        r#"Button::new()
    .content("Show Commands")
    .command_bar_flyout(CommandBarFlyout::new(
        [
            CommandBarCommand::button_with_icon("cut", "Cut", Symbol::Cut),
            CommandBarCommand::button_with_icon("copy", "Copy", Symbol::Copy),
            CommandBarCommand::button_with_icon("paste", "Paste", Symbol::Paste),
        ],
        [],
        context.callback(std::convert::identity),
    ))"#,
                    ),
                ),
                KeyedView::new(
                    "secondary",
                    sample_card(
                        "CommandBarFlyout with Secondary Commands",
                        Button::new().content("More Options").command_bar_flyout(
                            CommandBarFlyout::new(
                                [CommandBarCommand::button_with_icon(
                                    "share",
                                    "Share",
                                    Symbol::Send,
                                )],
                                [
                                    CommandBarCommand::button("select-all", "Select All"),
                                    CommandBarCommand::separator("separator"),
                                    CommandBarCommand::button("print", "Print"),
                                ],
                                context.callback(std::convert::identity),
                            ),
                        ),
                        r#"Button::new()
    .content("More Options")
    .command_bar_flyout(CommandBarFlyout::new(
        [CommandBarCommand::button_with_icon("share", "Share", Symbol::Send)],
        [
            CommandBarCommand::button("select-all", "Select All"),
            CommandBarCommand::separator("separator"),
            CommandBarCommand::button("print", "Print"),
        ],
        context.callback(std::convert::identity),
    ))"#,
                    ),
                ),
            ],
        )
    }
}
