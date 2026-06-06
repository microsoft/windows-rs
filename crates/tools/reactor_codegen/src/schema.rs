/// TOML schema types for reactor widget declarations.
///
/// These types define the internal representation that the codegen
/// generators consume. They are constructed by the TOML parser in
/// `toml_parser.rs`, not deserialized directly.
use serde::Deserialize;

use crate::metadata::MetadataResolver;

/// One widget / control declaration.
pub struct Control {
    /// Widget struct name (e.g. `"TextBlock"`).
    pub name: String,
    /// `ControlKind` variant. Defaults to `name` with `Widget` suffix stripped.
    pub kind: Option<String>,
    /// `Handle` variant / WinUI class name (e.g. `"TextBlock"`).
    /// Defaults to the resolved `kind`.
    /// The metadata resolver looks up `{namespace}.{handle}`.
    pub handle: Option<String>,
    /// WinRT namespace override. Defaults to `"Microsoft.UI.Xaml.Controls"`.
    pub namespace: Option<String>,
    /// Property bindings.
    pub prop: Vec<PropDecl>,
    /// Event bindings.
    pub event: Vec<EventDecl>,
}

impl Control {
    /// Resolved `kind` — defaults to `name` with trailing "Widget" stripped.
    pub fn kind(&self) -> &str {
        self.kind
            .as_deref()
            .unwrap_or_else(|| self.name.strip_suffix("Widget").unwrap_or(&self.name))
    }

    /// Resolved `handle` — defaults to `kind()`.
    pub fn handle(&self) -> &str {
        self.handle.as_deref().unwrap_or_else(|| self.kind())
    }
}

/// A property binding declaration.
pub struct PropDecl {
    /// Widget struct field name (e.g. `"content"`, `"font_size"`).
    pub field: String,
    /// WinUI metadata property name (the TOML key, e.g. `"PlaceholderText"`).
    /// Used as the default for `prop()` (Prop enum variant).
    pub meta_name: String,
    /// `Prop` enum variant name override. If omitted, defaults to `meta_name`.
    pub prop: Option<String>,
    /// `PropValue` variant name (e.g. `"Str"`, `"F64"`, `"Bool"`).
    /// If omitted, inferred from metadata parameter type.
    pub value: Option<String>,
    /// Emission rule for `bindings()`.
    pub emit: Emit,

    /// Override for IInspectable params: `"ireference"` to use IReference wrapping
    /// instead of the default TextBlock wrapping.
    /// Only meaningful when the metadata param type is IInspectable.
    pub wrap: Option<String>,

    // ── Setter (pick one) ─────────────────────────────────────────
    /// COM method name (e.g. `"put_Text"`).  The codegen tool resolves
    /// the owning interface from winmd metadata automatically.
    /// Mutually exclusive with `setter_fn` and `method_textblock`.
    pub method: Option<String>,

    /// Like `method` but wraps the value in `Some()` before calling.
    /// Used for nullable COM properties like `put_IsChecked(Option<bool>)`.
    /// Unset calls with `None`.
    pub method_optional: Option<String>,

    /// Like `method` but wraps the value in `IReference::from()` before calling.
    /// Used for IInspectable-typed properties that accept a boxed string.
    /// Generates:
    ///   set: `IReference::from(v.as_str())` → `h.cast::<I>()?.put_Foo(&insp)?`
    ///   unset: `h.cast::<I>()?.put_Foo(None)?`
    pub method_ireference: Option<String>,

    /// Like `method` but wraps a Str value in a TextBlock first.
    /// Used for `put_Header` and similar IInspectable-typed properties
    /// where the reactor passes a string.  Generates:
    ///   set: `string_as_textblock(v)` → `h.cast::<I>()?.put_Header(&tb)?`
    ///   unset: `h.cast::<I>()?.put_Header(None)?`
    pub method_textblock: Option<String>,

    /// Bool→enum mapping setter. Generates code that maps a bool to a
    /// WinUI enum value and calls the COM method.
    pub method_bool_enum: Option<BoolEnumSetter>,

