/// Reads WinUI `.winmd` files and resolves `put_*` method → interface mappings.
///
/// Given a WinUI class name (e.g. `"TextBlock"`) and a method (e.g. `"put_Text"`),
/// the resolver finds which exclusive interface owns that method (e.g. `"ITextBlock"`).
use std::collections::HashMap;
use std::path::Path;

use windows_metadata::reader::{File, Index, TypeCategory, TypeDef, TypeDefOrRef};
use windows_metadata::{HasAttributes, Type, Value};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReadValueConversion {
    Identity,
    Field(String),
    Nullable,
}

/// Pre-built lookup: `(class_short_name, method_name) → MethodRef`.
pub struct MetadataResolver {
    lookup: HashMap<(String, String), MethodRef>,
    base_classes: HashMap<(String, String), (String, String)>,
    /// Exclusive interface -> runtime class. Ambiguous non-exclusive interfaces map to `None`.
    interface_owners: HashMap<(String, String), Option<(String, String)>>,
    /// Value-type structs that wrap a single primitive field.
    /// Maps `(namespace, name)` → the unwrapped inner `Type`.
    single_field_types: HashMap<(String, String), (String, Type)>,
    /// Enum types: maps `(namespace, name)` → list of variant names.
    enum_variants: HashMap<(String, String), Vec<String>>,
    /// Non-generic delegate → args class short name, resolved from the
    /// delegate's `Invoke` method signature.
    delegate_args: HashMap<String, String>,
    content_properties: HashMap<(String, String), String>,
}

impl MetadataResolver {
    /// Load all `.winmd` files from `winmd_dir`, add the default Windows metadata, and build the
    /// resolver.
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

        let mut files = read_dir(winmd_dir);
        files.extend(
            [windows_default::WINRT, windows_default::WIN32]
                .into_iter()
                .map(|bytes| File::new(bytes.to_vec()).unwrap()),
        );

        assert!(
            !files.is_empty(),
            "no .winmd files found in {}",
            winmd_dir.display()
        );

        let index = Index::new(files);
        let mut lookup = HashMap::new();
        let mut interface_owners = HashMap::new();
        let mut base_classes = HashMap::new();
        let mut content_properties = HashMap::new();

        // Walk all types in the index, collecting method→interface for classes
        // in Microsoft.UI.Xaml namespaces.
        for (namespace, name, typedef) in index.iter() {
            if namespace.starts_with("Microsoft.UI.Xaml")
                && typedef.category() == TypeCategory::Class
            {
                if let Some(extends) = typedef.extends() {
                    let base = match extends {
                        TypeDefOrRef::TypeDef(base) => {
                            Some((base.namespace().to_string(), base.name().to_string()))
                        }
                        TypeDefOrRef::TypeRef(base) => {
                            Some((base.namespace().to_string(), base.name().to_string()))
                        }
                        _ => None,
                    };
                    if let Some(base) = base {
                        base_classes.insert((namespace.to_string(), name.to_string()), base);
                    }
                }
                if let Some(content) =
                    typedef
                        .find_attribute("ContentPropertyAttribute")
                        .and_then(|attribute| {
                            attribute.value().into_iter().find_map(|(name, value)| {
                                (name == "Name")
                                    .then_some(value)
                                    .and_then(|value| match value {
                                        Value::Utf8(value) => Some(value),
                                        _ => None,
                                    })
                            })
                        })
                {
                    content_properties.insert((namespace.to_string(), name.to_string()), content);
                }
                for implementation in typedef.interface_impls() {
                    let interface = implementation.interface(&[]);
                    let (interface_namespace, interface_name) = match interface {
                        Type::ClassName(type_name) | Type::ValueName(type_name) => {
                            (type_name.namespace, type_name.name)
                        }
                        _ => continue,
                    };
                    let owner = (namespace.to_string(), name.to_string());
                    interface_owners
                        .entry((interface_namespace, interface_name))
                        .and_modify(|existing: &mut Option<(String, String)>| {
                            if existing.as_ref() != Some(&owner) {
                                *existing = None;
                            }
                        })
                        .or_insert(Some(owner));
                }
                Self::collect_methods_for_class(&index, name, &typedef, &mut lookup);
            }
        }

