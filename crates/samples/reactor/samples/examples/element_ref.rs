use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let input = cx.use_element_ref::<TextBoxHandle>();
    let input_for_focus = input.clone();
    let (status, set_status) = cx.use_state("Not focused");

    vstack((
        text_block(
            "The typed reference exists across renders, points at the TextBox only while mounted, \
             and cannot be attached to a different widget type.",
        ),
        text_box("Focus target").element_ref(&input),
        button("Focus TextBox").on_click(move || {
            let status = match input_for_focus.focus() {
                Ok(true) => "Focused",
                Ok(false) => "Focus rejected",
                Err(_) => "Focus failed",
            };
            set_status.call(status);
        }),
        text_block(status),
    ))
    .spacing(8.0)
    .padding(16.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Typed Element Reference", app)
}