    /// Multi-variant enum mapping setter. Maps each Rust enum variant to a
    /// WinUI enum variant and calls one `put_*` method.
    pub method_enum_map: Option<EnumMapSetter>,

    /// Hand-written custom function name (e.g. `"custom_set_button_content"`).
    /// Used for complex setters that can't be expressed as a simple COM call.
    /// Can be `true` to auto-generate: `custom_set_{control}_{field}`.
    /// Mutually exclusive with `method`.
    pub setter_fn: Option<String>,

    /// How `PropValue::Unset` is handled.
    /// - `unset = "custom"` → auto-generates `custom_unset_{control}_{field}`
    /// - `unset = { default = "value" }` → reset to literal
    /// - `unset = { fn = "name" }` → call explicit function
    pub unset: Option<UnsetPolicy>,
}

/// Bool-to-enum setter configuration.
#[derive(Deserialize, Clone, Debug)]
pub struct BoolEnumSetter {
    /// COM method name (e.g. `"put_Orientation"`).
    /// Defaults to `put_{meta_name}`.
    #[serde(default)]
    pub method: Option<String>,
    /// Enum type path relative to `Xaml::` (e.g. `"Orientation"`).
    pub enum_type: String,
    /// Variant when true (e.g. `"Vertical"`).
    pub true_variant: String,
    /// Variant when false (e.g. `"Horizontal"`).
    pub false_variant: String,
}

impl BoolEnumSetter {
    pub fn method(&self) -> &str {
        self.method.as_deref().expect("method must be resolved")
    }
}

/// Multi-variant enum mapping setter.
#[derive(Deserialize, Clone, Debug)]
pub struct EnumMapSetter {
    /// COM method name (e.g. `"put_Severity"`).
    /// Defaults to `put_{meta_name}`.
    #[serde(default)]
    pub method: Option<String>,
    /// Rust enum type used inside the PropValue variant when it differs from
    /// the PropValue variant name (e.g. `"TreeSelectionMode"`).
    #[serde(default)]
    pub rust_type: Option<String>,
    /// WinUI enum type relative to `Xaml::` (e.g. `"InfoBarSeverity"`).
    pub winui_type: String,
    /// `[RustVariant, WinUIVariant]` pairs.
    pub variants: Vec<[String; 2]>,
}

impl EnumMapSetter {
    pub fn method(&self) -> &str {
        self.method.as_deref().expect("method must be resolved")
    }
}

/// Resolved setter kind — computed from the TOML fields.
pub enum SetterKind<'a> {
    /// Auto-resolved via metadata: `h.cast::<Xaml::{interface}>()?.{method}({arg})?`
    Method {
        method: &'a str,
    },
    /// Like Method but wraps value in Some(): `h.cast::<I>()?.method(Some(arg))?`
    MethodOptional {
        method: &'a str,
    },
    /// Wraps string in IReference then calls method.
    MethodIReference {
        method: &'a str,
    },
    /// Wrap string in TextBlock then call method with IInspectable.
    MethodTextblock {
        method: &'a str,
    },
    /// Map bool to enum variant then call method.
    MethodBoolEnum {
        setter: &'a BoolEnumSetter,
    },
    /// Map multi-variant Rust enum to WinUI enum.
    MethodEnumMap {
        setter: &'a EnumMapSetter,
    },
    Custom,
}

impl PropDecl {
    /// Resolve the prop name. Defaults to metadata name (TOML key).
    pub fn prop(&self) -> String {
        if let Some(p) = &self.prop {
            return p.clone();
        }
        self.meta_name.clone()
    }

    /// Get the resolved value variant name.
    /// Panics if not set (must call `resolve_defaults` first or set explicitly).
    pub fn value(&self) -> &str {
        self.value
            .as_deref()
            .unwrap_or_else(|| panic!("prop '{}' has no value — set it explicitly or ensure resolve_defaults() was called", self.prop()))
    }

