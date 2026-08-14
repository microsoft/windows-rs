use super::*;
use proc_macro2::TokenStream;
use quote::quote;

/// An owned Win32 function projection.
pub struct Function {
    architectures: i32,
    namespace: String,
    name: String,
    import_name: Option<String>,
    module: String,
    abi: &'static str,
    variadic: bool,
    signature: native_signature::Signature,
}

impl Function {
    pub(super) fn lower(
        database: &Database,
        dependencies: &native::DependencyCache,
        method: windows_metadata2::MethodDefinition<'_>,
        namespace: &str,
        name: &str,
    ) -> Result<Self, Error> {
        let full_name = format!("{namespace}.{name}");
        let architectures = method.architectures()?;
        let import = method.import()?.ok_or_else(|| Error::InvalidType {
            name: full_name.clone(),
            message: "Win32 function has no ImplMap",
        })?;
        let signature =
            native_signature::Signature::lower(database, dependencies, method, &full_name)?;
        let import_name = (import.name() != name).then(|| import.name().to_string());
        Ok(Self {
            architectures,
            namespace: namespace.to_string(),
            name: name.to_string(),
            import_name,
            module: import.module().to_lowercase(),
            abi: calling_convention(import.flags(), &full_name)?,
            variadic: signature.flags & 0x0f == 0x05,
            signature,
        })
    }

    /// Renders a flat Win32 `windows_link::link!` declaration.
    #[cfg(test)]
    pub fn write_sys(&self) -> TokenStream {
        self.write_context(Layout::Flat, Projection::Sys)
    }

    pub(super) fn write_context(&self, layout: Layout, projection: Projection) -> TokenStream {
        let architectures = tokens::architectures(self.architectures);
        let cfg = tokens::feature_cfg(
            &self.namespace,
            layout,
            self.signature
                .package_dependencies_for(projection)
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        );
        let module = &self.module;
        let abi = self.abi;
        let symbol = self.import_name.as_ref().map(|name| quote! { #name });
        let name = tokens::ident(&self.name);
        let parameters =
            self.signature
                .write_parameters_projection(&self.namespace, layout, projection);
        let variadic = self.variadic.then(|| quote! { , ... });
        let result = if self.signature.no_return {
            quote! { -> ! }
        } else {
            self.signature
                .write_result_projection(&self.namespace, layout, projection)
        };
        let pointer_alias = window_long_dependency(&self.name).map(|dependency| {
            let dependency = tokens::ident(dependency);
            quote! {
                #[cfg(target_pointer_width = "32")]
                pub use #dependency as #name;
            }
        });
        if !projection.is_sys() {
            if matches!(projection, Projection::Default)
                && let Some(wrapper) = self.signature.write_com_function(
                    &self.namespace,
                    layout,
                    &self.name,
                    module,
                    abi,
                    self.import_name.as_deref(),
                )
            {
                return quote! {
                    #architectures
                    #cfg
                    #wrapper
                    #pointer_alias
                };
            }
            quote! {
                #architectures
                #cfg
                windows_core::link!(#module #abi #symbol fn #name(#parameters #variadic) #result);
                #pointer_alias
            }
        } else {
            quote! {
                #architectures
                #cfg
                windows_link::link!(#module #abi #symbol fn #name(#parameters #variadic) #result);
                #pointer_alias
            }
        }
    }

    pub(super) fn package_features(
        &self,
        layout: Layout,
        projection: Projection,
    ) -> BTreeSet<String> {
        tokens::feature_names(
            &self.namespace,
            layout,
            self.signature
                .package_dependencies_for(projection)
                .iter()
                .map(|(namespace, name)| (namespace.as_str(), name.as_str())),
        )
    }

    pub(super) fn supports_package_sys(&self) -> bool {
        !self.signature.uses_winrt_projection()
    }
}

pub(super) fn window_long_dependency(name: &str) -> Option<&'static str> {
    match name {
        "GetWindowLongPtrA" => Some("GetWindowLongA"),
        "GetWindowLongPtrW" => Some("GetWindowLongW"),
        "SetWindowLongPtrA" => Some("SetWindowLongA"),
        "SetWindowLongPtrW" => Some("SetWindowLongW"),
        _ => None,
    }
}

pub(super) fn window_long_alias(name: &str) -> Option<&'static str> {
    match name {
        "GetWindowLongA" => Some("GetWindowLongPtrA"),
        "GetWindowLongW" => Some("GetWindowLongPtrW"),
        "SetWindowLongA" => Some("SetWindowLongPtrA"),
        "SetWindowLongW" => Some("SetWindowLongPtrW"),
        _ => None,
    }
}

fn calling_convention(flags: u16, name: &str) -> Result<&'static str, Error> {
    match flags & 0x0700 {
        0x0000 | 0x0100 | 0x0300 => Ok("system"),
        0x0200 => Ok("C"),
        0x0400 | 0x0500 => Err(Error::UnsupportedType {
            name: name.to_string(),
            shape: "unsupported calling convention".to_string(),
        }),
        _ => Err(Error::InvalidType {
            name: name.to_string(),
            message: "invalid P/Invoke calling convention",
        }),
    }
}
