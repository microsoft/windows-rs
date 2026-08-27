use crate::controls::*;
use windows_reactor::*;

#[derive(Clone)]
pub enum Message {
    Deleted,
}

pub struct FlyoutPage {
    deleted: bool,
}

impl Component for FlyoutPage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self { deleted: false }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::Deleted => self.deleted = true,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "Flyout",
            "A lightweight popup for contextual info.",
            [
                KeyedView::new(
                    "confirmation",
                    sample_card(
                        "Confirmation Flyout",
                        StackPanel::new().spacing(8.0).children((
                            Button::new()
                                .content(TextBlock::new().text("Delete item"))
                                .flyout_with(Flyout::rich(
                                    StackPanel::new().spacing(8.0).children((
                                        TextBlock::new().text("Are you sure?"),
                                        TextBlock::new().text("This cannot be undone."),
                                        Button::new()
                                            .on_click(context.message(Message::Deleted))
                                            .content(TextBlock::new().text("Confirm delete")),
                                    )),
                                )),
                            TextBlock::new()
                                .text(if self.deleted {
                                    "Item deleted!"
                                } else {
                                    "No action taken"
                                })
                                .opacity(0.6),
                        )),
                        r#"Button::new()
    .content(TextBlock::new().text("Delete item"))
    .flyout_with(Flyout::rich(
        Button::new()
            .content(TextBlock::new().text("Confirm delete"))
            .on_click(context.message(Message::Deleted)),
    ))"#,
                    ),
                ),
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Button with Flyout",
                        Button::new()
                            .content(TextBlock::new().text("Click for flyout"))
                            .flyout("This is a flyout!"),
                        r#"Button::new()
    .content(TextBlock::new().text("Click for flyout"))
    .flyout("This is a flyout!")"#,
                    ),
                ),
                KeyedView::new(
                    "multiple",
                    sample_card(
                        "Multiple Flyout Buttons",
                        StackPanel::new()
                            .orientation(Orientation::Horizontal)
                            .spacing(8.0)
                            .children((
                                Button::new()
                                    .content(TextBlock::new().text("Help"))
                                    .flyout("Press F1 for more help."),
                                Button::new()
                                    .content(TextBlock::new().text("Info"))
                                    .flyout_with(
                                        Flyout::text("This operation cannot be undone.")
                                            .placement(FlyoutPlacement::Bottom),
                                    ),
                            )),
                        r#"Button::new().content(TextBlock::new().text("Help")).flyout("Press F1 for more help."),
Button::new()
    .content(TextBlock::new().text("Info"))
    .flyout_with(Flyout::text("This operation cannot be undone.").placement(FlyoutPlacement::Bottom))"#,
                    ),
                ),
            ],
        )
    }
}
