#![windows_subsystem = "windows"]

use windows_reactor::{
    Element, Expander, FontWeight, RenderCx, StackPanel, TextBlock, hstack, vstack,
};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let details = cx.use_state(|| true);
    let more = cx.use_state(|| false);
    let settings = cx.use_state(|| true);
    let details_value = details.value();
    let more_value = more.value();
    let settings_value = settings.value();

    StackPanel::new([
        Expander::new(
            TextBlock::new("Details").build(),
            vstack(
                4.0,
                [
                    TextBlock::new("Hidden details live inside the expander.").build(),
                    TextBlock::new("Use the chevron to collapse this panel.")
                        .opacity(0.7)
                        .build(),
                ],
            ),
            move |value| {
                details.set(value);
            },
        )
        .expanded(details_value)
        .build(),
        Expander::new(
            TextBlock::new("More").build(),
            TextBlock::new("Collapsed by default.").build(),
            move |value| {
                more.set(value);
            },
        )
        .expanded(more_value)
        .build(),
        Expander::new(
            hstack(
                8.0,
                [
                    TextBlock::new("*").font_size(18.0).build(),
                    TextBlock::new("Settings")
                        .font_weight(FontWeight::BOLD)
                        .build(),
                ],
            ),
            TextBlock::new("Body content for the rich header expander.").build(),
            move |value| {
                settings.set(value);
            },
        )
        .expanded(settings_value)
        .build(),
    ])
    .spacing(8.0)
    .max_width(400.0)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Expander", app)
}
