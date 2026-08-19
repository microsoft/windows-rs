#![windows_subsystem = "windows"]

use windows_reactor::{
    ApplicationResource, Border, Button, Color, CornerRadius, Element, RenderCx, TextBlock,
    Thickness, vstack,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let styled = cx.use_state(|| true);
    let current = styled.value();
    let toggle = styled;

    let target = if current {
        Button::new("Delete")
            .resources([
                (
                    "ButtonBackground",
                    ApplicationResource::from(Color::rgb(178, 34, 34)),
                ),
                (
                    "ButtonForeground",
                    ApplicationResource::from(Color::rgb(255, 255, 255)),
                ),
                (
                    "ButtonBorderThemeThickness",
                    ApplicationResource::from(Thickness::uniform(0.0)),
                ),
                (
                    "ControlCornerRadius",
                    ApplicationResource::from(CornerRadius::uniform(8.0)),
                ),
            ])
            .build()
    } else {
        Button::new("Delete").build()
    };

    Border::new(vstack(
        12.0,
        [
            TextBlock::new("Element resources override WinUI lightweight styling values.").build(),
            target,
            Button::new(if current {
                "Clear resources"
            } else {
                "Apply resources"
            })
            .on_click(move || {
                toggle.set(!current);
            })
            .build(),
        ],
    ))
    .padding(Thickness::uniform(16.0))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Lightweight Resources", app)
}
