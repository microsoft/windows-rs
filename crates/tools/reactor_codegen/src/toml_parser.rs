/// V2 TOML schema: metadata-driven, concise format.
///
/// Format:
/// ```toml
/// ["Microsoft.UI.Xaml.Controls.Button"]
/// Content = { emit = "optional", wrap = "ireference" }
/// IsEnabled = { emit = "when_false" }
/// Click = {}
/// ```
///
/// Each section key is a full WinUI type name.
/// Each member key is a WinUI property or event name (from metadata).
/// The tool looks up `put_{Name}` or `add_{Name}` in metadata to determine
/// whether it's a property or event, and infers method, value type, invoke
/// pattern, etc.
///
/// Only overrides need to be specified — everything else comes from metadata.
use serde::Deserialize;

use crate::helpers::to_snake_case;
use crate::metadata::MetadataResolver;
use crate::schema::{
    BoolEnumSetter, Control, Emit, EnumMapSetter, EventDecl, PropDecl, UnsetPolicy,
};

/// Reserved key for control-level options (widget name, kind, etc.)
const CONTROL_KEY: &str = "_control";

/// Control-level overrides.
#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
struct ControlMeta {
    /// Rust widget struct name override (e.g. "DatePickerWidget").
    /// Default: short class name (e.g. "DatePicker").
    #[serde(default)]
    widget: Option<String>,
    /// Handle/Kind override. Default: short class name.
    #[serde(default)]
    handle: Option<String>,
    /// Namespace override. Default: extracted from full type name.
    #[serde(default)]
    namespace: Option<String>,
}

/// Override hints for a property or event member.
#[derive(Deserialize, Default, Debug)]
#[serde(deny_unknown_fields)]
struct MemberOverride {
    // ── Shared ──
    /// Emission pattern override.
    #[serde(default)]
    emit: Option<Emit>,
    /// Unset/reset policy.
    #[serde(default)]
    unset: Option<UnsetPolicy>,
    /// Explicit PropValue/EventHandler variant name.
    #[serde(default)]
    value: Option<String>,
    /// Explicit Rust field name (overrides snake_case of key).
    #[serde(default)]
    field: Option<String>,

    // ── Property-specific ──
    /// Custom setter function.
    #[serde(default)]
    setter_fn: Option<bool>,
    /// IReference wrapping (e.g. "ireference").
    #[serde(default)]
    wrap: Option<String>,
    /// Enum map setter.
    #[serde(default)]
    enum_map: Option<EnumMapSetter>,
    /// Bool-to-enum setter.
    #[serde(default)]
    bool_enum: Option<BoolEnumSetter>,
    /// Explicit Prop enum variant name (when it must differ from metadata name).
    #[serde(default)]
    prop: Option<String>,

    // ── Event-specific ──
    /// Custom event attachment function.
    #[serde(default)]
    attach_fn: Option<bool>,
    /// Explicit event handler type.
    #[serde(default)]
    handler: Option<String>,
    /// Explicit invoke pattern.
    #[serde(default)]
    invoke: Option<String>,
    /// Getter method on event args.
    #[serde(default)]
    getter: Option<String>,
    /// Second add method for bool_dual events.
    #[serde(default)]
    add_method_false: Option<String>,
    /// Explicit Event enum variant name (when it must differ from metadata name).
    #[serde(default)]
    event: Option<String>,
}

