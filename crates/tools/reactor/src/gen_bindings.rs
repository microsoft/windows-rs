/// Generates `bindings()` helper functions from control declarations.
use proc_macro2::TokenStream;
use quote::quote;

use crate::helpers::*;
use crate::schema::*;

pub fn generate(controls: &[Control]) -> String {
    let fns: Vec<TokenStream> = controls.iter().map(generate_one).collect();

    format_generated(quote! {
        use super::*;

        #(#fns)*
    })
}

fn generate_one(ctrl: &Control) -> TokenStream {
    let fn_name = ident(&format!("{}_bindings", to_snake_case(&ctrl.name)));
    let widget_type = ident(&ctrl.name);

    let has_optional_props = ctrl.prop.iter().any(|p| p.is_optional());

    let prop_stmts: Vec<TokenStream> = ctrl.prop.iter().map(gen_prop_binding).collect();
    let event_stmts: Vec<TokenStream> = ctrl.event.iter().map(gen_event_binding).collect();
    let cap = ctrl.prop.len() + ctrl.event.len();

    if cap == 0 {
        quote! {
            pub(crate) fn #fn_name(_w: &#widget_type) -> PropBindings {
                Vec::new()
            }
        }
    } else if !has_optional_props {
        // All items are unconditional — use vec![...] directly
        let prop_items: Vec<TokenStream> = ctrl.prop.iter().map(gen_prop_item).collect();
        let event_items: Vec<TokenStream> = ctrl.event.iter().map(gen_event_item).collect();
        quote! {
            pub(crate) fn #fn_name(w: &#widget_type) -> PropBindings {
                vec![#(#event_items,)* #(#prop_items,)*]
            }
        }
    } else {
        quote! {
            pub(crate) fn #fn_name(w: &#widget_type) -> PropBindings {
                let mut out = Vec::with_capacity(#cap);
                #(#event_stmts)*
                #(#prop_stmts)*
                out
            }
        }
    }
}

fn gen_prop_binding(p: &PropDecl) -> TokenStream {
    let prop = ident(p.prop());
    let field = ident(&p.field);

    if p.is_optional() {
        // Optional — field is Option<T>, emit when Some.
        if p.is_enum_as_i32() {
            quote! {
                if let Some(v) = w.#field {
                    out.push(Binding::Prop(Prop::#prop, PropValue::I32(v.0)));
                }
            }
        } else {
            let vv = ident(p.value());
            if p.copy_value {
                quote! {
                    if let Some(v) = w.#field {
                        out.push(Binding::Prop(Prop::#prop, PropValue::#vv(v)));
                    }
                }
            } else {
                quote! {
                    if let Some(v) = &w.#field {
                        out.push(Binding::Prop(Prop::#prop, PropValue::#vv(v.clone())));
                    }
                }
            }
        }
    } else {
        // Always emit — field is T, never skipped.
        let value_expr = always_value_expr(p.value(), &field, p.copy_value, p.is_enum_as_i32());
        quote! {
            out.push(Binding::Prop(Prop::#prop, #value_expr));
        }
    }
}

/// Generate a `Binding::Prop(...)` expression for use inside `vec![...]`.
fn gen_prop_item(p: &PropDecl) -> TokenStream {
    let prop = ident(p.prop());
    let field = ident(&p.field);
    let value_expr = always_value_expr(p.value(), &field, p.copy_value, p.is_enum_as_i32());
    quote! { Binding::Prop(Prop::#prop, #value_expr) }
}

/// Generate a `Binding::Event(...)` expression for use inside `vec![...]`.
fn gen_event_item(e: &EventDecl) -> TokenStream {
    let event = ident(&e.event());
    let handler_variant = ident(e.value());
    let field = ident(&e.field);
    quote! {
        Binding::Event(
            Event::#event,
            w.#field.as_ref().map(|cb| EventHandler::#handler_variant(cb.clone())),
        )
    }
}

fn gen_event_binding(e: &EventDecl) -> TokenStream {
    let event = ident(&e.event());
    let handler_variant = ident(e.value());
    let field = ident(&e.field);

    quote! {
        out.push(Binding::Event(
            Event::#event,
            w.#field.as_ref().map(|cb| EventHandler::#handler_variant(cb.clone())),
        ));
    }
}

/// Build the `PropValue::Variant(w.field)` expression for an always-emit prop.
fn always_value_expr(
    value_variant: &str,
    field: &proc_macro2::Ident,
    copy: bool,
    enum_as_i32: bool,
) -> TokenStream {
    // Enum properties use `.0` to access the inner i32 of the WinRT struct type.
    if enum_as_i32 {
        return quote! { PropValue::I32(w.#field.0) };
    }
    let variant = ident(value_variant);
    if copy {
        quote! { PropValue::#variant(w.#field) }
    } else {
        quote! { PropValue::#variant(w.#field.clone()) }
    }
}