    /// Fill in default values that can be inferred.
    ///
    /// * `method` defaults to `put_{meta_name}` when no setter kind is specified.
    /// * `value` defaults to the metadata-inferred type when available.
    /// * Setter pattern auto-detected from metadata param type:
    ///   - IInspectable + Str → `method_textblock` (or `method_ireference` with `wrap = "ireference"`)
    ///   - IReference<bool> → `method_optional`
    ///
    /// Must be called after construction, before any codegen.
    pub fn resolve_defaults(&mut self, resolver: Option<&MetadataResolver>, handle: &str) {
        use crate::metadata::ParamClass;

        // Infer method when no setter is specified.
        // Use put_{meta_name} — the metadata name is authoritative.
        let has_setter = self.method.is_some()
            || self.method_optional.is_some()
            || self.method_ireference.is_some()
            || self.method_textblock.is_some()
            || self.method_bool_enum.is_some()
            || self.method_enum_map.is_some()
            || self.setter_fn.is_some();
        if !has_setter {
            let method_name = format!("put_{}", self.meta_name);
            // Auto-detect setter pattern from metadata param type.
            if let Some(resolver) = resolver {
                if let Some(param_class) = resolver.classify_param(handle, &method_name) {
                    match param_class {
                        ParamClass::IInspectable => {
                            if self.wrap.as_deref() == Some("ireference") {
                                self.method_ireference = Some(method_name);
                            } else {
                                self.method_textblock = Some(method_name);
                            }
                        }
                        ParamClass::NullableBool => {
                            self.method_optional = Some(method_name);
                        }
                        ParamClass::Primitive | ParamClass::Complex => {
                            self.method = Some(method_name);
                        }
                    }
                } else {
                    self.method = Some(method_name);
                }
            } else {
                self.method = Some(method_name);
            }
        }

        // Resolve default method for bool_enum/enum_map when omitted.
        // Use put_{meta_name} — the metadata name is authoritative.
        if self
            .method_bool_enum
            .as_ref()
            .is_some_and(|s| s.method.is_none())
        {
            let method = format!("put_{}", self.meta_name);
            self.method_bool_enum.as_mut().unwrap().method = Some(method);
        }
        if self
            .method_enum_map
            .as_ref()
            .is_some_and(|s| s.method.is_none())
        {
            let method = format!("put_{}", self.meta_name);
            self.method_enum_map.as_mut().unwrap().method = Some(method);
        }

        // Infer value from metadata when omitted.
        if self.value.is_none() {
            let method_name = self
                .method
                .as_deref()
                .or(self.method_optional.as_deref())
                .or(self.method_ireference.as_deref())
                .or(self.method_textblock.as_deref())
                .or(self
                    .method_bool_enum
                    .as_ref()
                    .and_then(|s| s.method.as_deref()))
                .or(self
                    .method_enum_map
                    .as_ref()
                    .and_then(|s| s.method.as_deref()));
            if let (Some(resolver), Some(method)) = (resolver, method_name)
                && let Some(value) = resolver.infer_value_type(handle, method)
            {
                self.value = Some(value);
            }
        }

        // Infer unset from emit: when_false → true, when_true → false.
        if self.unset.is_none() && self.method.is_some() {
            let inferred = match &self.emit {
                Emit::WhenFalse => Some("true".to_string()),
                Emit::WhenTrue => Some("false".to_string()),
                _ => None,
            };
            if let Some(default) = inferred {
                self.unset = Some(UnsetPolicy::Default { default });
            }
        }
    }

