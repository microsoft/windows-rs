/// Generates `set_prop` dispatch arms from control declarations.
///
/// For props with `method = "put_Foo"`, the interface is resolved automatically
/// from the winmd metadata.  For props with `setter_fn = "..."`, the named
/// custom function is called instead.
use proc_macro2::TokenStream;
use quote::quote;

use crate::helpers::*;
use crate::metadata::MetadataResolver;
use crate::schema::*;

pub fn generate(controls: &[Control], resolver: &MetadataResolver) -> String {
    let mut all_arms = Vec::new();

    for ctrl in controls {
        let arms = generate_control_arms(ctrl, resolver);
        all_arms.extend(arms);
    }

    format_generated(quote! {
        use super::*;
        use super::convert::string_as_textblock;
        use crate::bindings as Xaml;

        /// Try to handle a `set_prop` call via generated dispatch.
        /// Returns `Ok(true)` if handled, `Ok(false)` to fall through.
        pub(crate) fn dispatch(
            handle: &Handle,
            prop: Prop,
            value: &PropValue,
        ) -> windows_core::Result<bool> {
            match (prop, value, handle) {
                #(#all_arms)*
                _ => return Ok(false),
            }
            Ok(true)
        }
    })
}

fn generate_control_arms(ctrl: &Control, resolver: &MetadataResolver) -> Vec<TokenStream> {
    let handle = ident(ctrl.handle());
    let mut arms = Vec::new();

    for p in &ctrl.prop {
        arms.extend(gen_prop_arms(&handle, ctrl.handle(), p, resolver));
    }
    arms
}

