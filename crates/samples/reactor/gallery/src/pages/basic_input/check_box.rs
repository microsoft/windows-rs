use crate::controls::*;
use windows_reactor::*;

pub struct CheckBoxPage {
    accepted: bool,
    email: bool,
    sms: bool,
    push: bool,
}

#[derive(Clone)]
pub enum Message {
    Accepted(bool),
    Email(bool),
    Sms(bool),
    Push(bool),
}

impl Component for CheckBoxPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            accepted: false,
            email: true,
            sms: false,
            push: true,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Accepted(value) => self.accepted = value,
            Message::Email(value) => self.email = value,
            Message::Sms(value) => self.sms = value,
            Message::Push(value) => self.push = value,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let active = [self.email, self.sms, self.push]
            .into_iter()
            .filter(|value| *value)
            .count();
        page_content(
            "CheckBox",
            "A control that a user can select or clear.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic CheckBox",
                        StackPanel::new().spacing(8.0).children((
                            CheckBox::new()
                                .is_checked(self.accepted)
                                .on_is_checked_changed(context.callback(Message::Accepted))
                                .content("I accept the terms and conditions"),
                            TextBlock::new()
                                .text(if self.accepted {
                                    "Accepted"
                                } else {
                                    "Not yet accepted"
                                })
                                .opacity(0.6),
                        )),
                        "CheckBox::new().is_checked(accepted).on_is_checked_changed(handler)",
                    ),
                ),
                KeyedView::new(
                    "preferences",
                    sample_card(
                        "Notification Preferences",
                        StackPanel::new().spacing(6.0).children((
                            CheckBox::new()
                                .is_checked(self.email)
                                .on_is_checked_changed(context.callback(Message::Email))
                                .content("Email notifications"),
                            CheckBox::new()
                                .is_checked(self.sms)
                                .on_is_checked_changed(context.callback(Message::Sms))
                                .content("SMS notifications"),
                            CheckBox::new()
                                .is_checked(self.push)
                                .on_is_checked_changed(context.callback(Message::Push))
                                .content("Push notifications"),
                            TextBlock::new()
                                .text(format!("{active} channel(s) active"))
                                .opacity(0.6),
                        )),
                        "CheckBox::new().is_checked(value).on_is_checked_changed(handler)",
                    ),
                ),
                KeyedView::new(
                    "disabled",
                    sample_card(
                        "Disabled States",
                        StackPanel::new().spacing(6.0).children((
                            CheckBox::new()
                                .is_checked(true)
                                .is_enabled(false)
                                .content("Locked on"),
                            CheckBox::new()
                                .is_checked(false)
                                .is_enabled(false)
                                .content("Locked off"),
                        )),
                        "CheckBox::new().is_checked(true).is_enabled(false)",
                    ),
                ),
            ],
        )
    }
}