    /// Get the resolved setter kind, validating mutual exclusion.
    pub fn setter(&self) -> SetterKind<'_> {
        let prop = self.prop();
        let count = self.method.is_some() as u8
            + self.method_optional.is_some() as u8
            + self.method_ireference.is_some() as u8
            + self.method_textblock.is_some() as u8
            + self.method_bool_enum.is_some() as u8
            + self.method_enum_map.is_some() as u8
            + self.setter_fn.is_some() as u8;
        assert!(
            count <= 1,
            "prop '{prop}' has multiple setter types — pick exactly one",
        );
        assert!(
            count >= 1,
            "prop '{prop}' has no setter — set one of method/method_optional/method_ireference/method_textblock/method_bool_enum/setter_fn",
        );
        if let Some(m) = &self.method {
            SetterKind::Method { method: m }
        } else if let Some(m) = &self.method_optional {
            SetterKind::MethodOptional { method: m }
        } else if let Some(m) = &self.method_ireference {
            SetterKind::MethodIReference { method: m }
        } else if let Some(m) = &self.method_textblock {
            SetterKind::MethodTextblock { method: m }
        } else if let Some(s) = &self.method_bool_enum {
            SetterKind::MethodBoolEnum { setter: s }
        } else if let Some(s) = &self.method_enum_map {
            SetterKind::MethodEnumMap { setter: s }
        } else {
            SetterKind::Custom
        }
    }
}

/// When a prop is emitted in `bindings()`.
#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Emit {
    /// Always emitted, even at default value.
    Always,
    /// Emitted only when `Some(v)` (field is `Option<T>`).
    Optional,
    /// Emitted only when the bool field is `true`.
    WhenTrue,
    /// Emitted only when the bool field is `false`.
    WhenFalse,
    /// Emitted only when the field differs from a default value.
    /// The string is the default expression to compare against.
    NonDefault(String),
}

/// What happens when a previously-set prop is removed (`PropValue::Unset`).
#[derive(Clone, Debug)]
pub enum UnsetPolicy {
    /// `unset = "custom"` — hand-written in mod.rs, skip codegen.
    Custom,
    /// `unset = { default = "expr" }` — reset to literal default.
    Default { default: String },
}

impl<'de> serde::Deserialize<'de> for UnsetPolicy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = toml::Value::deserialize(deserializer)?;
        match &value {
            toml::Value::String(s) if s == "custom" => Ok(UnsetPolicy::Custom),
            toml::Value::String(s) => Err(serde::de::Error::custom(format!(
                "unknown unset policy: \"{s}\""
            ))),
            toml::Value::Table(t) => {
                if let Some(d) = t.get("default").and_then(|v| v.as_str()) {
                    Ok(UnsetPolicy::Default {
                        default: d.to_string(),
                    })
                } else {
                    Err(serde::de::Error::custom(
                        "unset table must have a 'default' key",
                    ))
                }
            }
            _ => Err(serde::de::Error::custom("unset must be a string or table")),
        }
    }
}

/// Derive the argument expression from the `PropValue` variant name.
///
/// This replaces the old `ArgKind` enum — the mapping is deterministic:
/// - `Str` → `v.as_str()` (HSTRING params take `&str`)
/// - `Bool`, `F64`, `I32`, `U16`, `U32` → `*v` (Copy types)
/// - everything else → `v.clone()` (owned reference types)
pub fn arg_expr_for_value(value_variant: &str) -> &'static str {
    match value_variant {
        "Str" => "str_ref",
        "Bool" | "F64" | "I32" | "U16" | "U32" | "U8" => "copy",
        _ => "clone",
    }
}

/// An event binding declaration.
pub struct EventDecl {
    /// Widget struct field name (e.g. `"on_changed"`).
    pub field: String,
    /// WinUI metadata event name (the TOML key, e.g. `"PasswordChanged"`).
    /// Used as the default for `event()` (Event enum variant).
    pub meta_name: String,
    /// `Event` enum variant name override. If omitted, defaults to `meta_name`.
    pub event: Option<String>,
    /// `EventHandler` variant name. If omitted, inferred from `event`.
    pub handler: Option<String>,

    // ── Attachment (pick one) ─────────────────────────────
    /// Custom attach function name. If present, event attachment is
    /// delegated to this hand-written function.
    pub attach_fn: Option<String>,

    /// COM add method name for auto-generated event attachment.
    /// If omitted and no `attach_fn`, inferred as `"add_{event}"`.
    pub add_method: Option<String>,

