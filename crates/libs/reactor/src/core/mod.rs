macro_rules! public_submodules {
    ($($mod:ident),* $(,)?) => {
        $( pub mod $mod; )*
        $( #[allow(unused_imports)] pub use $mod::*; )*
    };
}

macro_rules! internal_submodules {
    ($($mod:ident),* $(,)?) => {
        $( pub(crate) mod $mod; )*
        $( #[allow(unused_imports)] pub(crate) use $mod::*; )*
    };
}

// Modules exposed as `windows_reactor::core::X` for downstream use.
public_submodules! {
    animation,
    backend,
    callback,
    component,
    component_element,
    context,
    custom,
    dispatcher,
    element,
    error_boundary,
    reconciler,
    render_context,
    render_host,
    rich_text,
    templated_list,
    theme,
    window,
}

// Modules whose types are re-exported at the crate root but whose module
// paths are not part of the public API.
internal_submodules! {
    accessibility,
    geometry,
    keyboard,
    modifiers,
    pointer,
    resource,
    tooltip,
    widgets,
}

pub(crate) mod prop_binding;
pub(crate) mod widget;
#[allow(unused_imports)]
pub(crate) use prop_binding::*;
#[allow(unused_imports)]
pub(crate) use widget::*;

pub(crate) mod generated_bindings;

pub(crate) mod into_elements;
pub(crate) mod rc_fn;
pub use into_elements::IntoElements;
