/// Generates `attach_event` dispatch arms from control declarations.
///
/// Events with `add_method` + `invoke` are auto-generated.
/// Events with `manual = true` are skipped; they use hand-written
/// attach_event arms in the backend.
use proc_macro2::TokenStream;
use quote::quote;

use crate::helpers::*;
use crate::metadata::MetadataResolver;
use crate::schema::*;

/// Returns `h` (via Deref) if iface is the default for the handle, else `h.cast::<bindings::I...>().unwrap()`.
fn event_receiver(iface: &proc_macro2::Ident, handle_name: &str) -> TokenStream {
    if iface == &format!("I{handle_name}") {
        quote! { h }
    } else {
        quote! { h.cast::<bindings::#iface>().unwrap() }
    }
}

pub fn generate(controls: &[Control], resolver: &MetadataResolver) -> String {
    let mut all_arms = Vec::new();

    for ctrl in controls {
        let arms = generate_control_event_arms(ctrl, resolver);
        all_arms.extend(arms);
    }

    format_generated(quote! {
        use super::*;

        /// Try to handle an `attach_event` call via generated dispatch.
        /// Returns `Some(revokers)` if handled, `None` to fall through.
        pub fn dispatch(
            handle: &Handle,
            event: Event,
            handler: &EventHandler,
        ) -> Option<Vec<windows_core::EventRevoker>> {
            let mut revokers = Vec::new();
            match (event, handle) {
                #(#all_arms)*
                _ => return None,
            }
            Some(revokers)
        }
    })
}

fn generate_control_event_arms(ctrl: &Control, resolver: &MetadataResolver) -> Vec<TokenStream> {
    if ctrl.event.is_empty() {
        return Vec::new();
    }
    let handle = ident(ctrl.handle());
    ctrl.event
        .iter()
        .filter_map(|e| gen_event_arm(&handle, ctrl.handle(), e, resolver))
        .collect()
}

/// Generate a captured-control getter pattern: capture the typed control
/// handle at attach time and read the getter through it on each event, so
/// there is no per-event `QueryInterface` on `sender`.
fn gen_sender_getter(
    iface: &proc_macro2::Ident,
    add_ident: &proc_macro2::Ident,
    handle_name: &str,
    getter: &str,
    invoke_call: TokenStream,
    default: TokenStream,
) -> TokenStream {
    let getter_ident = ident(getter);
    let receiver = event_receiver(iface, handle_name);
    quote! {
        let handler = handler.clone();
        let control = h.clone();
        revokers.push(
            #receiver
                .#add_ident(move |_sender, _args| {
                    let v = control.#getter_ident().unwrap_or(#default);
                    handler.#invoke_call;
                })
                .unwrap(),
        );
    }
}

/// Generate an args-getter pattern: read value from event args via getter, invoke handler.
fn gen_args_getter(
    iface: &proc_macro2::Ident,
    add_ident: &proc_macro2::Ident,
    handle_name: &str,
    getter: &str,
    invoke_call: TokenStream,
) -> TokenStream {
    let getter_ident = ident(getter);
    let receiver = event_receiver(iface, handle_name);
    quote! {
        let handler = handler.clone();
        revokers.push(
            #receiver
                .#add_ident(move |_sender, args| {
                    if let Some(a) = args.as_ref()
                        && let Ok(v) = a.#getter_ident()
                    {
                        handler.#invoke_call;
                    }
                })
                .unwrap(),
        );
    }
}

fn require_property(e: &EventDecl, handle_name: &str, pattern: &str) -> String {
    let prop = e.property.as_deref().unwrap_or_else(|| {
        panic!(
            "event '{}.{}': {pattern} requires property",
            handle_name,
            e.event()
        )
    });
    prop.to_string()
}

