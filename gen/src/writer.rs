use crate::*;

struct Namespace {
    types: Vec<Type>,
    namespaces: BTreeMap<String, Namespace>,
}

pub struct Writer {
    reader: Reader,
    //limits: BTreeSet<String>,
    namespaces: BTreeMap<String, Namespace>,
}

impl Writer {
    fn new() -> Writer {
        let reader = Reader::from_os();
        let namespaces = Default::default();
        let mut writer = Writer {reader, namespaces};

        writer.add_namespace("Windows.Foundation");
        writer.add_namespace("Windows.Foundation.Collections");
        writer.add_namespace("Windows.Foundation.Numerics");

        writer
    }


    fn add_type_name(&mut self, name: &TypeName) {

    }

    fn add_type_def(&mut self, def: &TypeDef) {

    }

    fn add_namespace(&mut self, namespace: &str) {
        if !self.namespaces.contains_key(namespace) {
            //self.reader.namespace_types(namespace).for_each(|def|self.add_type_def(def));
        }
    }
}
