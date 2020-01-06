// use crate::*;
// use proc_macro2::{Literal, TokenStream};
// use quote::{format_ident, quote};
// use winmd::*;

// // TOOD: make all subsequent write_ functions impls of Writer. The write_namespaces function
// // can then create one Writer for each namespace and then aggregate them all up at the end.
// // Each writer then manages its own generics and shares the limits

// pub(crate) fn write_selection(namespaces: NamespaceSet, limits: &std::collections::BTreeSet<String>) -> TokenStream {
//     let mut tokens = quote! {};
//     let mut writer = Writer { limits, generics: Default::default() };

//     for namespace in namespaces {
//         if writer.limits.contains(namespace.full_name()) {
//             let namespace = writer.write_namespace(&namespace);

//             tokens = quote! {
//                 #tokens
//                 #namespace
//             };
//         }
//     }

//     tokens
// }

// fn to_snake(camel: &str) -> String {
//     let mut result = String::new();

//     for c in camel.chars() {
//         if c.is_uppercase() {
//             if !result.is_empty() {
//                 result.push('_');
//             }
//             for c in c.to_lowercase() {
//                 result.push(c);
//             }
//         } else {
//             result.push(c);
//         }
//     }

//     result
// }

// struct Writer<'a> {
//     pub limits: &'a std::collections::BTreeSet<String>,
//     pub generics: Vec<Vec<String>>,
// }

// impl<'a> Writer<'a> {
//     // TODO: should return scope guard that pops generics stack on drop
//     fn push_generics(&mut self, generics: &[GenericParam]) {
//         let mut vec = Vec::with_capacity(generics.len());
//         for generic in generics {
//             vec.push(generic.name().to_string());
//         }
//         self.generics.push(vec);
//     }

//     fn write_namespaces(&mut self, namespaces: NamespaceSet) -> TokenStream {
//         let mut tokens = quote! {};

//         for namespace in namespaces {
//             if self.limits.contains(namespace.full_name()) {
//                 let namespace = self.write_namespace(&namespace);

//                 tokens = quote! {
//                     #tokens
//                     #namespace
//                 };
//             }
//         }

//         tokens
//     }

//     fn write_namespace(&mut self, namespace: &Namespace) -> TokenStream {
//         let module = format_ident!("{}", namespace.name().to_lowercase());
//         let types = self.write_namespace_types(namespace);
//         let namespaces = self.write_namespaces(namespace.namespaces());

//         quote! {
//             pub mod #module {
//                 #types
//                 #namespaces
//             }
//         }
//     }

//     fn write_namespace_types(&mut self, namespace: &Namespace) -> TokenStream {
//         let mut tokens = Vec::new();

//         for t in namespace.types() {
//             tokens.push(match t.category() {
//                 TypeCategory::Interface => self.write_interface(&t),
//                 TypeCategory::Class => self.write_class(&t),
//                 TypeCategory::Enum => self.write_enum(&t),
//                 TypeCategory::Struct => self.write_struct(&t),
//                 TypeCategory::Delegate => self.write_delegate(&t),
//                 _ => continue,
//             });
//         }

//         TokenStream::from_iter(tokens)
//     }

//     fn write_class(&mut self, class: &TypeDef) -> TokenStream {
//         // TODO: don't define struct here if the class is static - only declare.

//         let name = format_ident!("{}", class.name());
//         let functions = self.write_class_functions(&class);
//         let string_name = format!("{}.{}", class.namespace(), class.name());

//         quote! {
//             pub struct #name { ptr: *mut std::ffi::c_void }
//             impl #name { #functions }
//             impl winrt::TypeName for #name {
//                 fn type_name() -> &'static str {
//                     #string_name
//                 }
//             }
//             impl winrt::AsAbi for #name {
//                 type In = *const std::ffi::c_void;
//                 type Out = *mut *mut std::ffi::c_void;
//                 fn as_abi_in(&self) -> Self::In {
//                     self.ptr
//                 }
//                 fn as_abi_out(&mut self) -> Self::Out {
//                     debug_assert!(self.ptr.is_null());
//                     &mut self.ptr
//                 }
//             }
//             impl From<*mut std::ffi::c_void> for #name {
//                 fn from(ptr: *mut std::ffi::c_void) -> Self {
//                     Self { ptr }
//                 }
//             }
//         }
//     }

