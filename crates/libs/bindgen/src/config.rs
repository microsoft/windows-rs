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
    /// When present, the minimal filter controls method emission directly using
    /// raw metadata names, bypassing the Filter's method-level logic.
    pub minimal_filter: Option<&'a MinimalFilter>,
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
    /// In minimal mode, checks directly against the MinimalFilter using
    /// raw metadata names. Otherwise falls back to the Filter.
    pub fn includes_method(&self, type_name: TypeName, method: MethodDef) -> bool {
        if let Some(mf) = self.minimal_filter {
            // If `--implement` requests this interface, keep all methods.
            if let Some(implements) = self.implement {
                if implements.matches(type_name) {
                    return true;
                }
            }
            // In minimal mode, check both the raw method name and any overload name.
            if mf.includes_method(type_name, method.name()) {
                return true;
            }
            if let Some(overload) = method_overload_name(method) {
                return mf.includes_method(type_name, &overload);
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
