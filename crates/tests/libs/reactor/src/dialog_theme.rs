use super::*;

pub(super) struct DialogThemeFixture {
    theme: WindowTheme,
    scheme: Option<ColorScheme>,
    open: bool,
    attached: bool,
    replacement: bool,
}

#[derive(Clone, Copy)]
pub(super) enum Message {
    Theme(WindowTheme),
    Scheme(ColorScheme),
    Open(bool),
    ReplaceOwner,
}

impl Component for DialogThemeFixture {
    type Input = (WindowTheme, bool);
    type Message = Message;

    fn create(input: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self {
            theme: input.0,
            scheme: None,
            open: input.1,
            attached: input.1,
            replacement: false,
        }
    }

    fn update(&mut self, message: Message, _: &ComponentContext<Self>) {
        match message {
            Message::Theme(theme) => self.theme = theme,
            Message::Scheme(scheme) => self.scheme = Some(scheme),
            Message::Open(open) => {
                self.open = open;
                self.attached |= open;
            }
            Message::ReplaceOwner => self.replacement = !self.replacement,
        }
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        context.window_title(match input {
            (WindowTheme::Light, true) => "Reactor UI fixture: dialog-theme-light",
            (WindowTheme::Dark, true) => "Reactor UI fixture: dialog-theme-dark",
            _ => "Reactor UI fixture: dialog-theme-late",
        });
        context.window_visuals(
            WindowVisuals::new()
                .client_size(600.0, 500.0)
                .theme(self.theme),
        );
        context.on_color_scheme(context.callback(Message::Scheme));
        let theme_button = |id, theme| {
            Button::new()
                .automation_id(id)
                .content(id)
                .on_click(context.message(Message::Theme(theme)))
        };
        let controls = StackPanel::new().margin(20.0).spacing(12.0).children((
            theme_button("window-light", WindowTheme::Light),
            theme_button("window-dark", WindowTheme::Dark),
            Button::new()
                .automation_id("open")
                .content("Open")
                .on_click(context.message(Message::Open(true))),
            TextBlock::new()
                .automation_id("dialog-state")
                .text(if self.open { "Open" } else { "Closed" }),
        ));
        let owner: View = if self.replacement {
            Border::new().content(controls).into()
        } else {
            Grid::new().children([controls.into()]).into()
        };
        if !self.attached {
            return owner;
        }
        owner.content_dialog(
            ContentDialog::new()
                .title("Window theme")
                .close_button_text("Close")
                .is_open(self.open)
                .on_closed(context.callback(|_| Message::Open(false)))
                .content(
                    StackPanel::new().spacing(8.0).children((
                        TextBlock::new()
                            .automation_id("scheme")
                            .is_text_selection_enabled(true)
                            .foreground(ThemeBrush::PrimaryText)
                            .text(format!(
                                "Requested {:?}; owner {:?}",
                                self.theme, self.scheme
                            )),
                        theme_button("dialog-light", WindowTheme::Light),
                        theme_button("dialog-dark", WindowTheme::Dark),
                        theme_button("dialog-system", WindowTheme::System),
                        TextBlock::new()
                            .automation_id("owner-generation")
                            .text(format!("Owner {}", u8::from(self.replacement))),
                        Button::new()
                            .automation_id("replace-owner")
                            .content("Replace owner")
                            .on_click(context.message(Message::ReplaceOwner)),
                    )),
                ),
        )
    }
}