//     fn write_class_functions(&mut self, class: &TypeDef) -> TokenStream {
//         let mut tokens = quote! {};

//         for attribute in class.attributes() {
//             let (_, name) = attribute.name();

//             if name == "StaticAttribute" {
//                 for (_, sig) in attribute.arguments() {
//                     if let ArgumentSig::Type(interface) = sig {
//                         let class_name = format_ident!("{}", class.name());
//                         let interface_name = format_ident!("{}", interface.name());

//                         for method in interface.methods() {
//                             let method_name = format_ident!("r#{}", method.name());
//                             let signature = method.signature();
//                             let params = self.write_consume_params(&signature);
//                             let args = signature.params().iter().map(|param| format_ident!("{}", param.name()));

//                             if let Some(result) = signature.return_type() {
//                                 let result = self.write_type(result.sig_type());

//                                 tokens = quote! {
//                                     #tokens
//                                     pub fn #method_name(#params) -> winrt::Result<#result> {
//                                         winrt::factory::<#class_name, #interface_name>()?.#method_name(#(#args),*)
//                                     }
//                                 };
//                             } else {
//                                 tokens = quote! {
//                                     #tokens
//                                     pub fn #method_name(#params) -> winrt::Result<()> {
//                                             panic!();
//                                     }
//                                 };
//                             };
//                         }
//                     }
//                 }
//             }
//         }

//         tokens
//     }

//     fn write_guid(&mut self, t: &TypeDef) -> TokenStream {
//         let guid = t.find_attribute("Windows.Foundation.Metadata.GuidAttribute").unwrap();
//         let args = guid.arguments();

//         let mut iter = args.iter().map(|(_, value)| match value {
//             ArgumentSig::U8(value) => Literal::u8_unsuffixed(*value),
//             ArgumentSig::U16(value) => Literal::u16_unsuffixed(*value),
//             ArgumentSig::U32(value) => Literal::u32_unsuffixed(*value),
//             _ => panic!(),
//         });

//         let three = iter.by_ref().take(3);

//         quote! {
//             #(#three,)* &[#(#iter),*],
//         }
//     }

//     fn write_interface(&mut self, interface: &TypeDef) -> TokenStream {
//         let name = interface.name();
//         let generics = interface.generics().collect::<Vec<GenericParam>>();

//         //let writer = self.push_generics();

//         let (name_ident, abi_name_ident) = if generics.is_empty() { (format_ident!("{}", name), format_ident!("abi_{}", name)) } else { (format_ident!("{}", name), format_ident!("abi_{}", name)) };

//         let abi_methods = self.write_abi_methods(&interface);
//         let consume_methods = self.write_consume_methods(&interface);

//         let guid = self.write_guid(interface);

//         quote! {
//             #[repr(C)]
//             pub struct #name_ident { ptr: *mut std::ffi::c_void }
//             #[repr(C)]
//             struct #abi_name_ident {
//                 __0: usize,
//                 __1: usize,
//                 __2: usize,
//                 __3: usize,
//                 __4: usize,
//                 __5: usize,
//                 #abi_methods
//             }
//             impl #name_ident {
//                 #consume_methods
//             }
//             impl winrt::TypeGuid for #name_ident {
//                 fn type_guid() -> &'static winrt::Guid {
//                     static GUID: winrt::Guid = winrt::Guid::from_values(
//                         #guid
//                     );
//                     &GUID
//                 }
//             }
//             impl winrt::AsAbi for #name_ident {
//                 type In = *const std::ffi::c_void;
//                 type Out = *mut *mut std::ffi::c_void;
//                 fn as_abi_in(&self) -> Self::In {
//                     self.ptr
//                 }
//                 fn as_abi_out(&mut self) -> Self::Out {
//                     debug_assert!(self.ptr.is_null());
//                     &mut self.ptr
//                 }
//             }
//             impl From<*mut std::ffi::c_void> for #name_ident {
//                 fn from(ptr: *mut std::ffi::c_void) -> Self {
//                     Self { ptr }
//                 }
//             }
//         }
//     }

//     fn write_abi_methods(&mut self, interface: &TypeDef) -> TokenStream {
//         let mut tokens = quote! {};
//         println!("{}", interface.name());

