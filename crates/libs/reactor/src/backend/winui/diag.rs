use super::*;

pub fn unhandled_prop(id: ControlId, prop: Prop, value: &PropValue, handle: &Handle) {
    if cfg!(debug_assertions) {
        eprintln!(
            "windows-reactor: unhandled prop ({prop:?}, {value:?}) for {} {id}",
            handle.kind_name()
        );
    }
}

pub fn unhandled_modifier(site: &str, prop: Prop, handle: &Handle) {
    if cfg!(debug_assertions) {
        eprintln!(
            "windows-reactor: {site}: ignoring {prop:?} for {}",
            handle.kind_name()
        );
    }
}

pub fn com_error(site: &str, id: ControlId, err: &Error) {
    if cfg!(debug_assertions) {
        eprintln!("windows-reactor: {site} on {id}: {err:?}");
    }
}
