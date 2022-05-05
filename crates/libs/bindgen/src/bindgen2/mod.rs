mod com_methods;
mod enums;
mod extensions;
mod gen;
mod handles;
mod interfaces;
mod method_names;
mod replacements;
mod structs;
mod winrt_methods;
mod classes;
mod constants;
pub use gen::*;
use metadata::reader2::*;
use method_names::*;
use std::collections::*;
use tokens::*;

pub fn define(gen: &Gen, name: &str) -> String {
    let mut tokens = String::new();
    let type_name = TypeName::parse(name);

    for def in gen.reader.get(type_name) {
        tokens.push_str(gen.define(def).as_str());
    }

    if tokens.is_empty() {
        if let Some(apis) = gen.reader.get(TypeName::new(type_name.namespace, "Apis")).next() {
            for method in gen.reader.type_def_methods(apis) {
                if gen.reader.method_def_name(method) == type_name.name {
                    tokens.push_str(gen.define_function(method).as_str());
                }
            }
            if tokens.is_empty() {
                for field in gen.reader.type_def_fields(apis) {
                    if gen.reader.field_name(field) == type_name.name {
                        tokens.push_str(constants::gen(gen, field).as_str());
                    }
                }
            }
        }
    }

    assert!(!tokens.is_empty(), "`{}` not found", name);
    tokens
}

pub fn namespace(_gen: &Gen) -> String {
    String::new()
}

pub fn namespace_impl(_gen: &Gen) -> String {
    String::new()
}
