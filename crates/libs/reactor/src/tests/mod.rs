use super::*;

mod attachments;
mod declarations;
mod dialogs;
mod events;
mod failures;
mod feedback;
mod input;
mod reconcile;
mod reference;
mod retirement;
mod selection;
mod virtualization;

#[track_caller]
pub(crate) fn assert_panics(expected: &str, f: impl FnOnce()) {
    let payload = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).unwrap_err();
    let message = payload
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| payload.downcast_ref::<&str>().copied())
        .unwrap();
    assert!(
        message.contains(expected),
        "panic {message:?} does not contain {expected:?}"
    );
}

fn text(value: &str) -> View {
    TextBlock::new().text(value).into()
}
