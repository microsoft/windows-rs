/// Reads WinUI `.winmd` files and resolves `put_*` method → interface mappings.
///
/// Given a WinUI class name (e.g. `"TextBlock"`) and a method (e.g. `"put_Text"`),
/// the resolver finds which exclusive interface owns that method (e.g. `"ITextBlock"`).
use std::collections::HashMap;
use std::path::Path;

use windows_metadata::Type;
use windows_metadata::reader::{File, Index, TypeDef, TypeDefOrRef};

/// Resolved interface location: namespace + name.
#[derive(Clone, Debug)]
pub struct InterfaceRef {
    pub namespace: String,
    pub name: String,
}

impl InterfaceRef {
    /// The short name (e.g. `"ITextBlock"`).
    pub fn short_name(&self) -> &str {
        &self.name
    }

    /// Full dotted path (e.g. `"Microsoft.UI.Xaml.Controls.ITextBlock"`).
    pub fn full_path(&self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }
}

/// Resolved method location: owning interface + parameter types.
#[derive(Clone, Debug)]
pub struct MethodRef {
    pub interface: InterfaceRef,
    /// Parameter types from the method signature (excludes `this`/return).
    pub param_types: Vec<Type>,
    /// Return type of the method.
    pub return_type: Type,
}

/// Classification of a metadata parameter type for setter pattern inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamClass {
    /// Primitive type (String, Bool, F64, etc.) → direct `put_X(value)`.
    Primitive,
    /// IInspectable → needs wrapping (textblock by default, or IReference when
    /// the TOML specifies an explicit type).
    IInspectable,
    /// IReference<bool> → `put_X(Some(value))`.
    NullableBool,
    /// Enum, struct, or other complex type → needs explicit TOML config.
    Complex,
}

/// Pre-built lookup: `(class_short_name, method_name) → MethodRef`.
pub struct MetadataResolver {
    lookup: HashMap<(String, String), MethodRef>,
    /// Value-type structs that wrap a single primitive field.
    /// Maps `(namespace, name)` → the unwrapped inner `Type`.
    single_field_types: HashMap<(String, String), Type>,
    /// Enum types: maps `(namespace, name)` → list of variant names.
    enum_variants: HashMap<(String, String), Vec<String>>,
    /// Non-generic delegate → args class short name, resolved from the
    /// delegate's `Invoke` method signature.
    delegate_args: HashMap<String, String>,
}

