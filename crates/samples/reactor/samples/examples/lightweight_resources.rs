#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (styled, set_styled) = cx.use_state(true);

    let target: Element = if styled {
        button("Delete")
            .resource_overrides(|resources| {
                resources
                    .set("ButtonBackground", Color::rgb(178, 34, 34))
                    .set("ButtonForeground", Color::rgb(255, 255, 255))
                    .set("ButtonBorderThemeThickness", Thickness::uniform(0.0))
                    .set("ControlCornerRadius", CornerRadius::uniform(8.0))
            })
            .into()
    } else {
        button("Delete").into()
    };

    vstack((
        text_block("Element resources override WinUI lightweight styling values."),
        target,
        button(if styled {
            "Clear resources"
        } else {
            "Apply resources"
        })
        .on_click(move || set_styled.call(!styled)),
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("LightweightResources", app)
}