/// Parse the v2 TOML and resolve against metadata to produce `Vec<Control>`.
pub fn parse(toml_content: &str, resolver: &MetadataResolver) -> Vec<Control> {
    let table: toml::Table =
        toml::from_str(toml_content).unwrap_or_else(|e| panic!("Failed to parse TOML: {e}"));

    let line_index = LineIndex::new(toml_content);
    let mut controls = Vec::new();

    for (type_name, section) in &table {
        let section = section
            .as_table()
            .unwrap_or_else(|| panic!("expected table for '{type_name}'"));

        let (namespace, short_name) = split_type_name(type_name);

        let meta: ControlMeta = section
            .get(CONTROL_KEY)
            .map(|v| {
                v.clone().try_into().unwrap_or_else(|e| {
                    let line = line_index.find(type_name, CONTROL_KEY);
                    panic!("{line}failed to parse {CONTROL_KEY} for '{type_name}': {e}")
                })
            })
            .unwrap_or_default();

        let widget_name = meta.widget.as_deref().unwrap_or(short_name);
        let handle = meta.handle.as_deref().unwrap_or(short_name);
        let ns = meta.namespace.as_deref().unwrap_or(namespace);

        let mut props = Vec::new();
        let mut events = Vec::new();

        for (member_name, member_value) in section {
            if member_name == CONTROL_KEY {
                continue;
            }
            let line = line_index.find(type_name, member_name);
            assert!(
                !member_name.starts_with('_'),
                "{line}reserved key '{member_name}' in '{type_name}' — only '{CONTROL_KEY}' is supported"
            );

            let overrides: MemberOverride = member_value.clone().try_into().unwrap_or_else(|e| {
                panic!("{line}failed to parse member '{member_name}' in '{type_name}': {e}")
            });

            let is_prop = resolver.has_method(handle, &format!("put_{member_name}"))
                || resolver.has_method(handle, &format!("get_{member_name}"));
            let is_event = resolver.has_method(handle, &format!("add_{member_name}"));

            assert!(
                !(is_prop && is_event),
                "{line}'{member_name}' on '{type_name}' is both a property and an event in metadata — add explicit kind"
            );
            if !is_prop && !is_event {
                if overrides.setter_fn.is_some() {
                    let prop = build_prop(member_name, handle, &overrides, resolver, &line);
                    props.push(prop);
                    continue;
                }
                if overrides.attach_fn.is_some() {
                    let event = build_event(member_name, handle, &overrides, resolver);
                    events.push(event);
                    continue;
                }
                panic!(
                    "{line}'{member_name}' on '{type_name}' not found in metadata (no put_{member_name} or add_{member_name})"
                );
            }

            if is_prop {
                let prop = build_prop(member_name, handle, &overrides, resolver, &line);
                props.push(prop);
            } else {
                let event = build_event(member_name, handle, &overrides, resolver);
                events.push(event);
            }
        }

        // Build the Control, preserving existing naming conventions.
        let control = Control {
            name: widget_name.to_string(),
            kind: if widget_name != short_name {
                Some(short_name.to_string())
            } else {
                None
            },
            handle: if handle != short_name {
                Some(handle.to_string())
            } else {
                None
            },
            namespace: if ns != "Microsoft.UI.Xaml.Controls" {
                Some(ns.to_string())
            } else {
                None
            },
            prop: props,
            event: events,
        };

        controls.push(control);
    }

    controls
}

/// Build a PropDecl from metadata + overrides.
fn build_prop(
    member_name: &str,
    handle: &str,
    overrides: &MemberOverride,
    resolver: &MetadataResolver,
    line: &str,
) -> PropDecl {
    let field = overrides
        .field
        .clone()
        .unwrap_or_else(|| to_snake_case(member_name));
    let prop_variant = overrides.prop.clone();

    let emit = overrides
        .emit
        .clone()
        .unwrap_or_else(|| panic!("{line}'{member_name}' on '{handle}' needs explicit 'emit'"));

    let setter_fn = overrides.setter_fn.map(|_| "__custom__".to_string());
    let wrap = overrides.wrap.clone();

    let method_name = format!("put_{member_name}");
    let has_method = resolver.has_method(handle, &method_name);

    let value = if let Some(v) = &overrides.value {
        Some(v.clone())
    } else if overrides.bool_enum.is_some() {
        // bool_enum always maps to PropValue::Bool
        Some("Bool".to_string())
    } else if has_method {
        resolver.infer_value_type(handle, &method_name)
    } else {
        None
    };

    // Check if metadata says this is an enum type (for auto-inference).
    let is_metadata_enum = overrides.enum_map.is_none()
        && overrides.bool_enum.is_none()
        && setter_fn.is_none()
        && has_method
        && resolver.enum_info(handle, &method_name).is_some();

    let (method, method_optional, method_ireference, method_textblock) = if setter_fn.is_some()
        || overrides.enum_map.is_some()
        || overrides.bool_enum.is_some()
        || is_metadata_enum
    {
        (None, None, None, None)
    } else if has_method {
        classify_setter(handle, &method_name, wrap.as_deref(), resolver)
    } else {
        (None, None, None, None)
    };

    // Pre-set method on bool_enum/enum_map from the metadata name (TOML key),
    // since resolve_defaults infers from field/prop which may differ.
    let method_bool_enum = overrides.bool_enum.clone().map(|mut s| {
        if s.method.is_none() && has_method {
            s.method = Some(method_name.clone());
        }
        s
    });
    let method_enum_map = if let Some(mut s) = overrides.enum_map.clone() {
        // Explicit enum_map in TOML — just fill in method if missing.
        if s.method.is_none() && has_method {
            s.method = Some(method_name);
        }
        Some(s)
    } else if overrides.bool_enum.is_none() && setter_fn.is_none() && has_method {
        // No explicit enum_map — try to infer from metadata.
        if let Some((enum_name, variants)) = resolver.enum_info(handle, &method_name) {
            Some(EnumMapSetter {
                method: Some(method_name.clone()),
                rust_type: None,
                winui_type: enum_name.to_string(),
                variants: variants.iter().map(|v| [v.clone(), v.clone()]).collect(),
            })
        } else {
            None
        }
    } else {
        None
    };

    PropDecl {
        field,
        meta_name: member_name.to_string(),
        prop: prop_variant,
        value,
        emit,
        wrap,
        method,
        method_optional,
        method_ireference,
        method_textblock,
        method_bool_enum,
        method_enum_map,
        setter_fn,
        unset: overrides.unset.clone(),
    }
}

