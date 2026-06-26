//! Sample for the `RadioButton` element.

use windows_reactor::*;

#[derive(Copy, Clone, PartialEq, Eq, Default)]
enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

fn app(cx: &mut RenderCx) -> Element {
    let (size, set_size) = cx.use_state(Size::default());

    let choose = |value: Size| {
        let set_size = set_size.clone();
        move || set_size.call(value)
    };

    let label = match size {
        Size::Small => "Small",
        Size::Medium => "Medium",
        Size::Large => "Large",
    };

    vstack((
        RadioButton::new("Small")
            .group("size")
            .checked(size == Size::Small)
            .on_checked(choose(Size::Small)),
        RadioButton::new("Medium")
            .group("size")
            .checked(size == Size::Medium)
            .on_checked(choose(Size::Medium)),
        RadioButton::new("Large")
            .group("size")
            .checked(size == Size::Large)
            .on_checked(choose(Size::Large)),
        text_block(format!("size = {label}")),
        RadioButton::new("Disabled")
            .group("other")
            .checked(true)
            .enabled(false),
    ))
    .spacing(4.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("RadioButton", app)
}
