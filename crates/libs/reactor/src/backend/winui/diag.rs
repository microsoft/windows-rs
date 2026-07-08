use std::fmt::Arguments;

use super::*;

/// Emit a best-effort backend diagnostic. Warns under debug builds; a no-op in
/// release. Every reactor backend diagnostic routes through here, so tolerated
/// failures are visible while developing without any release-build cost.
pub fn warn(args: Arguments<'_>) {
    if cfg!(debug_assertions) {
        eprintln!("windows-reactor: {args}");
    }
}

/// Report a best-effort backend operation whose failure is tolerated, in place
/// of a silent `let _ = <com_call>`. Warns with the call site under debug
/// builds; a no-op in release.
#[track_caller]
pub fn dropped<T>(result: Result<T>) {
    if let Err(err) = result
        && cfg!(debug_assertions)
    {
        warn(format_args!(
            "ignored error at {}: {err:?}",
            std::panic::Location::caller()
        ));
    }
}

pub fn unhandled_prop(id: ControlId, prop: Prop, value: &PropValue, handle: &Handle) {
    warn(format_args!(
        "unhandled prop ({prop:?}, {value:?}) for {} {id}",
        handle.kind_name()
    ));
}

pub fn unhandled_modifier(site: &str, prop: Prop, handle: &Handle) {
    warn(format_args!(
        "{site}: ignoring {prop:?} for {}",
        handle.kind_name()
    ));
}
