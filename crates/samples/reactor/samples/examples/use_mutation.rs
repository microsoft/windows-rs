//! Sample for the `cx.use_mutation` hook.

use std::thread;
use std::time::Duration;

use windows_reactor::*;

fn save_data(name: &str) -> std::result::Result<String, String> {
    thread::sleep(Duration::from_millis(800));
    if name.is_empty() {
        Err("Name cannot be empty".to_string())
    } else {
        Ok(format!("Saved '{name}' successfully"))
    }
}

fn app(cx: &mut RenderCx) -> Element {
    let (name, set_name) = cx.use_state("Hello".to_string());
    let (save_state, save_trigger) = cx.use_mutation::<String>();

    let on_save = {
        let trigger = save_trigger.clone();
        let name = name.clone();
        move || {
            let n = name.clone();
            trigger.fire(move || save_data(&n));
        }
    };

    let on_save_empty = {
        let trigger = save_trigger;
        move || {
            trigger.fire(|| save_data(""));
        }
    };

    let on_name_changed = move |v: String| set_name.call(v);

    let status: Element = match &save_state {
        MutationState::Idle => text_block("Ready to save").into(),
        MutationState::Loading => ProgressRing::indeterminate().into(),
        MutationState::Success(msg) => text_block(msg).into(),
        MutationState::Error(e) => text_block(format!("Error: {e}")).into(),
    };

    vstack((
        text_block("use_mutation Demo").font_size(24.0),
        text_box(name)
            .header("Name")
            .on_text_changed(on_name_changed),
        hstack((
            button("Save")
                .enabled(!save_state.is_loading())
                .on_click(on_save),
            button("Save Empty (error)")
                .enabled(!save_state.is_loading())
                .on_click(on_save_empty),
        )),
        status,
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("UseMutation", app)
}