//         for method in interface.methods() {
//             let name = format_ident!("r#{}", method.name());
//             let params = self.write_abi_params(&method.signature());
//             tokens = quote! {
//                 #tokens
//                 #name: extern "system" fn(*const std::ffi::c_void, #params) -> winrt::ErrorCode,
//             };
//         }

//         tokens
//     }

//     fn write_consume_methods(&mut self, interface: &TypeDef) -> TokenStream {
//         let mut tokens = quote! {};
//         let abi_interface_name = format_ident!("abi_{}", interface.name());

//         for method in interface.methods() {
//             let name = format_ident!("r#{}", method.name());
//             let signature = method.signature();
//             let params = self.write_consume_params(&signature);
//             let args = self.write_abi_args(&signature);

//             if let Some(result) = signature.return_type() {
//                 let projected_result = self.write_type(result.sig_type());
//                 let receive_result = self.write_consume_receive_type(result.sig_type());

//                 tokens = quote! {
//                     #tokens
//                     pub fn #name(&self, #params) -> winrt::Result<#projected_result> {
//                         unsafe {
//                             #receive_result
//                             ((*(*(self.ptr as *const *const #abi_interface_name))).#name)(
//                                 self.ptr, #args &mut __ok,
//                             )
//                             .ok_or(From::from(__ok))
//                         }
//                     }
//                 };
//             } else {
//                 tokens = quote! {
//                     #tokens
//                     pub fn #name(&self, #params) -> winrt::Result<()> {
//                         unsafe {
//                             panic!();
//                         }
//                     }
//                 };
//             };
//         }

//         tokens
//     }

//     fn write_consume_receive_type(&mut self, value: &TypeSig) -> TokenStream {
//         match value.category() {
//             ParamCategory::Object | ParamCategory::String => quote! {
//                 let mut __ok = std::ptr::null_mut();
//             },
//             ParamCategory::Enum => quote! {
//                 let mut __ok;
//             },
//             _ => quote! {
//                 let mut __ok = Default::default();
//             },
//         }
//     }

//     fn write_enum(&mut self, t: &TypeDef) -> TokenStream {
//         let name = format_ident!("{}", t.name());
//         let fields = self.write_enum_fields(&t);
//         quote! {
//             pub enum #name { #fields }
//         }
//     }

//     fn write_enum_fields(&mut self, t: &TypeDef) -> TokenStream {
//         let mut tokens = quote! {};

//         for f in t.fields() {
//             for _c in f.constants() {
//                 let name = format_ident!("{}", f.name());
//                 //let value = c.value();

//                 tokens = quote! {
//                     #tokens
//                     #name,
//                     // TODO: write out the enum value
//                 };
//             }
//         }

//         tokens
//     }

//     fn write_delegate(&mut self, interface: &TypeDef) -> TokenStream {
//         //let generics = interface.generics();
//         let name = interface.name();
//         let name_ident = format_ident!("{}", name);
//         let abi_name_ident = format_ident!("abi_{}", name);

//         let guid = self.write_guid(interface);

//         quote! {
//             #[repr(C)]
//             pub struct #name_ident { ptr: *mut std::ffi::c_void }
//             #[repr(C)]
//             struct #abi_name_ident {
//                 __0: usize,
//                 __1: usize,
//                 __2: usize,
//             }
//             impl #name_ident {
//             }
//             impl winrt::TypeGuid for #name_ident {
//                 fn type_guid() -> &'static winrt::Guid {
//                     static GUID: winrt::Guid = winrt::Guid::from_values(
//                         #guid
//                     );
//                     &GUID
//                 }
//             }
//             impl winrt::AsAbi for #name_ident {
//                 type In = *const std::ffi::c_void;
//                 type Out = *mut *mut std::ffi::c_void;
//                 fn as_abi_in(&self) -> Self::In {
//                     self.ptr
//                 }
//                 fn as_abi_out(&mut self) -> Self::Out {
//                     debug_assert!(self.ptr.is_null());
//                     &mut self.ptr
//                 }
//             }
//             impl From<*mut std::ffi::c_void> for #name_ident {
//                 fn from(ptr: *mut std::ffi::c_void) -> Self {
//                     Self { ptr }
//                 }
//             }
//         }
//     }

