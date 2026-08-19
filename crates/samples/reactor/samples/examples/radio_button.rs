#![windows_subsystem = "windows"]

use windows_reactor::{Element, RadioButton, RenderCx, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let selected = cx.use_state(|| 1_u64);
    let current = selected.value();
    let label = ["Small", "Medium", "Large"][current as usize];
    let select_small = selected.clone();
    let select_medium = selected.clone();
    let select_large = selected;

    vstack(
        4.0,
        [
            TextBlock::new("Size").font_size(20.0).build(),
            RadioButton::new("Small", current == 0, move |checked| {
                if checked {
                    select_small.set(0);
                }
            })
            .group_name("size")
            .build(),
            TextBlock::new("The choices may be separated by arbitrary content.").build(),
            RadioButton::new("Medium", current == 1, move |checked| {
                if checked {
                    select_medium.set(1);
                }
            })
            .group_name("size")
            .build(),
            RadioButton::new("Large", current == 2, move |checked| {
                if checked {
                    select_large.set(2);
                }
            })
            .group_name("size")
            .build(),
            TextBlock::new(format!("size = {label}")).build(),
            RadioButton::display("Disabled", true)
                .group_name("disabled")
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("RadioButton", app)
}
