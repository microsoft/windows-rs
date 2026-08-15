use super::*;
use proc_macro2::TokenStream;
use quote::quote;

/// An owned Win32 native delegate projection.
pub struct Delegate {
    architectures: i32,
    namespace: String,
    name: String,
    abi: &'static str,
    signature: native_signature::Signature,
}

impl Delegate {
    pub(super) fn lower(
        database: &Database,
        dependencies: &native::DependencyCache,
        definition: TypeDefinition<'_>,
    ) -> Result<Self, Error> {
        let namespace = definition.namespace()?.to_string();
        let name = definition.name()?.to_string();
        let full_name = format!("{namespace}.{name}");
        let methods = definition.methods()?.collect::<Vec<_>>();
        let [method] = methods.as_slice() else {
            return Err(Error::InvalidType {
                name: full_name,
                message: "native delegate does not have one method",
            });
        };
        if method.name()? != "Invoke" {
            return Err(Error::InvalidType {
                name: full_name,
                message: "native delegate method is not Invoke",
            });
        }
        Ok(Self {
            architectures: definition.architectures()?,
            namespace,
            name,
            abi: calling_convention(definition, &full_name)?,
            signature: native_signature::Signature::lower(
                database,
                dependencies,
                *method,
                &full_name,
            )?,
        })
    }

    /// Renders a flat Win32 function-pointer alias.
    #[cfg(test)]
    pub fn write_sys(&self) -> TokenStream {
        self.write_context(Layout::Flat, Projection::Sys)
    }

    #[cfg(test)]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[cfg(test)]
    pub fn write_package(&self) -> TokenStream {
        self.write_context(Layout::Package, Projection::Default)
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
        let name = tokens::ident(&self.name);
        let abi = self.abi;
        let parameters = if projection.is_sys() {
            self.signature
                .write_parameters_projection(&self.namespace, layout, projection)
        } else {
            self.signature
                .write_delegate_parameters_projection(&self.namespace, layout, projection)
        };
        let result = self
            .signature
            .write_result_projection(&self.namespace, layout, projection);
        let ty = quote! { Option<unsafe extern #abi fn(#parameters) #result> };
        quote! {
            #architectures
            #cfg
            pub type #name = #ty;
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

fn calling_convention(
    definition: TypeDefinition<'_>,
    full_name: &str,
) -> Result<&'static str, Error> {
    let Some(attribute) = definition.find_attribute("UnmanagedFunctionPointerAttribute")? else {
        return Ok("system");
    };
    let arguments = attribute.arguments(&FrameworkEnums)?;
    let Some(AttributeArgument::Fixed { value, .. }) = arguments.first() else {
        return Err(Error::InvalidType {
            name: full_name.to_string(),
            message: "delegate calling convention has no argument",
        });
    };
    let value = match value {
        AttributeValue::I32(value) => Some(*value),
        AttributeValue::Enum { value, .. } => match value.as_ref() {
            AttributeValue::I32(value) => Some(*value),
            _ => None,
        },
        _ => None,
    }
    .ok_or_else(|| Error::InvalidType {
        name: full_name.to_string(),
        message: "delegate calling convention is not i32",
    })?;
    match value {
        1 | 5 => Ok("system"),
        2 => Ok("C"),
        _ => Err(Error::UnsupportedType {
            name: full_name.to_string(),
            shape: format!("delegate calling convention {value}"),
        }),
    }
}

struct FrameworkEnums;

impl windows_metadata2::EnumResolver for FrameworkEnums {
    fn enum_backing(
        &self,
        database: &Database,
        ty: &windows_metadata2::EnumType,
    ) -> Option<windows_metadata2::EnumBacking> {
        let windows_metadata2::EnumType::Metadata(identity) = ty else {
            return None;
        };
        match database
            .type_name(identity.file, identity.ty)
            .ok()
            .flatten()?
        {
            ("System.Runtime.InteropServices", "CallingConvention") => {
                Some(windows_metadata2::EnumBacking::I32)
            }
            _ => None,
        }
    }
}
