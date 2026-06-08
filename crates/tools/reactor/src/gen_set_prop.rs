/// Generates `set_prop` dispatch arms from control declarations.
///
/// Arms that share the same body across multiple Handle variants are merged:
/// - Into a wildcard `_` arm when no custom handler exists for that Prop
/// - Into an OR-pattern arm listing specific handles when custom handlers exist
///
/// This eliminates duplicate arms for base-interface properties like
/// `put_IsEnabled` (IControl) and `put_Content` (IContentControl).
use std::collections::{BTreeMap, HashSet};

use proc_macro2::TokenStream;
use quote::quote;

use crate::helpers::*;
use crate::metadata::MetadataResolver;
use crate::schema::*;

// ── Intermediate representation ──────────────────────────────────────

/// A match arm descriptor, collected before rendering.
struct ArmDesc {
    /// `Prop` enum variant name (e.g. `"IsEnabled"`).
    prop: String,
    /// `PropValue` variant name (e.g. `"Bool"`) or `"Unset"`.
    value_pat: String,
    /// Handle variant name (e.g. `"Button"`).
    handle: String,
    /// Grouping key — arms with the same key produce identical bodies.
    body_key: String,
    /// How to render the arm body.
    body: Body,
}

/// Describes the body of a set_prop match arm.
enum Body {
    /// `{cast}.{method}({arg})?`
    Cast {
        iface: String,
        method: String,
        value_variant: String,
    },
    /// `{cast}.{method}(Some({arg}))?`
    CastOptional {
        iface: String,
        method: String,
        value_variant: String,
    },
    /// `{cast}.{method}(None)?`
    CastNone { iface: String, method: String },
    /// `let insp = IReference::from(v.as_str()); {cast}.{method}(&insp)?`
    IReference { iface: String, method: String },
    /// `let tb = string_as_textblock(v.as_str())?; {cast}.{method}(&tb)?`
    Textblock { iface: String, method: String },
    /// `{cast}.{method}(Xaml::{winui_type}(*v))?`
    EnumMap {
        iface: String,
        method: String,
        winui_type: String,
    },
    /// `{cast}.{method}({default})?`
    UnsetDefault {
        iface: String,
        method: String,
        default_expr: String,
    },
}

impl Body {
    /// Grouping key — two arms with the same key have identical bodies.
    fn key(&self) -> String {
        match self {
            Self::Cast {
                iface,
                method,
                value_variant,
            } => format!(
                "cast.{iface}.{method}.{}",
                arg_expr_for_value(value_variant)
            ),
            Self::CastOptional {
                iface,
                method,
                value_variant,
            } => format!("opt.{iface}.{method}.{}", arg_expr_for_value(value_variant)),
            Self::CastNone { iface, method } => format!("none.{iface}.{method}"),
            Self::IReference { iface, method } => format!("iref.{iface}.{method}"),
            Self::Textblock { iface, method } => format!("tb.{iface}.{method}"),
            Self::EnumMap {
                iface,
                method,
                winui_type,
            } => {
                format!("enum.{iface}.{method}.{winui_type}")
            }
            Self::UnsetDefault {
                iface,
                method,
                default_expr,
            } => format!("unset.{iface}.{method}.{default_expr}"),
        }
    }
}

// ── Public entry point ───────────────────────────────────────────────

