use super::*;

pub struct MethodNames(BTreeMap::<String, u32>);

impl MethodNames {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn add(&mut self, method: &MethodDef) -> TokenStream {
        let name = method.rust_name();
        let overload = self.0.entry(name.to_string()).or_insert(0);
        *overload += 1;
        if *overload > 1 { format!("{}{}", name, overload).into() } else { gen_ident(&name) }
    }
}
