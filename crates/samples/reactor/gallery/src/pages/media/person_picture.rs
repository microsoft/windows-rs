use crate::controls::*;
use windows_reactor::*;

pub struct PersonPicturePage {
    show_display_names: bool,
}

pub enum Message {
    SetShowDisplayNames(bool),
}

impl Component for PersonPicturePage {
    type Message = Message;
    type Input = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            show_display_names: true,
        }
    }

    fn update(&mut self, message: Message, _context: &ComponentContext<Self>) {
        match message {
            Message::SetShowDisplayNames(value) => self.show_display_names = value,
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        let pictures: View = if self.show_display_names {
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(16.0)
                .children((
                    PersonPicture::new().display_name("Alice Smith"),
                    PersonPicture::new().display_name("Bob Johnson"),
                    PersonPicture::new().display_name("Carol Lee"),
                ))
        } else {
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(16.0)
                .children((
                    PersonPicture::new().initials("AS"),
                    PersonPicture::new().initials("BJ"),
                    PersonPicture::new().initials("CL"),
                ))
        };

        page_content(
            "PersonPicture",
            "A circular avatar for a person.",
            [KeyedView::new(
                "display-names-or-initials",
                sample_card(
                    "Display Names or Initials",
                    StackPanel::new().spacing(12.0).children((
                        ToggleSwitch::new()
                            .is_on(self.show_display_names)
                            .on_toggled(context.callback(Message::SetShowDisplayNames))
                            .slots([SlotView::new(ToggleSwitchSlot::Header, "Use display names")]),
                        pictures,
                        TextBlock::new()
                            .text(if self.show_display_names {
                                "Initials are generated from the display names."
                            } else {
                                "Initials can also be provided directly."
                            })
                            .opacity(0.6),
                    )),
                    r#"ToggleSwitch::new().is_on(value).on_toggled(...).slots([SlotView::new(
    ToggleSwitchSlot::Header, "Use display names")])
PersonPicture::new().display_name("Alice Smith")
PersonPicture::new().initials("AS")"#,
                ),
            )],
        )
    }
}