impl MetadataResolver {
    /// Load all `.winmd` files from `winmd_dir` — plus the reference winmds
    /// (`Windows.winmd`, `Windows.Win32.winmd`, `Windows.Wdk.winmd`) bundled in
    /// `windows-bindgen`'s `default/` directory — and build the resolver.
    ///
    /// The reference winmds live only in `crates/libs/bindgen/default` (the single
    /// source of truth); they are located relative to this crate's manifest so the
    /// path resolves the same whether the tool is run from the repo root or a test's
    /// crate directory.
    pub fn load(winmd_dir: &Path) -> Self {
        let read_dir = |dir: &Path| -> Vec<File> {
            std::fs::read_dir(dir)
                .unwrap_or_else(|_| panic!("cannot read winmd directory {}", dir.display()))
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.path()
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("winmd"))
                })
                .filter_map(|e| File::read(e.path()))
                .collect()
        };

        let default_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../libs/bindgen/default");

        let mut files = read_dir(winmd_dir);
        files.extend(read_dir(&default_dir));

        assert!(
            !files.is_empty(),
            "no .winmd files found in {}",
            winmd_dir.display()
        );

        let index = Index::new(files);
        let mut lookup = HashMap::new();

        // Walk all types in the index, collecting method→interface for classes
        // in Microsoft.UI.Xaml namespaces.
        for (namespace, name, typedef) in index.iter() {
            if namespace.starts_with("Microsoft.UI.Xaml") {
                Self::collect_methods_for_class(&index, name, &typedef, &mut lookup);
            }
        }

        // Validate all entries — remove any with namespaces that don't exist in the index.
        lookup.retain(|_, mref| {
            index
                .get(&mref.interface.namespace, &mref.interface.name)
                .next()
                .is_some()
        });

        // Build single-field struct map: value types with exactly one field
        // get mapped to their inner primitive type (e.g. FontWeight → U16).
        let mut single_field_types = HashMap::new();
        let mut enum_variants = HashMap::new();
        for (namespace, name, typedef) in index.iter() {
            let fields: Vec<_> = typedef.fields().collect();
            // Enums have a `value__` field plus named variant fields.
            let variant_names: Vec<_> = fields
                .iter()
                .filter(|f| f.name() != "value__")
                .map(|f| f.name().to_string())
                .collect();
            let has_value_field = fields.iter().any(|f| f.name() == "value__");

            if has_value_field && !variant_names.is_empty() {
                // This is an enum.
                enum_variants.insert((namespace.to_string(), name.to_string()), variant_names);
            } else if fields.len() == 1 {
                let inner_ty = fields[0].ty();
                if Self::primitive_value_for_type(&inner_ty).is_some() {
                    single_field_types.insert((namespace.to_string(), name.to_string()), inner_ty);
                }
            }
        }

        // Build delegate→args map: for non-generic delegates used by add_*
        // methods, resolve the Invoke method's second parameter to find the
        // event args class.
        let mut delegate_args = HashMap::new();
        for ((_, method_name), mref) in &lookup {
            if !method_name.starts_with("add_") {
                continue;
            }
            let Some(Type::ClassName(tn)) = mref.param_types.first() else {
                continue;
            };
            if !tn.generics.is_empty() || delegate_args.contains_key(&tn.name) {
                continue;
            }
            // Look up the delegate typedef and find its Invoke method.
            let Some(delegate_def) = index.get(&tn.namespace, &tn.name).next() else {
                continue;
            };
            for method in delegate_def.methods() {
                if method.name() == "Invoke" {
                    let sig = method.signature(&[]);
                    // Second parameter is the args type (first is sender).
                    if let Some(args_type) = sig.types.get(1) {
                        let args_name = match args_type {
                            Type::ClassName(args_tn) => Some(args_tn.name.clone()),
                            Type::ValueName(args_tn) => Some(args_tn.name.clone()),
                            _ => None,
                        };
                        if let Some(name) = args_name {
                            delegate_args.insert(tn.name.clone(), name);
                        }
                    }
                    break;
                }
            }
        }

        Self {
            lookup,
            single_field_types,
            enum_variants,
            delegate_args,
        }
    }

    /// Walk a class's interface hierarchy and record every `put_*` / `get_*` /
    /// `add_*` / `remove_*` method with its owning interface and parameter types.
    fn collect_methods_for_class(
        index: &Index,
        class_name: &str,
        typedef: &TypeDef<'_>,
        lookup: &mut HashMap<(String, String), MethodRef>,
    ) {
        // Walk implemented interfaces.
        for iface_impl in typedef.interface_impls() {
            let iface_type = iface_impl.interface(&[]);
            let (iface_ns, iface_name) = match &iface_type {
                Type::ClassName(tn) | Type::ValueName(tn) => {
                    (tn.namespace.as_str(), tn.name.as_str())
                }
                _ => continue,
            };

            // Look up the interface TypeDef to get its methods.
            let Some(iface_def) = index.get(iface_ns, iface_name).next() else {
                continue;
            };

            for method in iface_def.methods() {
                let method_name = method.name();
                if method_name.starts_with("put_")
                    || method_name.starts_with("get_")
                    || method_name.starts_with("add_")
                    || method_name.starts_with("remove_")
                {
                    lookup
                        .entry((class_name.to_string(), method_name.to_string()))
                        .or_insert_with(|| {
                            let sig = method.signature(&[]);
                            MethodRef {
                                interface: InterfaceRef {
                                    namespace: iface_ns.to_string(),
                                    name: iface_name.to_string(),
                                },
                                param_types: sig.types,
                                return_type: sig.return_type,
                            }
                        });
                }
            }
        }

        // Walk base class chain to inherit methods.
        let Some(extends) = typedef.extends() else {
            return;
        };
        let (base_ns, base_name) = match extends {
            TypeDefOrRef::TypeDef(td) => (td.namespace().to_string(), td.name().to_string()),
            TypeDefOrRef::TypeRef(tr) => (tr.namespace().to_string(), tr.name().to_string()),
            _ => return,
        };
        // Stop at System.Object / DependencyObject.
        if base_name == "Object" || base_name == "DependencyObject" {
            return;
        }

        // Resolve the base class and recurse.
        if let Some(base_def) = index.get(&base_ns, &base_name).next() {
            Self::collect_methods_for_class(index, &base_name, &base_def, lookup);

            // Propagate base class methods to the derived class.
            let base_methods: Vec<(String, MethodRef)> = lookup
                .iter()
                .filter(|((cn, _), _)| cn == &base_name)
                .map(|((_, mn), mref)| (mn.clone(), mref.clone()))
                .collect();
            for (method_name, method_ref) in base_methods {
                lookup
                    .entry((class_name.to_string(), method_name))
                    .or_insert(method_ref);
            }
        }
    }

    /// Resolve a method on a class to its interface.
    ///
    /// Returns the InterfaceRef if found.
    pub fn resolve(&self, class_name: &str, method_name: &str) -> Option<&InterfaceRef> {
        self.lookup
            .get(&(class_name.to_string(), method_name.to_string()))
            .map(|m| &m.interface)
    }

    /// Check if a method exists for a class in metadata.
    pub fn has_method(&self, class_name: &str, method_name: &str) -> bool {
        self.lookup
            .contains_key(&(class_name.to_string(), method_name.to_string()))
    }

    /// If a method's parameter is an enum, return `(short_name, [variant_names])`.
    /// Used to auto-generate `enum_map` from metadata without explicit TOML overrides.
    pub fn enum_info(&self, class_name: &str, method_name: &str) -> Option<(&str, &[String])> {
        let mref = self
            .lookup
            .get(&(class_name.to_string(), method_name.to_string()))?;
        let param = mref.param_types.first()?;
        if let Type::ValueName(tn) = param {
            let key = (tn.namespace.clone(), tn.name.clone());
            let variants = self.enum_variants.get(&key)?;
            Some((&tn.name, variants))
        } else {
            None
        }
    }

    /// Infer the `PropValue` variant name and Copy-ness from a method's parameter
    /// type in metadata.
    ///
    /// Returns `None` for complex types (enums, generics) that need
    /// explicit TOML declaration. The bool indicates whether the type is Copy.
    pub fn infer_value_type(&self, class_name: &str, method_name: &str) -> Option<(String, bool)> {
        let mref = self
            .lookup
            .get(&(class_name.to_string(), method_name.to_string()))?;
        let param = mref.param_types.first()?;
        let name = self.value_for_type(param)?;
        // Copy-ness follows the same unwrapping as value_for_type:
        // IReference<T> unwraps to T, single-field wrappers unwrap to inner.
        let copy = self.is_unwrapped_copy(param);
        Some((name, copy))
    }

    /// Infer the value type of a property on an event's args class.
    ///
    /// Given `(class, add_event, property)`, resolves the delegate parameter of
    /// `add_{event}` to find the args class, then looks up `get_{property}` on
    /// that class and returns the value type from its return type.
    pub fn infer_event_args_type(
        &self,
        class_name: &str,
        add_event: &str,
        property: &str,
    ) -> Option<String> {
        // Get the delegate type from the add method's first param.
        let add_ref = self
            .lookup
            .get(&(class_name.to_string(), add_event.to_string()))?;
        let delegate_type = add_ref.param_types.first()?;
        // Extract the args class name from the delegate type.
        let args_class = match delegate_type {
            // TypedEventHandler<TSender, TArgs> — extract TArgs from generics.
            Type::ClassName(tn) if tn.generics.len() == 2 => match &tn.generics[1] {
                Type::ClassName(args_tn) => args_tn.name.clone(),
                Type::ValueName(args_tn) => args_tn.name.clone(),
                _ => return None,
            },
            // Non-generic delegate — look up args class from Invoke signature.
            Type::ClassName(tn) => self.delegate_args.get(&tn.name)?.clone(),
            _ => return None,
        };
        // Look up get_{property} on the args class.
        let getter = format!("get_{property}");
        let getter_ref = self.lookup.get(&(args_class, getter))?;
        self.value_for_type(&getter_ref.return_type)
    }

    /// Returns true if a metadata `Type` is Copy (primitive or value type).
    ///
    /// Follows the same pattern as `is_copyable` in windows-bindgen:
    /// primitives and ValueName types (enums, blittable structs) are Copy;
    /// String, ClassName, Object, and generics are not.
    fn is_copy(ty: &Type) -> bool {
        match ty {
            Type::String | Type::Object | Type::ClassName(_) => false,
            Type::Generic(..) | Type::Array(_) => false,
            // Primitives, ValueName (enums + structs), etc. → Copy
            _ => true,
        }
    }

    /// Check copy-ness after applying the same unwrapping as `value_for_type`:
    /// IReference<T> → check T, single-field wrappers → check inner type.
    fn is_unwrapped_copy(&self, ty: &Type) -> bool {
        match ty {
            // IReference<T> → the PropValue wraps T, not IReference
            Type::ClassName(tn)
                if tn.namespace == "Windows.Foundation" && tn.name == "IReference`1" =>
            {
                tn.generics.first().is_some_and(Self::is_copy)
            }
            Type::ValueName(tn) => {
                let key = (tn.namespace.clone(), tn.name.clone());
                if let Some(inner) = self.single_field_types.get(&key) {
                    // Single-field wrapper → unwraps to inner primitive
                    Self::is_copy(inner)
                } else {
                    // Multi-field value type or enum → Copy
                    true
                }
            }
            _ => Self::is_copy(ty),
        }
    }

    /// Check copy-ness for a method's parameter type, applying IReference
    /// unwrapping.
    pub fn is_method_copy(&self, class_name: &str, method_name: &str) -> bool {
        let Some(mref) = self
            .lookup
            .get(&(class_name.to_string(), method_name.to_string()))
        else {
            return false;
        };
        let Some(param) = mref.param_types.first() else {
            return false;
        };
        self.is_unwrapped_copy(param)
    }

    /// Check if a PropValue variant name wraps a Copy type.
    ///
    /// Used as a fallback when no metadata method exists (setter_fn properties).
    /// Reverse-maps the name through `primitive_value_for_type` to find the
    /// underlying `Type` and checks `is_copy` on it. Unknown names default
    /// to non-Copy (safe — `.clone()` always works on Clone types).
    pub fn is_copy_value_name(&self, value_name: &str) -> bool {
        // Check every primitive Type — if primitive_value_for_type produces
        // this name, use is_copy on that Type.
        if [
            Type::String,
            Type::Bool,
            Type::F64,
            Type::I32,
            Type::U16,
            Type::U32,
            Type::U8,
            Type::Object,
        ]
        .iter()
        .any(|ty| {
            Self::primitive_value_for_type(ty).as_deref() == Some(value_name) && Self::is_copy(ty)
        }) {
            return true;
        }
        // Check metadata enums and value types by short name — enums and
        // value-type structs are always Copy in WinUI.
        self.enum_variants
            .keys()
            .any(|(_, name)| name == value_name)
            || self
                .single_field_types
                .keys()
                .any(|(_, name)| name == value_name)
    }

    /// Map a metadata Type to a PropValue variant name.
    /// Handles primitives, IReference<bool>, single-field wrapper structs
    /// (e.g. FontWeight{Weight: u16} → U16), and multi-field value-type
    /// structs by using the struct's short name (e.g. Thickness → "Thickness").
    pub fn value_for_type(&self, ty: &Type) -> Option<String> {
        // Try primitives and well-known types first.
        if let Some(v) = Self::primitive_value_for_type(ty) {
            return Some(v);
        }
        match ty {
            Type::ValueName(tn) => {
                let key = (tn.namespace.clone(), tn.name.clone());
                // Single-field wrapper structs → unwrap to inner primitive
                if let Some(inner) = self.single_field_types.get(&key) {
                    return Self::primitive_value_for_type(inner);
                }
                // Multi-field value types → use the struct's short name as
                // the PropValue variant (e.g. Thickness → "Thickness").
                // If there's no matching PropValue variant the generated code
                // won't compile, signalling that an explicit override is needed.
                Some(tn.name.clone())
            }
            _ => None,
        }
    }

    /// Map primitive and well-known Type variants to PropValue names.
    /// Does not require resolver state — used during construction.
    fn primitive_value_for_type(ty: &Type) -> Option<String> {
        match ty {
            Type::String => Some("Str".to_string()),
            Type::Bool => Some("Bool".to_string()),
            Type::F64 => Some("F64".to_string()),
            Type::I32 => Some("I32".to_string()),
            Type::U16 => Some("U16".to_string()),
            Type::U32 => Some("U32".to_string()),
            Type::Object => Some("Str".to_string()),
            Type::ClassName(tn)
                if tn.namespace == "Windows.Foundation"
                    && tn.name == "IReference`1"
                    && tn.generics.first() == Some(&Type::Bool) =>
            {
                Some("Bool".to_string())
            }
            _ => None,
        }
    }

    /// Classify the parameter type for setter pattern inference.
    pub fn classify_param(&self, class_name: &str, method_name: &str) -> Option<ParamClass> {
        let mref = self
            .lookup
            .get(&(class_name.to_string(), method_name.to_string()))?;
        let param = mref.param_types.first()?;
        Some(Self::classify_type(param))
    }

    /// Classify a Type into a setter pattern category.
    fn classify_type(ty: &Type) -> ParamClass {
        match ty {
            Type::String | Type::Bool | Type::F64 | Type::I32 | Type::U16 | Type::U32 => {
                ParamClass::Primitive
            }
            Type::Object => ParamClass::IInspectable,
            Type::ClassName(tn)
                if tn.namespace == "Windows.Foundation"
                    && tn.name == "IReference`1"
                    && tn.generics.first() == Some(&Type::Bool) =>
            {
                ParamClass::NullableBool
            }
            _ => ParamClass::Complex,
        }
    }

    /// Print a diagnostic report of resolution results for the given controls.
    pub fn report(&self, controls: &[crate::schema::Control]) {
        let mut resolved = 0u32;
        let mut custom = 0u32;
        let mut failed = 0u32;

        for ctrl in controls {
            for p in &ctrl.prop {
                match p.setter() {
                    crate::schema::SetterKind::Method { method }
                    | crate::schema::SetterKind::MethodOptional { method }
                    | crate::schema::SetterKind::MethodIReference { method }
                    | crate::schema::SetterKind::MethodTextblock { method } => {
                        if let Some(iface) = self.resolve(ctrl.handle(), method) {
                            eprintln!(
                                "  \u{2713} {}.{} -> {} resolved to {}",
                                ctrl.handle(),
                                p.prop(),
                                method,
                                iface.short_name()
                            );
                            resolved += 1;
                        } else {
                            eprintln!(
                                "  \u{2717} {}.{} -> {} NOT FOUND",
                                ctrl.handle(),
                                p.prop(),
                                method
                            );
                            failed += 1;
                        }
                    }
                    crate::schema::SetterKind::MethodEnumMap { setter } => {
                        if let Some(iface) = self.resolve(ctrl.handle(), setter.method()) {
                            eprintln!(
                                "  \u{2713} {}.{} -> {} resolved to {} (enum_map)",
                                ctrl.handle(),
                                p.prop(),
                                setter.method(),
                                iface.short_name()
                            );
                            resolved += 1;
                        } else {
                            eprintln!(
                                "  \u{2717} {}.{} -> {} NOT FOUND (enum_map)",
                                ctrl.handle(),
                                p.prop(),
                                setter.method()
                            );
                            failed += 1;
                        }
                    }
                    crate::schema::SetterKind::Custom => {
                        custom += 1;
                    }
                }
            }
        }
        eprintln!("  metadata: {resolved} resolved, {custom} custom, {failed} unresolved");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_textblock_put_text() {
        let resolver = MetadataResolver::load(Path::new("winmd"));
        let iface = resolver.resolve("TextBlock", "put_Text");
        assert_eq!(iface.map(|r| r.short_name()), Some("ITextBlock"));
    }

    #[test]
    fn resolve_button_put_is_enabled() {
        let resolver = MetadataResolver::load(Path::new("winmd"));
        // Button extends Control, so put_IsEnabled should resolve to IControl.
        let iface = resolver.resolve("Button", "put_IsEnabled");
        assert_eq!(iface.map(|r| r.short_name()), Some("IControl"));
    }

    #[test]
    fn resolve_slider_put_value() {
        let resolver = MetadataResolver::load(Path::new("winmd"));
        let iface = resolver.resolve("Slider", "put_Value");
        // Slider.put_Value is on IRangeBase (from RangeBase base class).
        assert!(iface.is_some(), "Slider.put_Value should resolve");
        assert_eq!(
            iface.unwrap().full_path(),
            "Microsoft.UI.Xaml.Controls.Primitives.IRangeBase"
        );
    }

    #[test]
    fn infer_single_field_wrapper_types() {
        let resolver = MetadataResolver::load(Path::new("winmd"));
        // FontWeight is a struct with one field (Weight: u16) → unwraps to U16, Copy
        assert_eq!(
            resolver.infer_value_type("TextBlock", "put_FontWeight"),
            Some(("U16".to_string(), true))
        );
        // Thickness is a multi-field struct → uses struct short name, Copy
        assert_eq!(
            resolver.infer_value_type("Border", "put_BorderThickness"),
            Some(("Thickness".to_string(), true))
        );
        // NavigateUri takes a Uri class → not a ValueName, returns None
        assert_eq!(
            resolver.infer_value_type("HyperlinkButton", "put_NavigateUri"),
            None
        );
        // Text takes a String → non-Copy
        assert_eq!(
            resolver.infer_value_type("TextBlock", "put_Text"),
            Some(("Str".to_string(), false))
        );
        // IsChecked takes IReference<bool> → unwraps to Copy bool
        assert!(resolver.is_method_copy("CheckBox", "put_IsChecked"));
        // ContentDialog.IsOpen — might not exist as put_IsOpen in metadata
        // (uses custom ShowAsync pattern)
        assert!(
            !resolver.has_method("ContentDialog", "put_IsOpen"),
            "ContentDialog.put_IsOpen should not be in metadata"
        );
        // Text takes String → non-Copy
        assert!(!resolver.is_method_copy("TextBlock", "put_Text"));
    }

    #[test]
    fn infer_event_args_type_numberbox() {
        let resolver = MetadataResolver::load(Path::new("winmd"));
        let result = resolver.infer_event_args_type("NumberBox", "add_ValueChanged", "NewValue");
        assert_eq!(
            result.as_deref(),
            Some("F64"),
            "NumberBox ValueChanged NewValue should be F64"
        );
    }

    #[test]
    fn infer_event_args_type_slider() {
        let resolver = MetadataResolver::load(Path::new("winmd"));
        let result = resolver.infer_event_args_type("Slider", "add_ValueChanged", "NewValue");
        assert_eq!(
            result.as_deref(),
            Some("F64"),
            "Slider ValueChanged NewValue should be F64"
        );
    }

    #[test]
    fn infer_event_args_type_breadcrumbbar() {
        let resolver = MetadataResolver::load(Path::new("winmd"));
        let result = resolver.infer_event_args_type("BreadcrumbBar", "add_ItemClicked", "Index");
        assert_eq!(
            result.as_deref(),
            Some("I32"),
            "BreadcrumbBar ItemClicked Index should be I32"
        );
    }
}
