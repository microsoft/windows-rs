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
}

/// Classification of a metadata parameter type for setter pattern inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamClass {
    /// Primitive type (String, Bool, F64, etc.) → direct `put_X(value)`.
    Primitive,
    /// IInspectable → needs wrapping (textblock by default, or IReference with override).
    IInspectable,
    /// IReference<bool> → `put_X(Some(value))`.
    NullableBool,
    /// Enum, struct, or other complex type → needs explicit TOML config.
    Complex,
}

/// Pre-built lookup: `(class_short_name, method_name) → MethodRef`.
pub struct MetadataResolver {
    lookup: HashMap<(String, String), MethodRef>,
}

impl MetadataResolver {
    /// Load all `.winmd` files from `winmd_dir` and build the resolver.
    pub fn load(winmd_dir: &Path) -> Self {
        let files: Vec<File> = std::fs::read_dir(winmd_dir)
            .expect("cannot read winmd directory")
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("winmd"))
            })
            .filter_map(|e| File::read(e.path()))
            .collect();

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

        MetadataResolver { lookup }
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

    /// Infer the `PropValue` variant name from a method's parameter type in metadata.
    ///
    /// Returns `None` for complex types (enums, generics) that need
    /// explicit TOML declaration.
    pub fn infer_value_type(&self, class_name: &str, method_name: &str) -> Option<String> {
        let mref = self
            .lookup
            .get(&(class_name.to_string(), method_name.to_string()))?;
        // put_ methods have exactly one parameter
        let param = mref.param_types.first()?;
        Self::value_for_type(param)
    }

    /// Map a metadata Type to a PropValue variant name.
    /// Returns None for types that need explicit TOML declaration.
    pub fn value_for_type(ty: &Type) -> Option<String> {
        match ty {
            Type::String => Some("Str".to_string()),
            Type::Bool => Some("Bool".to_string()),
            Type::F64 => Some("F64".to_string()),
            Type::I32 => Some("I32".to_string()),
            Type::U16 => Some("U16".to_string()),
            Type::U32 => Some("U32".to_string()),
            // IInspectable properties that accept Str are handled via textblock/ireference
            Type::Object => Some("Str".to_string()),
            // IReference<bool> → Bool
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
                    crate::schema::SetterKind::MethodBoolEnum { setter } => {
                        if let Some(iface) = self.resolve(ctrl.handle(), setter.method()) {
                            eprintln!(
                                "  \u{2713} {}.{} -> {} resolved to {} (bool_enum)",
                                ctrl.handle(),
                                p.prop(),
                                setter.method(),
                                iface.short_name()
                            );
                            resolved += 1;
                        } else {
                            eprintln!(
                                "  \u{2717} {}.{} -> {} NOT FOUND (bool_enum)",
                                ctrl.handle(),
                                p.prop(),
                                setter.method()
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
        let resolver = MetadataResolver::load(Path::new("../../../winmd"));
        let iface = resolver.resolve("TextBlock", "put_Text");
        assert_eq!(iface.map(|r| r.short_name()), Some("ITextBlock"));
    }

    #[test]
    fn resolve_button_put_is_enabled() {
        let resolver = MetadataResolver::load(Path::new("../../../winmd"));
        // Button extends Control, so put_IsEnabled should resolve to IControl.
        let iface = resolver.resolve("Button", "put_IsEnabled");
        assert_eq!(iface.map(|r| r.short_name()), Some("IControl"));
    }

    #[test]
    fn resolve_slider_put_value() {
        let resolver = MetadataResolver::load(Path::new("../../../winmd"));
        let iface = resolver.resolve("Slider", "put_Value");
        // Slider.put_Value is on IRangeBase (from RangeBase base class).
        assert!(iface.is_some(), "Slider.put_Value should resolve");
        assert_eq!(
            iface.unwrap().full_path(),
            "Microsoft.UI.Xaml.Controls.Primitives.IRangeBase"
        );
    }
}
