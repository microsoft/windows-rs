use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Function(pub tables::MethodDef);

impl Function {
    pub fn gen_name(&self) -> TokenStream {
        let name = format_ident!("{}", self.0.name());
        quote! { #name }
    }

    pub fn gen(&self, _: Gen) -> TokenStream {
        quote! {}
    }

    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        self.0.dependencies(&[])
    }
}

// // TODO: move winmd into gen crate to improve inlining and simplify
// #[derive(Debug)]
// pub struct Function {
//     pub signature: MethodSignature,
// }

// impl Function {
//     pub fn new(method: &winmd::MethodDef, calling_namespace: &'static str) -> Self {
//         let signature = MethodSignature::new(method, &[], calling_namespace);
//         Self { signature }
//     }

//     pub fn gen(&self) -> TokenStream {
//         let name = self.signature.method.name();
//         let name = to_ident(name);

//         let params = self.signature.params.iter().map(|t| {
//             let name = to_ident(&t.name);
//             let tokens = t.gen_field();
//             quote! { #name: #tokens }
//         });

//         let return_type = if let Some(t) = &self.signature.return_type {
//             let tokens = t.gen_field();
//             quote! { -> #tokens }
//         } else {
//             TokenStream::new()
//         };

//         let mut link = self.signature.method.impl_map().unwrap().scope().name();

//         // TODO: workaround for https://github.com/microsoft/windows-rs/issues/463
//         if link.contains("-ms-win-") || link == "D3DCOMPILER_47" {
//             link = "onecoreuap";
//         }

//         quote! {
//             #[link(name = #link)]
//             extern "system" {
//                 pub fn #name(#(#params),*) #return_type;
//             }
//         }
//     }

//     pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
//         self.signature.dependencies()
//     }
// }
