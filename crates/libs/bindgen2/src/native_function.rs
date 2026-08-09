use super::*;
use proc_macro2::TokenStream;
use quote::quote;

/// An owned Win32 function projection.
pub struct Function {
    namespace: String,
    name: String,
    import_name: Option<String>,
    module: String,
    abi: &'static str,
    variadic: bool,
    parameters: Vec<Parameter>,
    return_type: native::Type,
}

struct Parameter {
    name: String,
    ty: native::Type,
}

impl Function {
    pub(super) fn lower(
        database: &Database,
        method: windows_metadata2::MethodDefinition<'_>,
        namespace: &str,
        name: &str,
    ) -> Result<Self, Error> {
        let full_name = format!("{namespace}.{name}");
        let import = method.import()?.ok_or_else(|| Error::InvalidValue {
            name: full_name.clone(),
            message: "Win32 function has no ImplMap",
        })?;
        let MethodSignature {
            flags,
            return_type,
            parameters,
            ..
        } = method.signature()?;
        let parameter_rows = method.parameters_by_sequence()?;
        let parameters = parameters
            .into_iter()
            .enumerate()
            .map(|(position, ty)| {
                Ok(Parameter {
                    name: parameter_rows.parameters()[position]
                        .map(|parameter| parameter.name())
                        .transpose()?
                        .map_or_else(|| format!("p{position}"), str::to_lowercase),
                    ty: native::Type::lower(database, method.entity().file(), &full_name, ty)?,
                })
            })
            .collect::<Result<_, Error>>()?;
        let import_name = (import.name() != name).then(|| import.name().to_string());
        Ok(Self {
            namespace: namespace.to_string(),
            name: name.to_string(),
            import_name,
            module: import.module().to_lowercase(),
            abi: calling_convention(import.flags(), &full_name)?,
            variadic: flags & 0x0f == 0x05,
            parameters,
            return_type: native::Type::lower(
                database,
                method.entity().file(),
                &full_name,
                return_type,
            )?,
        })
    }

    /// Renders a flat Win32 `windows_link::link!` declaration.
    pub fn write(&self) -> TokenStream {
        let module = &self.module;
        let abi = self.abi;
        let symbol = self.import_name.as_ref().map(|name| quote! { #name });
        let name = tokens::ident(&self.name);
        let parameters = self.parameters.iter().map(|parameter| {
            let name = tokens::ident(&parameter.name);
            let ty = parameter.ty.write(&self.namespace);
            quote! { #name: #ty }
        });
        let variadic = self.variadic.then(|| quote! { , ... });
        let result = if self.return_type == native::Type::Void {
            quote! {}
        } else {
            let ty = self.return_type.write(&self.namespace);
            quote! { -> #ty }
        };
        quote! {
            windows_link::link!(#module #abi #symbol fn #name(#(#parameters),* #variadic) #result);
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
