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
        definition: TypeDefinition<'_>,
    ) -> Result<Self, Error> {
        let namespace = definition.namespace()?.to_string();
        let name = definition.name()?.to_string();
        let full_name = format!("{namespace}.{name}");
        let methods = definition.methods()?.collect::<Vec<_>>();
        let [method] = methods.as_slice() else {
            return Err(Error::InvalidValue {
                name: full_name,
                message: "native delegate does not have one method",
            });
        };
        if method.name()? != "Invoke" {
            return Err(Error::InvalidValue {
                name: full_name,
                message: "native delegate method is not Invoke",
            });
        }
        Ok(Self {
            architectures: definition.architectures()?,
            namespace,
            name,
            abi: calling_convention(definition, &full_name)?,
            signature: native_signature::Signature::lower(database, *method, &full_name)?,
        })
    }

    /// Renders a flat Win32 function-pointer alias.
    #[cfg(test)]
    pub fn write_sys(&self) -> TokenStream {
        self.write_sys_context(Layout::Flat)
    }

    pub(super) fn write_sys_context(&self, layout: Layout) -> TokenStream {
        let architectures = tokens::architectures(self.architectures);
        let name = tokens::ident(&self.name);
        let abi = self.abi;
        let parameters = self.signature.write_parameters(&self.namespace, layout);
        let result = self.signature.write_result(&self.namespace, layout);
        let ty = quote! { Option<unsafe extern #abi fn(#parameters) #result> };
        quote! {
            #architectures
            pub type #name = #ty;
        }
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
        return Err(Error::InvalidValue {
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
    .ok_or_else(|| Error::InvalidValue {
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
