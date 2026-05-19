use super::*;

/// Origin for crate-qualified helper paths like `windows_core::` or `super::`.
///
/// This captures the projection-mode/package/no-deps matrix once before rendering.
enum SpecificPathOrigin<'a> {
    SysCoreAbsolute,
    SysCoreFlat,
    SysCoreRelative { levels: usize },
    CoreCrate,
    SpecificCrate(&'a str),
}

/// Origin for a referenced type's namespace path.
///
/// Variants distinguish local relative paths from external-reference paths so rendering does
/// not need to reason about `flat`/`skip-root` booleans directly.
enum NamespacePathOrigin<'a> {
    Empty,
    ExternalFlat {
        crate_name: &'a str,
    },
    ExternalNamespaced {
        crate_name: &'a str,
        namespace: &'a str,
        skip_root: bool,
    },
    LocalRelative {
        current: &'a str,
        target: &'a str,
    },
}

impl Config<'_> {
    pub fn write_core(&self) -> TokenStream {
        self.write_specific("windows_core")
    }

    pub fn write_result(&self) -> TokenStream {
        self.write_specific("windows_result")
    }

    pub fn write_strings(&self) -> TokenStream {
        self.write_specific("windows_strings")
    }

    fn specific_path_origin<'a>(&'a self, specific: &'a str) -> SpecificPathOrigin<'a> {
        if self.mode.is_sys() {
            if self.package || !self.no_deps {
                SpecificPathOrigin::SysCoreAbsolute
            } else if self.flat {
                SpecificPathOrigin::SysCoreFlat
            } else {
                SpecificPathOrigin::SysCoreRelative {
                    levels: self.namespace.split('.').count(),
                }
            }
        } else if !self.specific_deps {
            SpecificPathOrigin::CoreCrate
        } else {
            SpecificPathOrigin::SpecificCrate(specific)
        }
    }

    fn write_specific(&self, specific: &str) -> TokenStream {
        match self.specific_path_origin(specific) {
            SpecificPathOrigin::SysCoreAbsolute => quote! { windows_sys::core:: },
            SpecificPathOrigin::SysCoreFlat => quote! {},
            SpecificPathOrigin::SysCoreRelative { levels } => {
                let mut path = String::new();

                for _ in 0..levels {
                    path.push_str("super::");
                }

                path.parse().unwrap()
            }
            SpecificPathOrigin::CoreCrate => quote! { windows_core:: },
            SpecificPathOrigin::SpecificCrate(specific) => format!("{specific}::").parse().unwrap(),
        }
    }

    pub fn write_generic_phantoms(&self, generics: &[Type]) -> TokenStream {
        if generics.is_empty() {
            quote! {}
        } else {
            let generics = generics.iter().map(|ty| ty.write_name(self));
            quote! { #(core::marker::PhantomData::<#generics>),* }
        }
    }

    pub fn write_generic_named_phantoms(&self, generics: &[Type]) -> TokenStream {
        if generics.is_empty() {
            quote! {}
        } else {
            let generics = generics.iter().map(|ty| ty.write_name(self));
            quote! { #(#generics: core::marker::PhantomData::<#generics>),* }
        }
    }

    pub fn write_generic_constraints(&self, generics: &[Type]) -> TokenStream {
        if generics.is_empty() {
            quote! {}
        } else {
            let generics = generics.iter().map(|ty| ty.write_name(self));
            quote! { #(#generics: windows_core::RuntimeType + 'static,)* }
        }
    }

    pub fn write_namespace(&self, type_name: &TypeName) -> TokenStream {
        fn namespace_path_origin<'a>(
            config: &'a Config<'_>,
            type_name: &'a TypeName,
        ) -> NamespacePathOrigin<'a> {
            let namespace = type_name.namespace();
            if namespace.is_empty() {
                return NamespacePathOrigin::Empty;
            }

            if let Some(reference) = {
                if config.types.contains_key(type_name) {
                    None
                } else {
                    config.references.contains(type_name)
                }
            } {
                if reference.style == ReferenceStyle::Flat {
                    return NamespacePathOrigin::ExternalFlat {
                        crate_name: &reference.name,
                    };
                }

                return NamespacePathOrigin::ExternalNamespaced {
                    crate_name: &reference.name,
                    namespace,
                    skip_root: reference.style == ReferenceStyle::SkipRoot,
                };
            }

            if config.flat || namespace == config.namespace.as_ref() {
                NamespacePathOrigin::Empty
            } else {
                NamespacePathOrigin::LocalRelative {
                    current: &config.namespace,
                    target: namespace,
                }
            }
        }

        let mut path = String::new();

        match namespace_path_origin(self, type_name) {
            NamespacePathOrigin::Empty => {}
            NamespacePathOrigin::ExternalFlat { crate_name } => {
                path.push_str(crate_name);
                path.push_str("::");
            }
            NamespacePathOrigin::ExternalNamespaced {
                crate_name,
                namespace,
                skip_root,
            } => {
                path.push_str(crate_name);
                path.push_str("::");

                let mut namespace = namespace.split('.').peekable();
                if skip_root {
                    namespace.next();
                }

                for segment in namespace {
                    path.push_str(segment);
                    path.push_str("::");
                }
            }
            NamespacePathOrigin::LocalRelative { current, target } => {
                let mut relative = current.split('.').peekable();
                let mut namespace = target.split('.').peekable();

                while relative.peek() == namespace.peek() {
                    if relative.next().is_none() {
                        break;
                    }
                    namespace.next();
                }

                for _ in 0..relative.count() {
                    path.push_str("super::");
                }

                for segment in namespace {
                    path.push_str(segment);
                    path.push_str("::");
                }
            }
        }

        path.parse().unwrap()
    }
}