//     fn write_struct(&mut self, t: &TypeDef) -> TokenStream {
//         let name = format_ident!("{}", t.name());
//         let fields = self.write_struct_fields(&t);
//         quote! {
//             #[repr(C)]
//             #[derive(Clone, Default, Debug, PartialEq)]
//             pub struct #name { #fields }
//         }
//     }

//     fn write_struct_fields(&mut self, t: &TypeDef) -> TokenStream {
//         let mut tokens = quote! {};

//         for f in t.fields() {
//             let name = format_ident!("r#{}", to_snake(f.name()));

//             tokens = quote! {
//                 #tokens
//                 pub #name: u8,
//                 // TODO: write out field type
//             };
//         }

//         tokens
//     }

//     //
//     // write_abi_params
//     //

//     fn write_abi_params(&mut self, signature: &MethodSig) -> TokenStream {
//         let mut tokens = Vec::new();

//         for param in signature.params() {
//             tokens.push(self.write_abi_param(param));
//         }

//         if let Some(param) = signature.return_type() {
//             tokens.push(self.write_abi_param(param));
//         }

//         TokenStream::from_iter(tokens)
//     }

//     fn write_abi_param(&mut self, param: &ParamSig) -> TokenStream {
//         match param.sig_type().sig_type() {
//             TypeSigType::ElementType(value) => self.write_abi_param_element_type(param, value),
//             TypeSigType::TypeDefOrRef(value) => self.write_abi_param_type(param, value),
//             TypeSigType::GenericSig(_value) => panic!("abi: GenericSig"),
//             TypeSigType::GenericTypeIndex(_value) => panic!("abi: GenericTypeIndex"),
//         }
//     }

//     fn write_abi_param_element_type(&mut self, param: &ParamSig, value: &ElementType) -> TokenStream {
//         if param.input() {
//             match value {
//                 ElementType::Bool => quote! { bool, },
//                 ElementType::Char => quote! { char, },
//                 ElementType::I8 => quote! { i8, },
//                 ElementType::U8 => quote! { u8, },
//                 ElementType::I16 => quote! { i16, },
//                 ElementType::U16 => quote! { u16, },
//                 ElementType::I32 => quote! { i32, },
//                 ElementType::U32 => quote! { u32, },
//                 ElementType::I64 => quote! { i64, },
//                 ElementType::U64 => quote! { u64, },
//                 ElementType::F32 => quote! { f32, },
//                 ElementType::F64 => quote! { f64, },
//                 ElementType::String => quote! { *const std::ffi::c_void, },
//                 ElementType::Object => quote! { *const std::ffi::c_void, },
//             }
//         } else {
//             match value {
//                 ElementType::Bool => quote! { &mut bool, },
//                 ElementType::Char => quote! { &mut char, },
//                 ElementType::I8 => quote! { &mut i8, },
//                 ElementType::U8 => quote! { &mut u8, },
//                 ElementType::I16 => quote! { &mut i16, },
//                 ElementType::U16 => quote! { &mut u16, },
//                 ElementType::I32 => quote! { &mut i32, },
//                 ElementType::U32 => quote! { &mut u32, },
//                 ElementType::I64 => quote! { &mut i64, },
//                 ElementType::U64 => quote! { &mut u64, },
//                 ElementType::F32 => quote! { &mut f32, },
//                 ElementType::F64 => quote! { &mut f64, },
//                 ElementType::String => quote! { &mut *mut std::ffi::c_void, },
//                 ElementType::Object => quote! { &mut *mut std::ffi::c_void, },
//             }
//         }
//     }

//     fn write_abi_param_type(&mut self, param: &ParamSig, value: &TypeDefOrRef) -> TokenStream {
//         match value {
//             TypeDefOrRef::TypeDef(value) => self.write_abi_param_type_def(param, value),
//             TypeDefOrRef::TypeRef(value) => self.write_abi_param_type_ref(param, value),
//             _ => panic!("write_abi_param_type"),
//         }
//     }

