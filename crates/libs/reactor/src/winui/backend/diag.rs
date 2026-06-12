use super::*;

pub(crate) fn unhandled_prop(id: ControlId, prop: Prop, value: &PropValue, handle: &Handle) {
    if cfg!(debug_assertions) {
        eprintln!(
            "windows-reactor: unhandled prop ({prop:?}, {value:?}) for {} {id}",
            handle.kind_name()
        );
    }
}

pub(crate) fn unhandled_modifier(site: &str, prop: Prop, handle: &Handle) {
    if cfg!(debug_assertions) {
        eprintln!(
            "windows-reactor: {site}: ignoring {prop:?} for {}",
            handle.kind_name()
        );
    }
}

pub(crate) fn com_error(site: &str, id: ControlId, err: &windows_core::Error) {
    if cfg!(debug_assertions) {
        eprintln!("windows-reactor: {site} on {id}: {err:?}");
    }
}
