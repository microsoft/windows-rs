use crate::controls::*;
use windows_reactor::*;

pub fn materials_page(
    backdrop: WindowBackdrop,
    on_backdrop_changed: Callback<WindowBackdrop>,
) -> View {
    let choice = |label: &'static str, value| {
        let callback = on_backdrop_changed.clone();
        Button::new()
            .style(if backdrop == value {
                ButtonStyle::Accent
            } else {
                ButtonStyle::Default
            })
            .on_click(move || callback.call(value))
            .content(label)
    };
    let description = match backdrop {
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
                ),
            ),
            KeyedView::new(
                "guidance",
                sample_card(
                    "Usage Guidance",
                    StackPanel::new().spacing(6.0).children((
                        "Use Mica for primary app windows.",
                        "Use Mica Alt for prominent tabs or sections.",
                        "Use Acrylic for transient surfaces.",
                        "Use None for a solid window background.",
                    )),
                ),
            ),
        ],
    )
}
