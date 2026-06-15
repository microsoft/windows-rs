use super::*;

#[derive(Clone)]
pub struct Config<'a> {
    pub bindgen: &'a Bindgen,
    pub reader: &'a Reader,
    pub types: &'a TypeMap,
    pub references: &'a References,
    pub filter: &'a Filter,
    pub implement: Option<&'a Implements>,
    pub derive: &'a Derive,
    pub link: &'a str,
    pub warnings: &'a WarningBuilder,
    pub namespace: &'static str,
    /// Delegates that are exclusively used as parameters in `add_*` SpecialName
    /// methods (event handlers). In minimal mode these delegates have their
    /// `new()` and `Invoke()` methods suppressed because the event-add wrapper
    /// inlines the DelegateBox construction directly.
    pub event_only_delegates: &'a HashSet<TypeName>,
}

impl Config<'_> {
    pub fn with_namespace(&self, namespace: &'static str) -> Self {
        let mut clone = self.clone();
        clone.namespace = namespace;
        clone
    }

    /// Returns `true` if the `_Impl` scaffolding for the given type should be
    /// emitted, based on the `--implement` option.
    ///
    /// `default` is the behavior to fall back on when `--implement` is not
    /// set: `true` for types that are emitted unconditionally today (such
    /// as COM/Win32 interfaces) and `false` (or some other condition such as
    /// `!is_exclusive`) for WinRT interfaces.
    pub fn should_implement(&self, name: TypeName, default: bool) -> bool {
        match self.implement {
            None => default,
            Some(implements) if implements.is_empty() => true,
            Some(implements) => implements.matches(name),
        }
    }

    /// Returns `true` if the given method should be emitted (not demoted).
    /// In lean+COM mode, only methods on explicitly-filtered types are kept;
    /// dependency types have all methods demoted by default.
    pub fn includes_method(&self, type_name: TypeName, method: MethodDef) -> bool {
        if self.bindgen.style.has_com() {
            // If `--implement` requests this interface, keep all methods.
            if let Some(implements) = self.implement {
                if implements.matches(type_name) {
                    return true;
                }
            }
            // In lean+COM mode, types not in the method filter have all
            // methods demoted — they are dependencies, not explicitly requested.
            let key = (
                type_name.namespace().to_string(),
                type_name.name().to_string(),
            );
            let Some(method_filter) = self.filter.method_entries().get(&key) else {
                return false;
            };
            // If the method filter has explicit keep entries, check them.
            // Empty keep = all methods (::* or type appeared as dependency of
            // a method entry without specific method restrictions).
            let raw = method.name();
            if method_filter.includes_for_closure(raw) {
                return true;
            }
            if let Some(overload) = method_overload_name(method) {
                if method_filter.includes_for_closure(&overload) {
                    return true;
                }
            }
            // If `remove_X` is requested via the corresponding `add_X`,
            // keep it — the auto-revoking event pattern needs both slots.
            if let Some(event) = raw.strip_prefix("remove_") {
                let add_name = format!("add_{event}");
                return method_filter.includes_for_closure(&add_name);
            }
            false
        } else {
            self.filter.includes_method(type_name, method)
        }
    }
}

impl Config<'_> {
    #[track_caller]
    pub fn write(&self, tree: TypeTree) {
        if self.bindgen.layout.is_package() {
            self.write_package(&tree);
        } else {
            self.write_file(tree);
        }
    }

    #[track_caller]
    fn write_file(&self, tree: TypeTree) {
        let tokens = if self.bindgen.layout.is_flat() {
            self.write_flat(tree)
        } else {
            self.write_modules(&tree)
        };

        write_to_file(&self.bindgen.output, self.format(&tokens.into_string()));
    }

    fn write_flat(&self, tree: TypeTree) -> TokenStream {
        let mut tokens = TokenStream::new();

        for ty in tree.flatten_types() {
            tokens.combine(ty.write(self));
        }

        tokens
    }

    fn write_modules(&self, tree: &TypeTree) -> TokenStream {
        let mut tokens = TokenStream::new();

        for ty in &tree.types {
            tokens.combine(ty.write(self));
        }

        for (name, tree) in &tree.nested {
            let name = to_ident(name);
            let nested = self.with_namespace(tree.namespace).write_modules(tree);
            tokens.combine(quote! { pub mod #name { #nested } });
        }

        tokens
    }
}
