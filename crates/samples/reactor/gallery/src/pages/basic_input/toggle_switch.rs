use crate::controls::*;
use windows_reactor::*;

pub struct ToggleSwitchPage {
    wifi: bool,
    notifications: bool,
    automation: bool,
    overnight: bool,
}

#[derive(Clone)]
pub enum Message {
    Wifi(bool),
    Notifications(bool),
    Automation(bool),
    Overnight(bool),
}

impl Component for ToggleSwitchPage {
    type Message = Message;
    type Input = ();

    fn create(_: &(), _: &ComponentContext<Self>) -> Self {
        Self {
            wifi: true,
            notifications: false,
            automation: false,
            overnight: true,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Wifi(value) => self.wifi = value,
            Message::Notifications(value) => self.notifications = value,
            Message::Automation(value) => self.automation = value,
            Message::Overnight(value) => self.overnight = value,
        }
    }

    fn view(&self, _: &(), context: &mut ViewContext<Self>) -> View {
        let switch = |is_on, header, message: fn(bool) -> Message| {
            ToggleSwitch::new()
                .is_on(is_on)
                .on_toggled(context.callback(message))
                .header(header)
                .on_content("On")
                .off_content("Off")
        };
        page_content(
            "ToggleSwitch",
            "A compact switch for turning a setting on or off.",
            [
                KeyedView::new(
                    "basic",
                    sample_card(
                        "Basic Toggle",
                        StackPanel::new().spacing(8.0).children((
                            switch(self.wifi, "Wi-Fi", Message::Wifi),
                            TextBlock::new()
                                .text(if self.wifi {
                                    "Wi-Fi is connected."
                                } else {
                                    "Wi-Fi is off."
                                })
                                .opacity(0.6),
                        )),
                        "ToggleSwitch::new().is_on(value).on_toggled(handler)",
                    ),
                ),
                KeyedView::new(
                    "notifications",
                    sample_card(
                        "Toggle with Header",
                        StackPanel::new().spacing(8.0).children((
                            switch(self.notifications, "Notifications", Message::Notifications),
                            TextBlock::new()
                                .text(if self.notifications {
                                    "Priority alerts will appear."
                                } else {
                                    "Notifications are muted."
                                })
                                .opacity(0.6),
                        )),
                        "ToggleSwitch::new().header(header).on_content(on).off_content(off)",
                    ),
                ),
                KeyedView::new(
                    "dependent",
                    sample_card(
                        "Disabled State",
                        StackPanel::new().spacing(8.0).children((
                            switch(
                                self.automation,
                                "Enable scheduled updates",
                                Message::Automation,
                            ),
                            ToggleSwitch::new()
                                .is_on(self.overnight)
                                .is_enabled(self.automation)
                                .on_toggled(context.callback(Message::Overnight))
                                .header("Install updates overnight"),
                        )),
                        "ToggleSwitch::new().is_enabled(parent_enabled)",
                    ),
                ),
            ],
        )
    }
}
