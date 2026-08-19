#![windows_subsystem = "windows"]

use std::thread;
use std::time::Duration;

use windows_reactor::{
    Button, Element, MutationState, ProgressRing, RenderCx, TextBlock, TextBox, hstack, vstack,
};

fn save_data(name: &str) -> Result<String, String> {
    thread::sleep(Duration::from_millis(800));
    if name.is_empty() {
        Err("Name cannot be empty".to_string())
    } else {
        Ok(format!("Saved '{name}' successfully"))
    }
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let name = cx.use_state(|| "Hello".to_string());
    let (save_state, save) = cx.use_mutation::<String>();

    let save_name = name.value();
    let save_success = save.clone();
    let save_error = save;
    let loading = save_state.is_loading();
    let status = match save_state {
        MutationState::Idle => TextBlock::new("Ready to save").build(),
        MutationState::Loading => ProgressRing::indeterminate().build(),
        MutationState::Success(message) => TextBlock::new(message).build(),
        MutationState::Error(error) => TextBlock::new(format!("Error: {error}")).build(),
    };

    vstack(
        12.0,
        [
            TextBlock::new("use_mutation Demo").font_size(24.0).build(),
            TextBox::new(name.value(), move |value| {
                name.set(value);
            })
            .header("Name")
            .build(),
            hstack(
                8.0,
                [
                    Button::new("Save")
                        .on_click(move || {
                            let name = save_name.clone();
                            save_success.fire(move || save_data(&name));
                        })
                        .enabled(!loading)
                        .build(),
                    Button::new("Save Empty (error)")
                        .on_click(move || {
                            save_error.fire(|| save_data(""));
                        })
                        .enabled(!loading)
                        .build(),
                ],
            ),
            status,
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Use Mutation", app)
}