//     fn write_abi_param_type_def(&mut self, param: &ParamSig, value: &TypeDef) -> TokenStream {
//         if param.input() {
//             match value.category() {
//                 TypeCategory::Enum | TypeCategory::Struct => {
//                     let name = format_ident!("{}", value.name());
//                     quote! { #name, }
//                 }
//                 _ => quote! { *const std::ffi::c_void, },
//             }
//         } else {
//             match value.category() {
//                 TypeCategory::Enum | TypeCategory::Struct => {
//                     let name = format_ident!("{}", value.name());
//                     quote! { &mut #name, }
//                 }
//                 _ => quote! { &mut *mut std::ffi::c_void, },
//             }
//         }
//     }

//     fn write_abi_param_type_ref(&mut self, param: &ParamSig, value: &TypeRef) -> TokenStream {
//         if value.name() == "Guid" && value.namespace() == "System" {
//             if param.input() {
//                 quote! { winrt::Guid, }
//             } else {
//                 quote! { &mut winrt::Guid, }
//             }
//         } else {
//             self.write_abi_param_type_def(param, &value.resolve())
//         }
//     }

//     //
//     // write_consume_params
//     //

//     fn write_consume_params(&mut self, signature: &MethodSig) -> TokenStream {
//         let mut tokens = Vec::new();

//         for param in signature.params() {
//             tokens.push(self.write_consume_param(param));
//         }

//         TokenStream::from_iter(tokens)
//     }

//     fn write_consume_param(&mut self, param: &ParamSig) -> TokenStream {
//         match param.sig_type().sig_type() {
//             TypeSigType::ElementType(value) => self.write_consume_param_element_type(param, value),
//             TypeSigType::TypeDefOrRef(value) => self.write_consume_param_type(param, value),
//             TypeSigType::GenericSig(_value) => panic!("consume GenericSig"),
//             TypeSigType::GenericTypeIndex(_value) => panic!("consume GenericTypeIndex"),
//         }
//     }

//     fn write_consume_param_element_type(&mut self, param: &ParamSig, value: &ElementType) -> TokenStream {
//         let name = format_ident!("{}", param.name());

//         if param.input() {
//             match value {
//                 ElementType::Bool => quote! { #name: bool, },
//                 ElementType::Char => quote! { #name: char, },
//                 ElementType::I8 => quote! { #name: i8, },
//                 ElementType::U8 => quote! { #name: u8, },
//                 ElementType::I16 => quote! { #name: i16, },
//                 ElementType::U16 => quote! { #name: u16, },
//                 ElementType::I32 => quote! { #name: i32, },
//                 ElementType::U32 => quote! { #name: u32, },
//                 ElementType::I64 => quote! { #name: i64, },
//                 ElementType::U64 => quote! { #name: u64, },
//                 ElementType::F32 => quote! { #name: f32, },
//                 ElementType::F64 => quote! { #name: f64, },
//                 ElementType::String => quote! { #name: &winrt::String, }, // AsRef<String> e.g. &str/String
//                 ElementType::Object => quote! { #name: &winrt::Object, }, // AsRef<Object> e.g. boxing/polymorphism
//             }
//         } else {
//             match value {
//                 ElementType::Bool => quote! { #name: &mut bool, },
//                 ElementType::Char => quote! { #name: &mut char, },
//                 ElementType::I8 => quote! { #name: &mut i8, },
//                 ElementType::U8 => quote! { #name: &mut u8, },
//                 ElementType::I16 => quote! { #name: &mut i16, },
//                 ElementType::U16 => quote! { #name: &mut u16, },
//                 ElementType::I32 => quote! { #name: &mut i32, },
//                 ElementType::U32 => quote! { #name: &mut u32, },
//                 ElementType::I64 => quote! { #name: &mut i64, },
//                 ElementType::U64 => quote! { #name: &mut u64, },
//                 ElementType::F32 => quote! { #name: &mut f32, },
//                 ElementType::F64 => quote! { #name: &mut f64, },
//                 ElementType::String => quote! { #name: &mut winrt::String, },
//                 ElementType::Object => quote! { #name: &mut winrt::Object, },
//             }
//         }
//     }

//     fn write_consume_param_type(&mut self, param: &ParamSig, value: &TypeDefOrRef) -> TokenStream {
//         match value {
//             TypeDefOrRef::TypeDef(value) => self.write_consume_param_type_def(param, value),
//             TypeDefOrRef::TypeRef(value) => self.write_consume_param_type_ref(param, value),
//             _ => panic!("write_consume_param_type"),
//         }
//     }

