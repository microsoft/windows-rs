use crate::controls::*;
use windows_reactor::*;

#[derive(Clone, PartialEq)]
pub struct MaterialsInput {
    pub backdrop: WindowBackdrop,
    pub on_backdrop_changed: Callback<WindowBackdrop>,
}

pub struct MaterialsPage;

impl Component for MaterialsPage {
    type Message = ();
    type Input = MaterialsInput;

    fn create(_: &Self::Input, _: &ComponentContext<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: (), _: &ComponentContext<Self>) {}

    fn view(&self, input: &Self::Input, _: &mut ViewContext<Self>) -> View {
        let choice = |label: &'static str, backdrop| {
            let callback = input.on_backdrop_changed.clone();
            Button::new()
                .style(if input.backdrop == backdrop {
                    ButtonStyle::Accent
                } else {
                    ButtonStyle::Default
                })
                .on_click(Callback::new(move |_| {
                    let _ = callback.call(backdrop);
                }))
                .content(TextBlock::new().text(label))
        };
        let description = match input.backdrop {
            WindowBackdrop::Mica => "Mica samples the desktop wallpaper with a subtle tint.",
            WindowBackdrop::MicaAlt => "Mica Alt uses a stronger tint for tabbed interfaces.",
            WindowBackdrop::Acrylic => "Acrylic provides a translucent blurred material.",
            WindowBackdrop::None => "No system backdrop; the window uses a solid background.",
        };
        page_content(
            "Materials",
            "Window backdrop materials provide depth and hierarchy.",
            [
                KeyedView::new(
                    "switcher",
                    sample_card(
                        "Live Backdrop Switcher",
                        StackPanel::new().spacing(8.0).children((
                            StackPanel::new()
                                .orientation(Orientation::Horizontal)
                                .spacing(8.0)
                                .children((
                                    choice("Mica", WindowBackdrop::Mica),
                                    choice("Mica Alt", WindowBackdrop::MicaAlt),
                                    choice("Acrylic", WindowBackdrop::Acrylic),
                                    choice("None", WindowBackdrop::None),
                                )),
                            TextBlock::new().text(description).opacity(0.7),
                        )),
                        "WindowVisuals::new().backdrop(WindowBackdrop::Mica)",
                    ),
                ),
                KeyedView::new(
                    "guidance",
                    sample_card(
                        "Usage Guidance",
                        StackPanel::new().spacing(6.0).children((
                            TextBlock::new().text("Use Mica for primary app windows."),
                            TextBlock::new().text("Use Mica Alt for prominent tabs or sections."),
                            TextBlock::new().text("Use Acrylic for transient surfaces."),
                            TextBlock::new().text("Use None for a solid window background."),
                        )),
                        "WindowBackdrop::{Mica, MicaAlt, Acrylic, None}",
                    ),
                ),
            ],
        )
    }
}