/// Classify a property setter method into the correct setter kind.
fn classify_setter(
    handle: &str,
    method_name: &str,
    wrap: Option<&str>,
    resolver: &MetadataResolver,
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    use crate::metadata::ParamClass;

    let param_class = resolver.classify_param(handle, method_name);

    match param_class {
        Some(ParamClass::NullableBool) => (None, Some(method_name.to_string()), None, None),
        Some(ParamClass::IInspectable) => {
            if wrap == Some("ireference") {
                (None, None, Some(method_name.to_string()), None)
            } else {
                (None, None, None, Some(method_name.to_string()))
            }
        }
        _ => (Some(method_name.to_string()), None, None, None),
    }
}

/// Build an EventDecl from metadata + overrides.
fn build_event(
    member_name: &str,
    _handle: &str,
    overrides: &MemberOverride,
    _resolver: &MetadataResolver,
) -> EventDecl {
    let field = overrides
        .field
        .clone()
        .unwrap_or_else(|| format!("on_{}", to_snake_case(member_name)));
    let event_variant = overrides.event.clone();
    let handler = overrides.handler.clone();
    let attach_fn = overrides.attach_fn.map(|_| "__custom__".to_string());
    let add_method = if attach_fn.is_some() {
        None
    } else {
        Some(format!("add_{member_name}"))
    };

    EventDecl {
        field,
        meta_name: member_name.to_string(),
        event: event_variant,
        handler,
        attach_fn,
        add_method,
        invoke: overrides.invoke.clone(),
        getter: overrides.getter.clone(),
        add_method_false: overrides.add_method_false.clone(),
    }
}

/// Split "Microsoft.UI.Xaml.Controls.Button" into ("Microsoft.UI.Xaml.Controls", "Button").
fn split_type_name(full_name: &str) -> (&str, &str) {
    full_name.rsplit_once('.').unwrap_or_else(|| {
        panic!(
            "type name '{full_name}' must contain a dot (e.g. 'Microsoft.UI.Xaml.Controls.Button')"
        )
    })
}

/// Convert PascalCase to snake_case.
/// Maps TOML keys to source line numbers for clickable error messages.
struct LineIndex {
    /// (section_header, member_key) → 1-based line number
    entries: std::collections::HashMap<(String, String), usize>,
}

impl LineIndex {
    fn new(source: &str) -> Self {
        let mut entries = std::collections::HashMap::new();
        let mut current_section = String::new();
        for (i, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            if let Some(inner) = trimmed.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
                current_section = inner.trim_matches('"').to_string();
            } else if let Some(key) = trimmed.split('=').next() {
                let key = key.trim();
                if !key.is_empty() && !current_section.is_empty() {
                    entries.insert((current_section.clone(), key.to_string()), i + 1);
                }
            }
        }
        Self { entries }
    }

    /// Returns a prefix like "reactor_widgets.toml:42: " or "" if not found.
    fn find(&self, section: &str, key: &str) -> String {
        self.entries
            .get(&(section.to_string(), key.to_string()))
            .map(|line| format!("reactor_widgets.toml:{line}: "))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn resolver() -> MetadataResolver {
        MetadataResolver::load(Path::new("../../../winmd"))
    }

    // ── Valid TOML parsing ─────────────────────────────────────────

    #[test]
    fn parse_simple_prop() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.TextBlock"]
Text = { emit = "always" }
"#;
        let controls = parse(toml, &resolver());
        assert_eq!(controls.len(), 1);
        assert_eq!(controls[0].name, "TextBlock");
        assert_eq!(controls[0].prop.len(), 1);
        assert_eq!(controls[0].prop[0].field, "text");
        assert_eq!(controls[0].prop[0].meta_name, "Text");
    }

    #[test]
    fn parse_simple_event() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.Button"]
