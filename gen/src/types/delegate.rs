use crate::*;

#[derive(Debug)]
pub struct Delegate {
    pub name: TypeName,
    pub method: Method,
}

impl Delegate {
    pub fn from_type_def(reader: &Reader, def: TypeDef) -> Self {
        let name = TypeName::from_type_def(reader, def);
        let method = def
            .methods(reader)
            .find(|method| method.name(reader) == "Invoke")
            .unwrap();
        let method = Method::from_method_def(reader, method, &name.generics);
        Self { name, method }
    }

    pub fn to_stream(&self) -> TokenStream {
        panic!();
    }
    
    pub fn dependencies(&self) -> Vec<TypeDef> {
        Vec::new()
    }


}
