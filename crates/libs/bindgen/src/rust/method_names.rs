use super::*;
use metadata::HasAttributes;

pub struct MethodNames(std::collections::BTreeMap<String, u32>);

impl MethodNames {
    pub fn new() -> Self {
        Self(std::collections::BTreeMap::new())
    }

    pub fn add(&mut self, method: metadata::MethodDef) -> TokenStream {
        let name = method_def_special_name(method);
        let overload = self.0.entry(name.to_string()).or_insert(0);
        *overload += 1;
        if *overload > 1 {
            format!("{name}{overload}").into()
        } else {
            to_ident(&name)
        }
    }

    pub fn add_vtable_types(&mut self, def: metadata::TypeDef) {
        for def in metadata::type_def_vtables(def) {
            if let metadata::Type::TypeDef(def, _) = def {
                for method in def.methods() {
                    self.add(method);
                }
            }
        }
    }
}

fn method_def_special_name(row: metadata::MethodDef) -> String {
    let name = row.name();
    if row.flags().contains(metadata::MethodAttributes::SpecialName) {
        if name.starts_with("get") {
            name[4..].to_string()
        } else if name.starts_with("put") {
            format!("Set{}", &name[4..])
        } else if name.starts_with("add") {
            name[4..].to_string()
        } else if name.starts_with("remove") {
            format!("Remove{}", &name[7..])
        } else {
            name.to_string()
        }
    } else {
        if let Some(attribute) = row.find_attribute("OverloadAttribute") {
            for (_, arg) in attribute.args() {
                if let metadata::Value::String(name) = arg {
                    return name;
                }
            }
        }
        name.to_string()
    }
}
