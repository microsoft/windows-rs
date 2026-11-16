use super::*;

mod declarations;
mod reference;
mod selection;

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
