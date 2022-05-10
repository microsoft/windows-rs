use super::*;

pub fn gen(gen: &Gen, def: TypeDef) -> TokenStream {
    " ".into()
}
//     if gen.reader.type_def_kind(def) != TypeKind::Interface || (!gen.component && !gen.reader.type_def_can_implement(def)) {
//         return quote! {};
//     }

//     let name = gen.reader.type_def_name(def);

//     if name.starts_with("Disp") && gen.reader.type_def_methods(def).next().is_none() {
//         return quote! {};
//     }

//     let type_ident = to_ident(name);
//     let impl_ident = type_ident.join("_Impl");
//     let vtbl_ident = type_ident.join("_Vtbl");
//     let constraints = gen.type_constraints(def, gen);
//     let generics = gen.type_generics(def, gen);
//     let phantoms = gen.named_phantoms(def, gen);
//     let cfg = gen.reader.type_def_impl_cfg(def);
//     let mut requires = quote! {};

//     fn gen_required_trait(gen: &Gen, def: TypeDef) -> TokenStream {
//         let name = gen.type_def_name_imp(def, "_Impl");
//         let namespace = gen.namespace(gen.reader.type_def_namespace(def));
//         quote! {
//             + #namespace #name
//         }
//     }

//     let mut matches = quote! { iid == &<#type_ident<#(#generics)*> as ::windows::core::Interface>::IID };

//     for def in gen.reader.type_def_vtables(def) {
//         if let Type::TypeDef(def) = def {
//             requires.combine(&gen_required_trait(gen, def));

//             let name = to_ident(gen.reader.type_def_name(def));
//             let namespace = gen.namespace(gen.reader.type_def_namespace(def));

//             matches.combine(&quote! {
//                 || iid == &<#namespace #name as ::windows::core::Interface>::IID
//             })
//         }
//     }

//     if gen.reader.type_def_flags(def).winrt() {
//         for def in gen.reader.type_def_interfaces(def) {
//             requires.combine(&gen_required_trait(&def, gen));
//         }
//     }

//     let runtime_name = gen.runtime_name(def, &cfg, gen);
//     let cfg = gen.cfg(&cfg);
//     let mut method_names = MethodNames::new();
//     method_names.add_vtable_types(def);

//     let method_traits = gen.reader.type_def_methods(def).map(|method| {
//         let name = method_names.add(&method);
//         let signature = gen_impl_signature(def, &method, gen);
//         // If it can be implemented but is exclusive and has no return value then
//         // it is a Xaml override so give it a default implementation to make it easier
//         // to override individual methods for Xaml notifications.
//         if !gen.component && def.is_exclusive() && method.signature(&def.generics).return_type.is_none() {
//             quote! {
//                 fn #name #signature {
//                     ::core::result::Result::Ok(())
//                 }
//             }
//         } else {
//             quote! { fn #name #signature; }
//         }
//     });

//     let mut method_names = MethodNames::new();
//     method_names.add_vtable_types(def);

//     let method_impls = def.methods().map(|method| {
//         let name = method_names.add(&method);
//         let signature = method.signature(&def.generics);
//         let vtbl_signature = gen_vtbl_signature(def, &method, gen);

//         let invoke_upcall = if def.is_winrt() { gen_winrt_upcall(&signature, quote! { this.#name }) } else { gen_win32_upcall(&signature, quote! { this.#name }) };

//         quote! {
//             unsafe extern "system" fn #name<#(#constraints)* Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: #impl_ident<#(#generics)*>, const OFFSET: isize> #vtbl_signature {
//                 // offset the `this` pointer by `OFFSET` times the size of a pointer and cast it as an IUnknown implementation
//                 let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
//                 let this = (*this).get_impl();
//                 #invoke_upcall
//             }
//         }
//     });

//     let mut methods = quote! {};

//     match def.vtable_types().last() {
//         Some(Type::IUnknown) => methods.combine(&quote! { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), }),
//         Some(Type::IInspectable) => methods.combine(&quote! { base__: ::windows::core::IInspectableVtbl::new::<Identity, #type_ident<#(#generics)*>, OFFSET>(), }),
//         Some(Type::TypeDef(def)) => {
//             let vtbl = gen_vtbl_ident(def, gen);
//             let namespace = gen.namespace(def.namespace());
//             methods.combine(&quote! { base__: #namespace #vtbl::new::<Identity, Impl, OFFSET>(), });
//         }
//         _ => {}
//     }

//     let mut method_names = MethodNames::new();
//     method_names.add_vtable_types(def);

//     for method in def.methods() {
//         let name = method_names.add(&method);
//         methods.combine(&quote! { #name: #name::<#(#generics)* Identity, Impl, OFFSET>, });
//     }

//     quote! {
//         #cfg
//         pub trait #impl_ident<#(#generics)*> : Sized #requires where #(#constraints)* {
//             #(#method_traits)*
//         }
//         #runtime_name
//         #cfg
//         impl<#(#constraints)*> #vtbl_ident<#(#generics)*> {
//             pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: #impl_ident<#(#generics)*>, const OFFSET: isize>() -> #vtbl_ident<#(#generics)*> {
//                 #(#method_impls)*
//                 Self{
//                     #methods
//                     #(#phantoms)*
//                 }
//             }
//             pub fn matches(iid: &windows::core::GUID) -> bool {
//                 #matches
//             }
//         }
//     }
// }
