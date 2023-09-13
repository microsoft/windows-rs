use super::*;

pub struct MethodNames(BTreeMap<String, u32>);

impl MethodNames {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn add(&mut self, writer: &Writer, method: MethodDef) -> TokenStream {
        let name = method_def_special_name(writer.reader, method);
        let overload = self.0.entry(name.to_string()).or_insert(0);
        *overload += 1;
        if *overload > 1 {
            format!("{name}{overload}").into()
        } else {
            to_ident(&name)
        }
    }

    pub fn add_vtable_types(&mut self, writer: &Writer, def: TypeDef) {
        for def in type_def_vtables(writer.reader, def) {
            if let Type::TypeDef(def, _) = def {
                for method in writer.reader.type_def_methods(def) {
                    self.add(writer, method);
                }
            }
        }
    }
}

fn method_def_special_name(reader: &Reader, row: MethodDef) -> String {
    let name = reader.method_def_name(row);
    if reader.method_def_flags(row).contains(MethodAttributes::SpecialName) {
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
        if let Some(attribute) = reader.find_attribute(row, "OverloadAttribute") {
            for (_, arg) in reader.attribute_args(attribute) {
                if let Value::String(name) = arg {
                    return name;
                }
            }
        }
        name.to_string()
    }
}
