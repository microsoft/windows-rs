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
        method: windows_metadata2::MethodDefinition<'_>,
        namespace: &str,
        name: &str,
    ) -> Result<Self, Error> {
        let full_name = format!("{namespace}.{name}");
        let architectures = method.architectures()?;
        let import = method.import()?.ok_or_else(|| Error::InvalidValue {
            name: full_name.clone(),
            message: "Win32 function has no ImplMap",
        })?;
        let signature = native_signature::Signature::lower(database, method, &full_name)?;
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
    pub fn write_sys(&self) -> TokenStream {
        self.write_sys_context(Layout::Flat)
    }

    pub(super) fn write_sys_context(&self, layout: Layout) -> TokenStream {
        let architectures = tokens::architectures(self.architectures);
        let module = &self.module;
        let abi = self.abi;
        let symbol = self.import_name.as_ref().map(|name| quote! { #name });
        let name = tokens::ident(&self.name);
        let parameters = self.signature.write_parameters(&self.namespace, layout);
        let variadic = self.variadic.then(|| quote! { , ... });
        let result = self.signature.write_result(&self.namespace, layout);
        quote! {
            #architectures
            windows_link::link!(#module #abi #symbol fn #name(#parameters #variadic) #result);
        }
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
        _ => Err(Error::InvalidValue {
            name: name.to_string(),
            message: "invalid P/Invoke calling convention",
        }),
    }
}