fn gen_event_arm(
    handle: &proc_macro2::Ident,
    handle_name: &str,
    e: &EventDecl,
    resolver: &MetadataResolver,
) -> Option<TokenStream> {
    if e.manual {
        return None;
    }

    let event_name = e.event();
    let add_method = e.add_method().unwrap_or_else(|| {
        panic!("event '{handle_name}.{event_name}' has neither manual nor add_method")
    });
    let invoke = e.invoke();

    let event = ident(&event_name);
    let iface_ref = resolver
        .resolve(handle_name, &add_method)
        .unwrap_or_else(|| panic!("metadata: cannot resolve {handle_name}.{add_method} for event"));
    let iface = ident(iface_ref.short_name());
    // The generated bindings use the bare event name (e.g. "Click" not
    // "add_Click") since the remove method is suppressed.
    let add_ident = ident(&event_name);

    let handler_body = match invoke {
        "invoke" => {
            let receiver = event_receiver(&iface, handle_name);
            quote! {
                let handler = handler.clone();
                revokers.push(
                    #receiver
                        .#add_ident(move |_sender, _args| {
                            handler.invoke();
                        })
                        .unwrap(),
                );
            }
        }
        "invoke_bool_getter" => {
            let g = require_property(e, handle_name, invoke);
            gen_sender_getter(
                &iface,
                &add_ident,
                handle_name,
                &g,
                quote!(invoke_bool(v)),
                quote!(false),
            )
        }
        "invoke_string_getter" => {
            let g = require_property(e, handle_name, invoke);
            gen_sender_getter(
                &iface,
                &add_ident,
                handle_name,
                &g,
                quote!(invoke_string(v)),
                quote!(Default::default()),
            )
        }
        "invoke_i32_getter" => {
            let g = require_property(e, handle_name, invoke);
            gen_sender_getter(
                &iface,
                &add_ident,
                handle_name,
                &g,
                quote!(invoke_i32(v)),
                quote!(-1),
            )
        }
        "invoke_f64_getter" => {
            let g = require_property(e, handle_name, invoke);
            gen_sender_getter(
                &iface,
                &add_ident,
                handle_name,
                &g,
                quote!(invoke_f64(v)),
                quote!(-1.0),
            )
        }
        "invoke_i32_args" => {
            let g = require_property(e, handle_name, invoke);
            gen_args_getter(&iface, &add_ident, handle_name, &g, quote!(invoke_i32(v)))
        }
        "invoke_f64_args" => {
            let g = require_property(e, handle_name, invoke);
            gen_args_getter(&iface, &add_ident, handle_name, &g, quote!(invoke_f64(v)))
        }
        "invoke_bool_dual" => {
            let false_evt = e.false_event.as_deref().unwrap_or_else(|| {
                panic!("event '{handle_name}.{event_name}': invoke_bool_dual requires false_event")
            });
            let add_false = format!("add_{false_evt}");
            let add_false_ident = ident(false_evt);
            let iface_false_ref = resolver
                .resolve(handle_name, &add_false)
                .unwrap_or_else(|| {
                    panic!("metadata: cannot resolve {handle_name}.{add_false} for event")
                });
            let iface_false = ident(iface_false_ref.short_name());
            let receiver = event_receiver(&iface, handle_name);
            let receiver_false = event_receiver(&iface_false, handle_name);
            quote! {
                let true_handler = handler.clone();
                revokers.push(
                    #receiver
                        .#add_ident(move |_sender, _args| {
                            true_handler.invoke_bool(true);
                        })
                        .unwrap(),
                );
                let false_handler = handler.clone();
                revokers.push(
                    #receiver_false
                        .#add_false_ident(move |_sender, _args| {
                            false_handler.invoke_bool(false);
                        })
                        .unwrap(),
                );
            }
        }
        _ => panic!("event '{handle_name}.{event_name}': unknown invoke pattern '{invoke}'"),
    };

    Some(quote! {
        (Event::#event, Handle::#handle(h)) => {
            #handler_body
        }
    })
}
