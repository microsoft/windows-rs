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
    pub namespace: &'static str,
    /// Delegates that are exclusively used as parameters in `add_*` SpecialName
    /// methods (event handlers). In minimal mode these delegates have their
    /// `new()` and `Invoke()` methods suppressed because the event-add wrapper
    /// inlines the DelegateBox construction directly.
    pub event_only_delegates: &'a HashSet<TypeName>,
    /// The type whose `impl` block is currently being generated, if any. When a
    /// reference to this exact type (with matching generics) is written inside the
    /// block, it is emitted as `Self` so the generated code is `clippy::use_self`
    /// friendly. See `TypeName::write`.
    pub self_ty: Option<TypeName>,
    /// The generic arguments of `self_ty`'s enclosing `impl` block, used to ensure
    /// `Self` is only substituted when the referenced generics match exactly.
    pub self_generics: Vec<Type>,
}

impl Config<'_> {
    pub fn with_namespace(&self, namespace: &'static str) -> Self {
        let mut clone = self.clone();
        clone.namespace = namespace;
        clone
    }

    /// Returns a clone configured to emit `Self` for references to `name` (with the
    /// given generics) while generating that type's `impl` block.
    pub fn with_self_ty(&self, name: TypeName, generics: &[Type]) -> Self {
        let mut clone = self.clone();
        clone.self_ty = Some(name);
        clone.self_generics = generics.to_vec();
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

    /// Returns `true` if the WinRT `RuntimeType::NAME` constant should be emitted
    /// for the given interface. The constant is only needed for interfaces that
    /// are implemented (for `GetRuntimeClassName`); minimal bindings omit it for
    /// non-implemented interfaces, while other styles always emit it.
    pub fn emit_runtime_name(&self, name: TypeName) -> bool {
        !self.bindgen.style.is_minimal() || self.should_implement(name, false)
    }

    /// Emits the `RuntimeType::NAME` constant for a value type (struct/enum).
    /// Value types are never implemented as COM objects, so NAME is only useful
    /// when the type appears as a generic argument in an implemented
    /// parameterized interface; minimal bindings skip it (the trait default —
    /// an empty name — is sufficient).
    pub fn write_value_name_const(&self, type_name: TypeName) -> TokenStream {
        if self.bindgen.style.is_minimal() {
            quote! {}
        } else {
            let type_name_bytes = Literal::byte_string(format!("{type_name}").as_bytes());
            quote! {
                const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(#type_name_bytes);
            }
        }
    }

    /// Returns the visibility to emit on a generated item. With `--dead-code`
    /// this is `pub(crate)` so the compiler's dead-code lint can flag unused
    /// bindings (a `pub` item in a non-public module is not linted — see
    /// <https://github.com/rust-lang/rust/issues/157961>); otherwise `pub`.
    pub fn item_vis(&self) -> TokenStream {
        if self.bindgen.dead_code {
            quote! { pub(crate) }
        } else {
            quote! { pub }
        }
    }

    /// Returns `true` if the given method should be emitted (not demoted).
    pub fn includes_method(&self, type_name: TypeName, method: MethodDef) -> bool {
        // If `--implement` requests this interface, keep all methods.
        if let Some(implements) = self.implement {
            if implements.matches(type_name) {
                return true;
            }
        }
        self.filter
            .includes_method(type_name, method, self.bindgen.style.is_minimal())
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