pub fn generate(controls: &[Control], resolver: &MetadataResolver) -> String {
    // Props with custom handlers (setter_fn) can't use wildcard arms because
    // the hand-written dispatch in mod.rs must remain reachable for those handles.
    let custom_props: HashSet<String> = controls
        .iter()
        .flat_map(|c| c.prop.iter())
        .filter(|p| matches!(p.setter(), SetterKind::Custom))
        .map(|p| p.prop())
        .collect();

    // Phase 1: collect all arm descriptors.
    let mut all_descs: Vec<ArmDesc> = Vec::new();
    for ctrl in controls {
        for p in &ctrl.prop {
            all_descs.extend(collect_prop_arms(ctrl.handle(), p, resolver));
        }
    }

    // Phase 2: group by (prop, value_pat), then sub-group by body_key.
    let mut groups: BTreeMap<(String, String), Vec<ArmDesc>> = BTreeMap::new();
    for desc in all_descs {
        groups
            .entry((desc.prop.clone(), desc.value_pat.clone()))
            .or_default()
            .push(desc);
    }

    // Phase 3: merge duplicate sub-groups.
    let mut specific_arms = Vec::new();
    let mut wildcard_arms = Vec::new();

    for ((prop_name, _), descs) in &groups {
        let mut by_body: BTreeMap<&str, Vec<&ArmDesc>> = BTreeMap::new();
        for d in descs {
            by_body.entry(&d.body_key).or_default().push(d);
        }

        let has_custom = custom_props.contains(prop_name);

        // Find the body_key with the most arms (>1) — candidate for wildcard.
        let merge_key = by_body
            .iter()
            .filter(|(_, v)| v.len() > 1)
            .max_by_key(|(_, v)| v.len())
            .map(|(k, _)| *k);

        for (key, sub) in &by_body {
            if sub.len() > 1 {
                if Some(*key) == merge_key && !has_custom {
                    // Largest duplicate group + no custom conflict → wildcard
                    wildcard_arms.push(render_arm(sub[0], true));
                } else {
                    // Custom conflict or non-largest group → OR-pattern
                    specific_arms.push(render_or_pattern(sub));
                }
            } else {
                // Singleton — specific arm
                specific_arms.push(render_arm(sub[0], false));
            }
        }
    }

    // Specific/OR-pattern arms before wildcards (specifics take priority).
    let mut all_arms = specific_arms;
    all_arms.extend(wildcard_arms);

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

// ── Arm collection ───────────────────────────────────────────────────

fn collect_prop_arms(handle: &str, p: &PropDecl, resolver: &MetadataResolver) -> Vec<ArmDesc> {
    let prop = p.prop();
    let mut descs = Vec::new();

    match p.setter() {
        SetterKind::Custom => return descs,

        SetterKind::Method { method } => {
            let iface = resolve_iface(resolver, handle, method);
            descs.push(make_arm(
                &prop,
                p.value(),
                handle,
                Body::Cast {
                    iface,
                    method: method.to_string(),
                    value_variant: p.value().to_string(),
                },
            ));
        }

        SetterKind::MethodOptional { method } => {
            let iface = resolve_iface(resolver, handle, method);
            descs.push(make_arm(
                &prop,
                p.value(),
                handle,
                Body::CastOptional {
                    iface: iface.clone(),
                    method: method.to_string(),
                    value_variant: p.value().to_string(),
                },
            ));
            descs.push(make_arm(
                &prop,
                "Unset",
                handle,
                Body::CastNone {
                    iface,
                    method: method.to_string(),
                },
            ));
            return descs;
        }

        SetterKind::MethodIReference { method } => {
            let iface = resolve_iface(resolver, handle, method);
            descs.push(make_arm(
                &prop,
                "Str",
                handle,
                Body::IReference {
                    iface: iface.clone(),
                    method: method.to_string(),
                },
            ));
            descs.push(make_arm(
                &prop,
                "Unset",
                handle,
                Body::CastNone {
                    iface,
                    method: method.to_string(),
                },
            ));
            return descs;
        }

        SetterKind::MethodTextblock { method } => {
            let iface = resolve_iface(resolver, handle, method);
            descs.push(make_arm(
                &prop,
                "Str",
                handle,
                Body::Textblock {
                    iface: iface.clone(),
                    method: method.to_string(),
                },
            ));
            descs.push(make_arm(
                &prop,
                "Unset",
                handle,
                Body::CastNone {
                    iface,
                    method: method.to_string(),
                },
            ));
            return descs;
        }

        SetterKind::MethodEnumMap { setter } => {
            let iface = resolve_iface(resolver, handle, setter.method());
            descs.push(make_arm(
                &prop,
                "I32",
                handle,
                Body::EnumMap {
                    iface,
                    method: setter.method().to_string(),
                    winui_type: setter.winui_type.clone(),
                },
            ));
            return descs;
        }
    }

    // Unset arm — only for Method setters with a default reset value.
    if let Some(UnsetPolicy::Default { default }) = &p.unset
        && let Some(method) = &p.method
    {
        let iface = resolve_iface(resolver, handle, method);
        descs.push(make_arm(
            &prop,
            "Unset",
            handle,
            Body::UnsetDefault {
                iface,
                method: method.clone(),
                default_expr: default.clone(),
            },
        ));
    }

    descs
}

fn make_arm(prop: &str, value_pat: &str, handle: &str, body: Body) -> ArmDesc {
    ArmDesc {
        prop: prop.to_string(),
        value_pat: value_pat.to_string(),
        handle: handle.to_string(),
        body_key: body.key(),
        body,
    }
}

fn resolve_iface(resolver: &MetadataResolver, handle: &str, method: &str) -> String {
    resolver
        .resolve(handle, method)
        .unwrap_or_else(|| {
            panic!(
                "metadata: cannot resolve {handle}.{method} — \
                 mark as setter_fn or check method name"
            )
        })
        .short_name()
        .to_string()
}

// ── Rendering ────────────────────────────────────────────────────────

/// Render a single arm — either a specific `Handle::X(h)` or a wildcard `_`.
fn render_arm(desc: &ArmDesc, wildcard: bool) -> TokenStream {
    let prop_id = ident(&desc.prop);
    let handle_name = if wildcard {
        None
    } else {
        Some(desc.handle.as_str())
    };
    let body = render_body(&desc.body, wildcard, handle_name);

    match (desc.value_pat.as_str(), wildcard) {
        ("Unset", true) => quote! {
            (Prop::#prop_id, PropValue::Unset, _) => { #body }
        },
        ("Unset", false) => {
            let h = ident(&desc.handle);
            quote! {
                (Prop::#prop_id, PropValue::Unset, Handle::#h(h)) => { #body }
            }
        }
        (vp, true) => {
            let v = ident(vp);
            quote! {
                (Prop::#prop_id, PropValue::#v(v), _) => { #body }
            }
        }
        (vp, false) => {
            let v = ident(vp);
            let h = ident(&desc.handle);
            quote! {
                (Prop::#prop_id, PropValue::#v(v), Handle::#h(h)) => { #body }
            }
        }
    }
}

/// Render merged arms as an OR-pattern over specific handles.
/// Uses `handle.cast_inner()` since individual inner types are not bound.
fn render_or_pattern(descs: &[&ArmDesc]) -> TokenStream {
    let prop_id = ident(&descs[0].prop);
    let body = render_body(&descs[0].body, true, None);

    let mut handles: Vec<&str> = descs.iter().map(|d| d.handle.as_str()).collect();
    handles.sort();
    let handle_pats: Vec<TokenStream> = handles
        .iter()
        .map(|h| {
            let hi = ident(h);
            quote! { Handle::#hi(_) }
        })
        .collect();

    if descs[0].value_pat == "Unset" {
        quote! {
            (Prop::#prop_id, PropValue::Unset, #(#handle_pats)|*) => { #body }
        }
    } else {
        let vp = ident(&descs[0].value_pat);
        quote! {
            (Prop::#prop_id, PropValue::#vp(v), #(#handle_pats)|*) => { #body }
        }
    }
}

/// Generate the cast expression: specific `h.cast` or merged `handle.cast_inner`.
/// When the interface is the default (deref target) for the handle, no cast is needed.
fn cast_tokens(iface: &str, wildcard: bool, handle_name: Option<&str>) -> TokenStream {
    // Default interface for class `Foo` is `IFoo` — Deref handles it.
    if !wildcard && handle_name.is_some_and(|h| iface == format!("I{h}")) {
        return quote! { h };
    }
    let i = ident(iface);
    if wildcard {
        quote! { handle.cast_inner::<Xaml::#i>()? }
    } else {
        quote! { h.cast::<Xaml::#i>()? }
    }
}

/// Render the body of a match arm.
fn render_body(body: &Body, wildcard: bool, handle_name: Option<&str>) -> TokenStream {
    match body {
        Body::Cast {
            iface,
            method,
            value_variant,
        } => {
            let cast = cast_tokens(iface, wildcard, handle_name);
            let m = ident(method);
            let arg = arg_tokens(value_variant);
            quote! { #cast.#m(#arg)?; }
        }
        Body::CastOptional {
            iface,
            method,
            value_variant,
        } => {
            let cast = cast_tokens(iface, wildcard, handle_name);
            let m = ident(method);
            let arg = arg_tokens(value_variant);
            quote! { #cast.#m(Some(#arg))?; }
        }
        Body::CastNone { iface, method } => {
            let cast = cast_tokens(iface, wildcard, handle_name);
            let m = ident(method);
            quote! { #cast.#m(None)?; }
        }
        Body::IReference { iface, method } => {
            let cast = cast_tokens(iface, wildcard, handle_name);
            let m = ident(method);
            quote! {
                let insp = windows_reference::IReference::from(v.as_str());
                #cast.#m(&insp)?;
            }
        }
        Body::Textblock { iface, method } => {
            let cast = cast_tokens(iface, wildcard, handle_name);
            let m = ident(method);
            quote! {
                let tb = string_as_textblock(v.as_str())?;
                #cast.#m(&tb)?;
            }
        }
        Body::EnumMap {
            iface,
            method,
            winui_type,
        } => {
            let cast = cast_tokens(iface, wildcard, handle_name);
            let m = ident(method);
            let wt = ident(winui_type);
            quote! {
                #cast.#m(Xaml::#wt(*v))?;
            }
        }
        Body::UnsetDefault {
            iface,
            method,
            default_expr,
        } => {
            let cast = cast_tokens(iface, wildcard, handle_name);
            let m = ident(method);
            let default: TokenStream = default_expr.parse().unwrap();
            quote! { #cast.#m(#default)?; }
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
