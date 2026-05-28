//! Minimal sample for `error_boundary`.
//!
//! `error_boundary` wraps a subtree and renders a fallback element if
//! any descendant component panics while rendering. Toggle the button
//! to flip a flag that drives the child component.

#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, PartialEq)]
struct PanicMaybeProps {
    should_panic: bool,
}

fn panic_maybe(props: &PanicMaybeProps, _cx: &mut RenderCx) -> impl Into<Element> {
    assert!(
        !props.should_panic,
        "intentional render failure for the error-boundary demo"
    );
    text_block("Healthy child renders normally.").font_size(14.0)
}

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (should_panic, set_should_panic) = cx.use_state(false);

    let toggle = move || set_should_panic.call(!should_panic);

    let boundary = error_boundary(
        component(panic_maybe, PanicMaybeProps { should_panic }),
        |msg| {
            text_block(format!("⚠ Fallback UI — child panicked with: {msg}"))
                .font_size(13.0)
                .into()
        },
    );

    vstack((
        TitleBar::new("windows_reactor — error_boundary"),
        text_block(
            "Toggle the button to make the child panic on render. The boundary swaps in \
             the fallback subtree; toggle again to recover.",
        ),
        button(if should_panic {
            "Recover (stop panicking)"
        } else {
            "Trigger panic"
        })
        .on_click(toggle),
        boundary,
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
}

fn main() -> Result<()> {
    App::new()
        .title("windows_reactor — error_boundary")
        .render(app)
}