fn gen_prop_arms(
    handle: &proc_macro2::Ident,
    handle_name: &str,
    p: &PropDecl,
    resolver: &MetadataResolver,
) -> Vec<TokenStream> {
    let prop = ident(&p.prop());
    let value_variant = ident(p.value());
    let mut arms = Vec::new();

    match p.setter() {
        SetterKind::Custom => {
            // Custom setter — skip; stays in hand-written mod.rs for now
            return arms;
        }
        SetterKind::Method { method } => {
            let iface_ref = resolver.resolve(handle_name, method).unwrap_or_else(|| {
                panic!(
                    "metadata: cannot resolve {handle_name}.{method} — \
                     mark as setter_fn = \"...\" or check method name"
                )
            });
            let iface = ident(iface_ref.short_name());
            let method_ident = ident(method);
            let arg = arg_tokens(p.value());

            arms.push(quote! {
                (Prop::#prop, PropValue::#value_variant(v), Handle::#handle(h)) => {
                    h.cast::<Xaml::#iface>()?.#method_ident(#arg)?;
                }
            });
        }
        SetterKind::MethodOptional { method } => {
            let iface_ref = resolver.resolve(handle_name, method).unwrap_or_else(|| {
                panic!(
                    "metadata: cannot resolve {handle_name}.{method} (optional) — check method name"
                )
            });
            let iface = ident(iface_ref.short_name());
            let method_ident = ident(method);
            let arg = arg_tokens(p.value());

            // Set arm: wrap value in Some()
            arms.push(quote! {
                (Prop::#prop, PropValue::#value_variant(v), Handle::#handle(h)) => {
                    h.cast::<Xaml::#iface>()?.#method_ident(Some(#arg))?;
                }
            });
            // Unset arm: pass None
            arms.push(quote! {
                (Prop::#prop, PropValue::Unset, Handle::#handle(h)) => {
                    h.cast::<Xaml::#iface>()?.#method_ident(None)?;
                }
            });
            return arms; // Already handled unset
        }
        SetterKind::MethodIReference { method } => {
            let iface_ref = resolver.resolve(handle_name, method).unwrap_or_else(|| {
                panic!(
                    "metadata: cannot resolve {handle_name}.{method} (ireference) — check method name"
                )
            });
            let iface = ident(iface_ref.short_name());
            let method_ident = ident(method);

            // Set arm: wrap in IReference
            arms.push(quote! {
                (Prop::#prop, PropValue::Str(v), Handle::#handle(h)) => {
                    let insp = windows_reference::IReference::from(v.as_str());
                    h.cast::<Xaml::#iface>()?.#method_ident(&insp)?;
                }
            });
            // Unset arm: pass None
            arms.push(quote! {
                (Prop::#prop, PropValue::Unset, Handle::#handle(h)) => {
                    h.cast::<Xaml::#iface>()?.#method_ident(None)?;
                }
            });
            return arms; // Already handled unset
        }
        SetterKind::MethodTextblock { method } => {
            let iface_ref = resolver.resolve(handle_name, method).unwrap_or_else(|| {
                panic!(
                    "metadata: cannot resolve {handle_name}.{method} (textblock) — check method name"
                )
            });
            let iface = ident(iface_ref.short_name());
            let method_ident = ident(method);

            // Set arm: wrap string in TextBlock
            arms.push(quote! {
                (Prop::#prop, PropValue::Str(v), Handle::#handle(h)) => {
                    let tb = string_as_textblock(v.as_str())?;
                    h.cast::<Xaml::#iface>()?.#method_ident(&tb)?;
                }
            });
            // Unset arm: pass None
            arms.push(quote! {
                (Prop::#prop, PropValue::Unset, Handle::#handle(h)) => {
                    h.cast::<Xaml::#iface>()?.#method_ident(None)?;
                }
            });
            return arms; // Already handled unset
        }
        SetterKind::MethodEnumMap { setter } => {
            let iface_ref = resolver
                .resolve(handle_name, setter.method())
                .unwrap_or_else(|| {
                    panic!(
                        "metadata: cannot resolve {handle_name}.{} (enum_map)",
                        setter.method()
                    )
                });
            let iface = ident(iface_ref.short_name());
            let method_ident = ident(setter.method());
            let rust_type = ident(setter.rust_type.as_deref().unwrap_or(p.value()));
            let winui_type = ident(&setter.winui_type);

            // Build the match arms for each variant mapping
            let variant_arms: Vec<TokenStream> = setter
                .variants
                .iter()
                .map(|[rust_v, winui_v]| {
                    let rv = ident(rust_v);
                    let wv = ident(winui_v);
                    quote! { #rust_type::#rv => Xaml::#winui_type::#wv, }
                })
                .collect();

            arms.push(quote! {
                (Prop::#prop, PropValue::#value_variant(v), Handle::#handle(h)) => {
                    let mapped = match v { #(#variant_arms)* };
                    h.cast::<Xaml::#iface>()?.#method_ident(mapped)?;
                }
            });
            return arms; // No auto-unset for enum maps
        }
    }

    // Unset arm — only emit for method-based props with default unset
    if let Some(unset) = &p.unset {
        match unset {
            UnsetPolicy::Custom => {}
            UnsetPolicy::Default { .. } if p.method.is_some() => {
                arms.push(gen_unset_arm(
                    &prop,
                    handle,
                    handle_name,
                    p,
                    unset,
                    resolver,
                ));
            }
            _ => {
                // setter_fn with default unset — can't auto-generate safely
            }
        }
    }

    arms
}

fn gen_unset_arm(
    prop: &proc_macro2::Ident,
    handle: &proc_macro2::Ident,
    handle_name: &str,
    p: &PropDecl,
    unset: &UnsetPolicy,
    resolver: &MetadataResolver,
) -> TokenStream {
    match unset {
        UnsetPolicy::Custom => {
            // Should not reach here — caller skips custom unset.
            unreachable!("gen_unset_arm called with custom unset policy")
        }
        UnsetPolicy::Default { default } => {
            let default_expr: TokenStream = default.parse().unwrap();
            // If the prop has a method, use the same interface for the unset arm.
            if let Some(method) = &p.method {
                let iface_ref = resolver.resolve(handle_name, method).unwrap_or_else(|| {
                    panic!("metadata: cannot resolve {handle_name}.{method} for unset")
                });
                let iface = ident(iface_ref.short_name());
                let method_ident = ident(method);
                quote! {
                    (Prop::#prop, PropValue::Unset, Handle::#handle(h)) => {
                        h.cast::<Xaml::#iface>()?.#method_ident(#default_expr)?;
                    }
                }
            } else {
                // Custom setter with default unset — just emit the default.
                quote! {
                    (Prop::#prop, PropValue::Unset, Handle::#handle(_h)) => {
                        let _ = #default_expr;
                    }
                }
            }
        }
    }
}

fn arg_tokens(value_variant: &str) -> TokenStream {
    match arg_expr_for_value(value_variant) {
        "str_ref" => quote! { v.as_str() },
        "copy" => quote! { *v },
        _ => quote! { v.clone() },
    }
}