//     fn write_consume_param_type_def(&mut self, param: &ParamSig, value: &TypeDef) -> TokenStream {
//         let param_name = format_ident!("{}", param.name());
//         let type_name = format_ident!("{}", value.name());

//         if param.input() {
//             quote! { #param_name: &#type_name, }
//         } else {
//             quote! { #param_name: &mut #type_name, }
//         }
//     }

//     fn write_consume_param_type_ref(&mut self, param: &ParamSig, value: &TypeRef) -> TokenStream {
//         let param_name = format_ident!("{}", param.name());

//         if value.name() == "Guid" && value.namespace() == "System" {
//             if param.input() {
//                 quote! { #param_name: &winrt::Guid, }
//             } else {
//                 quote! { #param_name: &mut winrt::Guid, }
//             }
//         } else {
//             self.write_consume_param_type_def(param, &value.resolve())
//         }
//     }

//     //
//     // write_abi_args
//     //

//     fn write_abi_args(&mut self, signature: &MethodSig) -> TokenStream {
//         let mut tokens = Vec::new();

//         for param in signature.params() {
//             tokens.push(self.write_abi_arg(param));
//         }

//         TokenStream::from_iter(tokens)
//     }

//     fn write_abi_arg(&mut self, param: &ParamSig) -> TokenStream {
//         let name = format_ident!("{}", param.name());
//         let category = param.sig_type().category();

//         if param.input() {
//             match category {
//                 ParamCategory::Enum | ParamCategory::Primitive => quote! { #name, },
//                 ParamCategory::Struct => quote! { #name.clone(), },
//                 _ => quote! { winrt::AsAbi::as_abi_in(#name), },
//             }
//         } else {
//             match category {
//                 ParamCategory::Enum | ParamCategory::Primitive | ParamCategory::Struct => quote! { &mut #name, },
//                 _ => quote! { winrt::AsAbi::as_abi_out(#name), },
//             }
//         }
//     }

//     //
//     // write_type
//     //

//     fn write_type(&mut self, value: &TypeSig) -> TokenStream {
//         match value.sig_type() {
//             TypeSigType::ElementType(value) => self.write_type_element(value),
//             TypeSigType::TypeDefOrRef(value) => self.write_type_def_or_ref(value),
//             TypeSigType::GenericSig(_value) => panic!("type: GenericSig"),
//             TypeSigType::GenericTypeIndex(_value) => panic!("type: GenericTypeIndex"),
//         }
//     }

//     fn write_type_element(&mut self, value: &ElementType) -> TokenStream {
//         match value {
//             ElementType::Bool => quote! { bool },
//             ElementType::Char => quote! { char },
//             ElementType::I8 => quote! { i8 },
//             ElementType::U8 => quote! { u8 },
//             ElementType::I16 => quote! { i16 },
//             ElementType::U16 => quote! { u16 },
//             ElementType::I32 => quote! { i32 },
//             ElementType::U32 => quote! { u32 },
//             ElementType::I64 => quote! { i64 },
//             ElementType::U64 => quote! { u64 },
//             ElementType::F32 => quote! { f32 },
//             ElementType::F64 => quote! { f64 },
//             ElementType::String => quote! { winrt::String },
//             ElementType::Object => quote! { winrt::Object },
//         }
//     }

//     fn write_type_def_or_ref(&mut self, value: &TypeDefOrRef) -> TokenStream {
//         match value {
//             TypeDefOrRef::TypeDef(value) => self.write_type_def(value),
//             TypeDefOrRef::TypeRef(value) => self.write_type_ref(value),
//             _ => panic!("write_type_def_or_ref"),
//         }
//     }

//     fn write_type_def(&mut self, value: &TypeDef) -> TokenStream {
//         let name = format_ident!("{}", value.name());
//         quote! { #name }
//     }

//     fn write_type_ref(&mut self, value: &TypeRef) -> TokenStream {
//         if value.name() == "Guid" && value.namespace() == "System" {
//             quote! { winrt::Guid }
//         } else {
//             self.write_type_def(&value.resolve())
//         }
//     }
// }
