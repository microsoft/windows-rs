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

    let has_conditional_props = ctrl.prop.iter().any(|p| !matches!(p.emit, Emit::Always));
    let all_unconditional = !has_conditional_props;

    let prop_stmts: Vec<TokenStream> = ctrl.prop.iter().map(gen_prop_binding).collect();
    let event_stmts: Vec<TokenStream> = ctrl.event.iter().map(gen_event_binding).collect();
    let cap = ctrl.prop.len() + ctrl.event.len();

    if cap == 0 {
        quote! {
            pub(crate) fn #fn_name(_w: &#widget_type) -> PropBindings {
                Vec::new()
            }
        }
    } else if all_unconditional {
        // All items are unconditional — use vec![...] directly
        let prop_items: Vec<TokenStream> = ctrl.prop.iter().map(gen_prop_item).collect();
        let event_items: Vec<TokenStream> = ctrl.event.iter().map(gen_event_item).collect();
        quote! {
            pub(crate) fn #fn_name(w: &#widget_type) -> PropBindings {
                vec![#(#prop_items,)* #(#event_items,)*]
            }
        }
    } else {
        quote! {
            pub(crate) fn #fn_name(w: &#widget_type) -> PropBindings {
                let mut out = Vec::with_capacity(#cap);
                #(#prop_stmts)*
                #(#event_stmts)*
                out
            }
        }
    }
}

fn gen_prop_binding(p: &PropDecl) -> TokenStream {
    let prop = ident(&p.prop());
    let field = ident(&p.field);

    match &p.emit {
        Emit::Always => {
            let value_expr = always_value_expr(p.value(), &field);
            quote! {
                out.push(Binding::Prop(Prop::#prop, #value_expr));
            }
        }
        Emit::Optional => {
            let value_variant = ident(p.value());
            if is_copy_value(p.value()) {
                quote! {
                    if let Some(v) = w.#field {
                        out.push(Binding::Prop(Prop::#prop, PropValue::#value_variant(v)));
                    }
                }
            } else {
                quote! {
                    if let Some(v) = &w.#field {
                        out.push(Binding::Prop(Prop::#prop, PropValue::#value_variant(v.clone())));
                    }
                }
            }
        }
        Emit::WhenTrue => {
            let val = when_bool_value(p.value(), true);
            quote! {
                if w.#field {
                    out.push(Binding::Prop(Prop::#prop, #val));
                }
            }
        }
        Emit::WhenFalse => {
            let val = when_bool_value(p.value(), false);
            quote! {
                if !w.#field {
                    out.push(Binding::Prop(Prop::#prop, #val));
                }
            }
        }
        Emit::NonDefault(default_expr) => {
            if default_expr == "false" {
                // `w.field != false` → `w.field`
                let value_expr = always_value_expr(p.value(), &field);
                quote! {
                    if w.#field {
                        out.push(Binding::Prop(Prop::#prop, #value_expr));
                    }
                }
            } else if default_expr == "true" {
                // `w.field != true` → `!w.field`
                let value_expr = always_value_expr(p.value(), &field);
                quote! {
                    if !w.#field {
                        out.push(Binding::Prop(Prop::#prop, #value_expr));
                    }
                }
            } else {
                let default_tokens: TokenStream = default_expr.parse().unwrap();
                let value_expr = always_value_expr(p.value(), &field);
                quote! {
                    if w.#field != #default_tokens {
                        out.push(Binding::Prop(Prop::#prop, #value_expr));
                    }
                }
            }
        }
    }
}

/// Generate a `Binding::Prop(...)` expression for use inside `vec![...]`.
fn gen_prop_item(p: &PropDecl) -> TokenStream {
    let prop = ident(&p.prop());
    let field = ident(&p.field);
    let value_expr = always_value_expr(p.value(), &field);
    quote! { Binding::Prop(Prop::#prop, #value_expr) }
}

/// Generate a `Binding::Event(...)` expression for use inside `vec![...]`.
fn gen_event_item(e: &EventDecl) -> TokenStream {
    let event = ident(&e.event());
    let handler_variant = ident(&e.handler());
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
    let handler_variant = ident(&e.handler());
    let field = ident(&e.field);

    quote! {
        out.push(Binding::Event(
            Event::#event,
            w.#field.as_ref().map(|cb| EventHandler::#handler_variant(cb.clone())),
        ));
    }
}

/// Build the `PropValue::Variant(w.field)` expression for an always-emit prop.
fn always_value_expr(value_variant: &str, field: &proc_macro2::Ident) -> TokenStream {
    let variant = ident(value_variant);
    if is_copy_value(value_variant) {
        quote! { PropValue::#variant(w.#field) }
    } else {
        quote! { PropValue::#variant(w.#field.clone()) }
    }
}

/// Returns true if the PropValue variant wraps a Copy type (no .clone() needed).
fn is_copy_value(variant: &str) -> bool {
    matches!(
        variant,
        "Bool"
            | "F64"
            | "I32"
            | "U16"
            | "U32"
            | "U8"
            | "Vertical"
            | "ImageStretch"
            | "CommandBarLabelPosition"
            | "InfoBarSev"
            | "NavPaneDisplayMode"
            | "PasswordRevealMode"
            | "ScrollViewScrollBarVis"
            | "ScrollVis"
            | "TeachingTipPlacement"
            | "TreeViewSelectionMode"
            | "SymbolIcon"
            | "Thickness"
            | "HAlign"
            | "VAlign"
            | "FlyoutPlacement"
    )
}

fn when_bool_value(value_variant: &str, val: bool) -> TokenStream {
    let variant = ident(value_variant);
    quote! { PropValue::#variant(#val) }
}