Click = {}
"#;
        let controls = parse(toml, &resolver());
        assert_eq!(controls[0].event.len(), 1);
        assert_eq!(controls[0].event[0].field, "on_click");
        assert_eq!(controls[0].event[0].meta_name, "Click");
    }

    #[test]
    fn parse_field_override() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.TextBlock"]
Text = { field = "content", emit = "always" }
"#;
        let controls = parse(toml, &resolver());
        assert_eq!(controls[0].prop[0].field, "content");
    }

    #[test]
    fn parse_event_handler_override() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.InfoBar"]
Closed = { handler = "Click" }
"#;
        let controls = parse(toml, &resolver());
        assert_eq!(controls[0].event[0].handler(), "Click");
    }

    #[test]
    fn parse_setter_fn_prop() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.Button"]
Content = { emit = "always", setter_fn = true, value = "Str" }
"#;
        let controls = parse(toml, &resolver());
        assert!(controls[0].prop[0].setter_fn.is_some());
    }

    #[test]
    fn parse_empty_section() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.Canvas"]
"#;
        let controls = parse(toml, &resolver());
        assert_eq!(controls.len(), 1);
        assert_eq!(controls[0].name, "Canvas");
        assert!(controls[0].prop.is_empty());
        assert!(controls[0].event.is_empty());
    }

    #[test]
    fn parse_namespace_extraction() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.Primitives.RepeatButton"]
Click = {}
"#;
        let controls = parse(toml, &resolver());
        assert_eq!(controls[0].name, "RepeatButton");
        assert_eq!(
            controls[0]
                .namespace
                .as_deref()
                .unwrap_or("Microsoft.UI.Xaml.Controls"),
            "Microsoft.UI.Xaml.Controls.Primitives"
        );
    }

    #[test]
    fn parse_value_inferred_from_metadata() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.ProgressBar"]
Value = { emit = "always" }
"#;
        let controls = parse(toml, &resolver());
        // put_Value on RangeBase takes f64 — value should be inferred
        assert_eq!(controls[0].prop[0].value.as_deref(), Some("F64"));
    }

    // ── Error messages ─────────────────────────────────────────────

    #[test]
    #[should_panic(expected = "reactor_widgets.toml:3: ")]
    fn error_includes_line_number() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.Button"]
NonExistentProperty = { emit = "always" }
"#;
        parse(toml, &resolver());
    }

    #[test]
    #[should_panic(expected = "not found in metadata")]
    fn error_unknown_member() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.Button"]
NonExistentProperty = { emit = "always" }
"#;
        parse(toml, &resolver());
    }

    #[test]
    #[should_panic(expected = "needs explicit 'emit'")]
    fn error_missing_emit() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.TextBlock"]
Text = {}
"#;
        parse(toml, &resolver());
    }

    #[test]
    #[should_panic(expected = "reserved key")]
    fn error_unknown_underscore_key() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.Button"]
_bogus = { foo = true }
"#;
        parse(toml, &resolver());
    }

    #[test]
    #[should_panic(expected = "must contain a dot")]
    fn error_invalid_type_name() {
        let toml = r#"
["Button"]
Click = {}
"#;
        parse(toml, &resolver());
    }

    #[test]
    #[should_panic(expected = "unknown field")]
    fn error_unknown_override_field() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.Button"]
Click = { bogus_field = true }
"#;
        parse(toml, &resolver());
    }

    #[test]
    #[should_panic(expected = "Failed to parse")]
    fn error_malformed_toml() {
        let toml = r#"
["Microsoft.UI.Xaml.Controls.Button"
Click = {}
"#;
        parse(toml, &resolver());
    }

    #[test]
    fn parse_full_production_toml() {
        // Parse the actual production TOML to ensure it's valid
        let toml_path = "../../../crates/tools/reactor_codegen/reactor_widgets.toml";
        let toml = std::fs::read_to_string(toml_path).expect("production TOML should exist");
        let controls = parse(&toml, &resolver());
        assert!(
            controls.len() >= 40,
            "expected at least 40 controls, got {}",
            controls.len()
        );
    }
}
