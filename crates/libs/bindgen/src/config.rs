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
    /// Event-only delegates whose constructor and `Invoke` can be suppressed in minimal mode.
    pub event_only_delegates: &'a HashSet<TypeName>,
    /// Current `impl` target, used by `TypeName::write` to emit `Self` when possible.
    pub self_ty: Option<TypeName>,
    /// Generics of `self_ty`, used to avoid incorrect `Self` substitutions.
    pub self_generics: Vec<Type>,
    /// Pruned empty namespaces; `Cfg::write` must not emit feature gates for them.
    pub prunable: std::sync::Arc<BTreeSet<&'static str>>,
}

impl Config<'_> {
    pub fn with_namespace(&self, namespace: &'static str) -> Self {
        let mut clone = self.clone();
        clone.namespace = namespace;
        clone
    }

    /// Returns a clone carrying the given set of pruned namespaces (see [`Config::prunable`]).
    pub fn with_prunable(&self, prunable: std::sync::Arc<BTreeSet<&'static str>>) -> Self {
        let mut clone = self.clone();
        clone.prunable = prunable;
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

    /// Applies `--implement`, falling back to `default` when the option is absent.
    pub fn should_implement(&self, name: TypeName, default: bool) -> bool {
        match self.implement {
            None => default,
            Some(implements) if implements.is_empty() => true,
            Some(implements) => implements.matches(name),
        }
    }

    /// Returns whether a class is an explicit minimal-mode composition target.
    pub fn should_compose(&self, name: TypeName) -> bool {
        self.bindgen.compose.iter().any(|target| {
            target
                .strip_prefix(name.namespace())
                .and_then(|name| name.strip_prefix('.'))
                == Some(name.name())
        })
    }

    /// Minimal bindings emit `RuntimeType::NAME` only for implemented interfaces.
    pub fn emit_runtime_name(&self, name: TypeName) -> bool {
        !self.bindgen.style.is_minimal() || self.should_implement(name, false)
    }

    /// Emits the value-type runtime name used by non-minimal generic signatures.
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

    /// Emits `pub(crate)` under `--dead-code` so unused generated items are linted.
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
        if let Some(implements) = self.implement
            && implements.matches(type_name)
        {
            return true;
        }
        self.filter.includes_method(type_name, method)
    }
}

impl Config<'_> {
    #[track_caller]
    pub fn write(&self, tree: TypeTree) {
        if self.bindgen.layout.is_package() {
            let phase = std::time::Instant::now();
            self.write_package(&tree);
            report_timing(&self.bindgen.output, "package output", phase.elapsed());
        } else {
            self.write_file(tree);
        }
    }

    #[track_caller]
    fn write_file(&self, tree: TypeTree) {
        let phase = std::time::Instant::now();
        let tokens = if self.bindgen.layout.is_flat() {
            self.write_flat(tree)
        } else {
            self.write_modules(&tree)
        };
        let tokens = tokens.into_string();
        report_timing(&self.bindgen.output, "render", phase.elapsed());

        let phase = std::time::Instant::now();
        let formatted = self.format(&tokens);
        report_timing(&self.bindgen.output, "format", phase.elapsed());

        let phase = std::time::Instant::now();
        write_to_file(&self.bindgen.output, formatted);
        report_timing(&self.bindgen.output, "write", phase.elapsed());
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
