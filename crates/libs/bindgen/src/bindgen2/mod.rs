// mod r#async;
mod callbacks;
mod classes;
mod constants;
mod delegates;
mod enums;
// mod extensions;
mod functions;
mod gen;
// mod handles;
// mod helpers;
// mod implements;
mod interfaces;
// mod iterator;
// mod method_names;
// mod methods;
// mod names;
// mod replacements;
// mod signatures;
mod structs;

// use functions::*;
pub use gen::*;
// use helpers::*;
// use iterator::*;
use metadata::reader2::*;
// use method_names::*;
// use methods::*;
// use names::*;
// use r#async::*;
// use signatures::*;
use tokens::*;

pub fn gen_type(name: &str, gen: &Gen) -> String {
    let mut tokens = String::new();
    let type_name = TypeName::parse(name);

    for def in gen.reader.get(type_name) {
        tokens.push_str(gen_type_def(def, gen).as_str());
    }

    if tokens.is_empty() {
        if let Some(apis) = gen.reader.get(TypeName::new(type_name.namespace, "Apis")).next() {
            for method in gen.reader.type_def_methods(apis) {
                if gen.reader.method_def_name(method) == type_name.name {
                    tokens.push_str(functions::gen_function(method, gen).as_str());
                }
            }
            for field in gen.reader.type_def_fields(apis) {
                if gen.reader.field_name(field) == type_name.name {
                    tokens.push_str(constants::gen(field, gen).as_str());
                }
            }
        }
    }

    assert!(!tokens.is_empty(), "`{}` not found", name);
    tokens
}

// pub fn gen_namespace(gen: &Gen) -> String {
//     let tree = TypeReader::get().get_namespace(gen.namespace).expect("Namespace not found");

//     let namespaces = tree.namespaces.iter().map(move |(name, tree)| {
//         if tree.namespace == "Windows.Win32.Interop" {
//             return quote! {};
//         }

//         let name = gen_ident(name);
//         let namespace = tree.namespace[tree.namespace.find('.').unwrap() + 1..].replace('.', "_");
//         if gen.cfg {
//             quote! {
//                 #[cfg(feature = #namespace)] pub mod #name;
//             }
//         } else {
//             quote! {
//                 pub mod #name;
//             }
//         }
//     });

//     let functions = gen_sys_functions(tree, gen);
//     let types = gen_non_sys_function_types(tree, gen);

//     let tokens = quote! {
//         #![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
//         #(#namespaces)*
//         #functions
//         #types
//     };

//     tokens.into_string()
// }

// pub fn gen_namespace_impl(gen: &Gen) -> String {
//     let tree = TypeReader::get().get_namespace(gen.namespace).expect("Namespace not found");
//     let mut tokens = TokenStream::new();

//     for entry in tree.types.values() {
//         for def in entry {
//             if let Type::TypeDef(def) = def {
//                 let def = &def.clone().with_generics();
//                 tokens.combine(&implements::gen(def, gen));
//             }
//         }
//     }

//     tokens.into_string()
// }

// fn gen_non_sys_function_types(tree: &TypeTree, gen: &Gen) -> TokenStream {
//     let mut tokens = TokenStream::new();

//     for entry in tree.types.values() {
//         for def in entry {
//             tokens.combine(&gen_type_imp(def, gen));
//         }
//     }

//     tokens
// }

fn gen_type_def(def: TypeDef, gen: &Gen) -> TokenStream {
    match gen.reader.type_def_kind(def) {
        TypeKind::Class => classes::gen(def, gen),
        TypeKind::Interface => interfaces::gen(def, gen),
        TypeKind::Enum => enums::gen(def, gen),
        TypeKind::Struct => structs::gen(def, gen),
        TypeKind::Delegate => {
            if gen.reader.type_def_flags(def).winrt() {
                delegates::gen(def, gen)
            } else {
                callbacks::gen(def, gen)
            }
        }
    }
}