    /// How the handler is invoked. Required when `add_method` is set.
    pub invoke: Option<String>,

    /// Getter method on sender/args for extracting values (e.g. `"get_IsOn"`).
    pub getter: Option<String>,

    /// Second add method for dual-event patterns (e.g. `"add_Collapsed"`).
    pub add_method_false: Option<String>,
}

impl EventDecl {
    /// Resolve the event name. Defaults to metadata name (TOML key).
    pub fn event(&self) -> String {
        if let Some(e) = &self.event {
            return e.clone();
        }
        self.meta_name.clone()
    }

    /// Resolve the handler name, inferring from event if not explicit.
    pub fn handler(&self) -> String {
        if let Some(h) = &self.handler {
            return h.clone();
        }
        // Default: handler = event
        self.event()
    }

    /// Resolve the add_method, inferring from event if not explicit.
    pub fn add_method(&self) -> Option<String> {
        if let Some(m) = &self.add_method {
            return Some(m.clone());
        }
        if self.attach_fn.is_some() {
            return None;
        }
        // Infer: "Click" → "add_Click"
        Some(format!("add_{}", self.event()))
    }

    /// Resolve the invoke pattern, inferring from handler + getter + add_method_false.
    ///
    /// Rules:
    /// - Explicit invoke override → use as-is
    /// - add_method_false present → "invoke_bool_dual" (hooks both true/false events)
    /// - Click handler (unit callback) → "invoke"
    /// - CheckedChanged + getter → "invoke_bool_getter"
    /// - TextChanged + getter → "invoke_string_getter"
    /// - ValueChanged + getter → "invoke_f64_getter" (or "invoke_f64_args" if explicit)
    /// - IndexChanged + getter → "invoke_i32_getter" (or "invoke_i32_args" if explicit)
    pub fn invoke(&self) -> &str {
        if let Some(inv) = &self.invoke {
            return inv;
        }
        if self.add_method_false.is_some() {
            return "invoke_bool_dual";
        }
        let handler = self.handler();
        match handler.as_str() {
            "Click" => "invoke",
            "CheckedChanged" => "invoke_bool_getter",
            "TextChanged" | "TabKey" if self.getter.is_some() => "invoke_string_getter",
            "ValueChanged" if self.getter.is_some() => "invoke_f64_getter",
            "IndexChanged" if self.getter.is_some() => "invoke_i32_getter",
            _ => "invoke",
        }
    }
}

/// Validate TOML declarations against metadata. Returns a list of warnings.
/// Catches mismatches like a method name that doesn't resolve, or an event
/// add_method that doesn't exist in metadata.
pub fn validate(controls: &[Control], resolver: &MetadataResolver) -> Vec<String> {
    let mut warnings = Vec::new();
    for ctrl in controls {
        let handle = ctrl.handle();
        for p in &ctrl.prop {
            // Skip custom setters — they have hand-written code.
            if p.setter_fn.is_some() {
                continue;
            }
            let method_name = p
                .method
                .as_deref()
                .or(p.method_optional.as_deref())
                .or(p.method_ireference.as_deref())
                .or(p.method_textblock.as_deref())
                .or(p
                    .method_bool_enum
                    .as_ref()
                    .and_then(|s| s.method.as_deref()))
                .or(p.method_enum_map.as_ref().and_then(|s| s.method.as_deref()));
            if let Some(method) = method_name
                && resolver.resolve(handle, method).is_none()
            {
                warnings.push(format!(
                    "  ⚠ {handle}.{} → {method} not found in metadata",
                    p.field,
                ));
            }
        }
        for e in &ctrl.event {
            // Skip custom attach — they have hand-written code.
            if e.attach_fn.is_some() {
                continue;
            }
            if let Some(add_method) = e.add_method()
                && resolver.resolve(handle, &add_method).is_none()
            {
                warnings.push(format!(
                    "  ⚠ {handle}.{} → {add_method} not found in metadata",
                    e.field,
                ));
            }
        }
    }
    warnings
}
