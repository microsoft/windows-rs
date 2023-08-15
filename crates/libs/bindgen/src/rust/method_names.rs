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
