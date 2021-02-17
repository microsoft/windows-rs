use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Callback(pub tables::TypeDef);

impl Callback {
    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        self.0
            .methods()
            .filter_map(|m| {
                if m.name() == "Invoke" {
                    Some(m.dependencies(&[]))
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    pub fn gen(&self, _: Gen) -> TokenStream {
        quote! {}
    }
}

// #[derive(Debug)]
// pub struct Callback {
//     pub name: TypeName,
//     pub signature: MethodSignature,
// }

// impl Callback {
//     pub fn from_type_name(name: TypeName) -> Self {
//         let method = name
//             .def
//             .methods()
//             .find(|method| method.name() == "Invoke")
//             .unwrap();

//         let signature = MethodSignature::new(&method, &[], &name.namespace);
//         Self { name, signature }
//     }

//     pub fn dependencies(&self) -> Vec<winmd::TypeDef> {
//         self.signature.dependencies()
//     }

//     pub fn gen(&self) -> TokenStream {
//         let name = self.name.gen();

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

//         quote! {
//             #[allow(non_camel_case_types)]
//             pub type #name = extern "system" fn(#(#params),*) #return_type;
//         }
//     }
// }