        // Validate all entries - remove any with namespaces that don't exist in the index.
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
                    single_field_types.insert(
                        (namespace.to_string(), name.to_string()),
                        (crate::helpers::to_snake_case(fields[0].name()), inner_ty),
                    );
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
            base_classes,
            interface_owners,
            single_field_types,
            enum_variants,
            delegate_args,
            content_properties,
        }
    }

    pub fn class_derives_from(&self, class: &str, base: &str) -> bool {
        let Some((class_namespace, class_name)) = class.rsplit_once('.') else {
            return false;
        };
        let Some((base_namespace, base_name)) = base.rsplit_once('.') else {
            return false;
        };
        let mut current = (class_namespace.to_string(), class_name.to_string());
        while let Some(parent) = self.base_classes.get(&current) {
            if parent.0 == base_namespace && parent.1 == base_name {
                return true;
            }
            current.clone_from(parent);
        }
        false
    }

    pub fn content_property(&self, class: &str) -> Option<String> {
        let (namespace, name) = class.rsplit_once('.')?;
        let mut current = (namespace.to_string(), name.to_string());
        loop {
            if let Some(content) = self.content_properties.get(&current) {
                return Some(content.clone());
            }
            current = self.base_classes.get(&current)?.clone();
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

    /// Resolve an exclusive interface to the runtime class that owns its static members.
    pub fn runtime_class(&self, interface: &InterfaceRef) -> Option<String> {
        let (namespace, name) = self
            .interface_owners
            .get(&(interface.namespace.clone(), interface.name.clone()))?
            .as_ref()?;
        Some(format!("{namespace}.{name}"))
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

    pub fn enum_path(&self, class_name: &str, method_name: &str) -> Option<String> {
        let mref = self
            .lookup
            .get(&(class_name.to_string(), method_name.to_string()))?;
        let Type::ValueName(type_name) = mref.param_types.first()? else {
            return None;
        };
        self.enum_variants
            .contains_key(&(type_name.namespace.clone(), type_name.name.clone()))
            .then(|| format!("{}.{}", type_name.namespace, type_name.name))
    }

    pub fn single_field_param(
        &self,
        class_name: &str,
        method_name: &str,
    ) -> Option<(String, String)> {
        let mref = self
            .lookup
            .get(&(class_name.to_string(), method_name.to_string()))?;
        let Type::ValueName(type_name) = mref.param_types.first()? else {
            return None;
        };
        let (field, _) = self
            .single_field_types
            .get(&(type_name.namespace.clone(), type_name.name.clone()))?;
        Some((
            format!("{}.{}", type_name.namespace, type_name.name),
            field.clone(),
        ))
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
    #[cfg(test)]
    pub fn infer_event_args_type(
        &self,
        class_name: &str,
        add_event: &str,
        property: &str,
    ) -> Option<String> {
        self.resolve_event_args_property(class_name, add_event, property)
            .map(|(value, _, _)| value)
    }

    pub fn resolve_event_args_property(
        &self,
        class_name: &str,
        add_event: &str,
        property: &str,
    ) -> Option<(String, String, ReadValueConversion)> {
        // Get the delegate type from the add method's first param.
        let add_ref = self
            .lookup
            .get(&(class_name.to_string(), add_event.to_string()))?;
        let delegate_type = add_ref.param_types.first()?;
        // Extract the args class name from the delegate type.
        let args_class = match delegate_type {
            // TypedEventHandler<TSender, TArgs> - extract TArgs from generics.
            Type::ClassName(tn) if tn.generics.len() == 2 => match &tn.generics[1] {
                Type::ClassName(args_tn) => args_tn.name.clone(),
                Type::ValueName(args_tn) => args_tn.name.clone(),
                _ => return None,
            },
            // Non-generic delegate - look up args class from Invoke signature.
            Type::ClassName(tn) => self.delegate_args.get(&tn.name)?.clone(),
            _ => return None,
        };
        // Look up get_{property} on the args class.
        let getter = format!("get_{property}");
        let getter_ref = self.lookup.get(&(args_class, getter))?;
        let (value, conversion) = self.read_value_for_type(&getter_ref.return_type)?;
        Some((value, getter_ref.interface.full_path(), conversion))
    }

    pub fn resolve_event_args_property_interface(
        &self,
        class_name: &str,
        add_event: &str,
        property: &str,
    ) -> Option<String> {
        let add_ref = self
            .lookup
            .get(&(class_name.to_string(), add_event.to_string()))?;
        let delegate_type = add_ref.param_types.first()?;
        let args_class = match delegate_type {
            Type::ClassName(tn) if tn.generics.len() == 2 => match &tn.generics[1] {
                Type::ClassName(args_tn) => args_tn.name.clone(),
                Type::ValueName(args_tn) => args_tn.name.clone(),
                _ => return None,
            },
            Type::ClassName(tn) => self.delegate_args.get(&tn.name)?.clone(),
            _ => return None,
        };
        self.lookup
            .get(&(args_class, format!("get_{property}")))
            .map(|method| method.interface.full_path())
    }

    pub fn resolve_event_args_object_property(
        &self,
        class_name: &str,
        add_event: &str,
        property: &str,
    ) -> Option<String> {
        let add_ref = self
            .lookup
            .get(&(class_name.to_string(), add_event.to_string()))?;
        let delegate_type = add_ref.param_types.first()?;
        let args_class = match delegate_type {
            Type::ClassName(tn) if tn.generics.len() == 2 => match &tn.generics[1] {
                Type::ClassName(args_tn) => args_tn.name.clone(),
                Type::ValueName(args_tn) => args_tn.name.clone(),
                _ => return None,
            },
            Type::ClassName(tn) => self.delegate_args.get(&tn.name)?.clone(),
            _ => return None,
        };
        let getter = format!("get_{property}");
        let getter_ref = self.lookup.get(&(args_class, getter))?;
        matches!(getter_ref.return_type, Type::Object).then(|| getter_ref.interface.full_path())
    }

    /// Resolves an event-args property that returns a class type (not a primitive or
    /// IInspectable). Returns the interface path for the getter on the event args.
    pub fn resolve_event_args_class_property(
        &self,
        class_name: &str,
        add_event: &str,
        property: &str,
    ) -> Option<String> {
        let add_ref = self
            .lookup
            .get(&(class_name.to_string(), add_event.to_string()))?;
        let delegate_type = add_ref.param_types.first()?;
        let args_class = match delegate_type {
            Type::ClassName(tn) if tn.generics.len() == 2 => match &tn.generics[1] {
                Type::ClassName(args_tn) => args_tn.name.clone(),
                Type::ValueName(args_tn) => args_tn.name.clone(),
                _ => return None,
            },
            Type::ClassName(tn) => self.delegate_args.get(&tn.name)?.clone(),
            _ => return None,
        };
        let getter = format!("get_{property}");
        let getter_ref = self.lookup.get(&(args_class, getter))?;
        matches!(getter_ref.return_type, Type::ClassName(_))
            .then(|| getter_ref.interface.full_path())
    }

    pub fn resolve_property_read(
        &self,
        class_name: &str,
        property: &str,
    ) -> Option<(String, String, ReadValueConversion)> {
        let getter = format!("get_{property}");
        let getter_ref = self.lookup.get(&(class_name.to_string(), getter))?;
        let (value, conversion) = self.read_value_for_type(&getter_ref.return_type)?;
        Some((value, getter_ref.interface.full_path(), conversion))
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
                if let Some((_, inner)) = self.single_field_types.get(&key) {
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
    #[cfg(test)]
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
                if let Some((_, inner)) = self.single_field_types.get(&key) {
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

    fn read_value_for_type(&self, ty: &Type) -> Option<(String, ReadValueConversion)> {
        match ty {
            Type::ClassName(type_name)
                if type_name.namespace == "Windows.Foundation"
                    && type_name.name == "IReference`1"
                    && type_name.generics.first() == Some(&Type::Bool) =>
            {
                Some(("Bool".to_string(), ReadValueConversion::Identity))
            }
            Type::ClassName(type_name)
                if type_name.namespace == "Windows.Foundation"
                    && type_name.name == "IReference`1" =>
            {
                let Type::ValueName(value) = type_name.generics.first()? else {
                    return None;
                };
                matches!(value.name.as_str(), "DateTime" | "TimeSpan")
                    .then(|| (value.name.clone(), ReadValueConversion::Nullable))
            }
            Type::Object | Type::ClassName(_) => None,
            Type::ValueName(type_name) => {
                let key = (type_name.namespace.clone(), type_name.name.clone());
                if let Some((field, inner)) = self.single_field_types.get(&key) {
                    return Some((
                        Self::primitive_value_for_type(inner)?,
                        ReadValueConversion::Field(field.clone()),
                    ));
                }
                (type_name.name == "Color")
                    .then(|| (type_name.name.clone(), ReadValueConversion::Identity))
            }
            _ => Some((
                Self::primitive_value_for_type(ty)?,
                ReadValueConversion::Identity,
            )),
        }
    }

    /// Map primitive and well-known Type variants to PropValue names.
    /// Does not require resolver state - used during construction.
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

    pub fn param_class_name(&self, class_name: &str, method_name: &str) -> Option<String> {
        let mref = self
            .lookup
            .get(&(class_name.to_string(), method_name.to_string()))?;
        let Type::ClassName(name) = mref.param_types.first()? else {
            return None;
        };
        Some(format!("{}.{}", name.namespace, name.name))
    }

    pub fn returns_inspectable_vector(&self, class_name: &str, method_name: &str) -> bool {
        let Some(mref) = self
            .lookup
            .get(&(class_name.to_string(), method_name.to_string()))
        else {
            return false;
        };
        matches!(
            &mref.return_type,
            Type::ClassName(name)
                if name.namespace == "Windows.Foundation.Collections"
                    && name.name == "IVector`1"
                    && name.generics.as_slice() == [Type::Object]
        )
    }

    pub fn return_vector_element_class_name(
        &self,
        class_name: &str,
        method_name: &str,
    ) -> Option<String> {
        let method = self
            .lookup
            .get(&(class_name.to_string(), method_name.to_string()))?;
        let Type::ClassName(vector) = &method.return_type else {
            return None;
        };
        if vector.namespace != "Windows.Foundation.Collections"
            || !matches!(vector.name.as_str(), "IVector`1" | "IObservableVector`1")
        {
            return None;
        }
        let Type::ClassName(element) = vector.generics.first()? else {
            return None;
        };
        Some(format!("{}.{}", element.namespace, element.name))
    }

    pub fn returns_observable_vector(&self, class_name: &str, method_name: &str) -> bool {
        let Some(method) = self
            .lookup
            .get(&(class_name.to_string(), method_name.to_string()))
        else {
            return false;
        };
        matches!(
            &method.return_type,
            Type::ClassName(name)
                if name.namespace == "Windows.Foundation.Collections"
                    && name.name == "IObservableVector`1"
        )
    }

    pub fn returns_object(&self, class_name: &str, method_name: &str) -> bool {
        self.lookup
            .get(&(class_name.to_string(), method_name.to_string()))
            .is_some_and(|method| method.return_type == Type::Object)
    }

    pub fn return_class_name(&self, class_name: &str, method_name: &str) -> Option<String> {
        let method = self
            .lookup
            .get(&(class_name.to_string(), method_name.to_string()))?;
        let Type::ClassName(name) = &method.return_type else {
            return None;
        };
        Some(format!("{}.{}", name.namespace, name.name))
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
    fn classifies_collection_getters() {
        let resolver = MetadataResolver::load(Path::new("winmd"));

        assert_eq!(
            resolver
                .return_class_name("ListBox", "get_Items")
                .as_deref(),
            Some("Microsoft.UI.Xaml.Controls.ItemCollection")
        );
        assert!(resolver.returns_inspectable_vector("NavigationView", "get_MenuItems"));
        assert_eq!(
            resolver
                .return_vector_element_class_name("SelectorBar", "get_Items")
                .as_deref(),
            Some("Microsoft.UI.Xaml.Controls.SelectorBarItem")
        );
    }

    #[test]
    fn resolves_inherited_content_properties() {
        let resolver = MetadataResolver::load(Path::new("winmd"));

        assert_eq!(
            resolver.content_property("Microsoft.UI.Xaml.Controls.Border"),
            Some("Child".to_string())
        );
        assert_eq!(
            resolver.content_property("Microsoft.UI.Xaml.Controls.Button"),
            Some("Content".to_string())
        );
        assert_eq!(
            resolver.content_property("Microsoft.UI.Xaml.Controls.TextBlock"),
            Some("Inlines".to_string())
        );
    }

    #[test]
    fn resolve_runtime_classes_for_versioned_and_digit_named_interfaces() {
        let resolver = MetadataResolver::load(Path::new("winmd"));

        let navigation = resolver
            .resolve("NavigationView", "put_IsBackButtonVisible")
            .unwrap();
        assert_eq!(
            resolver.runtime_class(navigation).as_deref(),
            Some("Microsoft.UI.Xaml.Controls.NavigationView")
        );

        let webview = resolver.resolve("WebView2", "put_Source").unwrap();
        assert_eq!(
            resolver.runtime_class(webview).as_deref(),
            Some("Microsoft.UI.Xaml.Controls.WebView2")
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
        // ContentDialog.IsOpen - might not exist as put_IsOpen in metadata
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
