use crate::controls::*;
use windows_reactor::*;

pub struct PasswordBoxPage {
    password: String,
}

impl Component for PasswordBoxPage {
    type Message = String;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            password: String::new(),
        }
    }

    fn update(&mut self, password: String, _: &ComponentContext<Self>) {
        self.password = password;
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        page_content(
            "PasswordBox",
            "A text input that conceals typed characters.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic PasswordBox",
                        StackPanel::new().spacing(8.0).children((
                            PasswordBox::new()
                                .password(&self.password)
                                .placeholder_text("Enter password")
                                .on_password_changed(context.callback(std::convert::identity))
                                .slots([SlotView::new(PasswordBoxSlot::Header, "Password")]),
                            TextBlock::new()
                                .text(format!("Length: {} chars", self.password.len()))
                                .opacity(0.6),
                        )),
                        "PasswordBox::new().password(value).on_password_changed(handler)",
                    ),
                ),
                KeyedView::new(
                    "reveal",
                    sample_card(
                        "PasswordBox with Reveal Button",
                        PasswordBox::new()
                            .placeholder_text("Enter secret")
                            .password_reveal_mode(PasswordRevealMode::Peek),
                        "PasswordBox::new().password_reveal_mode(PasswordRevealMode::Peek)",
                    ),
                ),
                KeyedView::new(
                    "disabled",
                    sample_card(
                        "Disabled PasswordBox",
                        PasswordBox::new().password("hunter2").is_enabled(false),
                        "PasswordBox::new().password(value).is_enabled(false)",
                    ),
                ),
            ],
        )
    }
}
