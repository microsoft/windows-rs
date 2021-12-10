use super::*;
use std::collections::*;

#[derive(Default)]
pub struct Gen<'a> {
    pub namespace: &'a str,
    pub sys: bool,
    pub flatten: bool,
    pub cfg: bool,
    pub doc: bool,
    pub min_enum: bool,
    pub min_inherit: bool,
    pub min_xaml: bool,
}

impl Gen<'_> {
    pub(crate) fn namespace(&self, namespace: &str) -> TokenStream {
        if self.flatten || namespace == self.namespace {
            quote! {}
        } else {
            let mut relative = self.namespace.split('.').peekable();
            let mut namespace = namespace.split('.').peekable();

            while relative.peek() == namespace.peek() {
                if relative.next().is_none() {
                    break;
                }

                namespace.next();
            }

            let mut tokens = TokenStream::new();

            for _ in 0..relative.count() {
                tokens.push_str("super::");
            }

            for namespace in namespace {
                tokens.push_str(namespace);
                tokens.push_str("::");
            }

            tokens
        }
    }

    pub(crate) fn element_cfg(&self, def: &ElementType) -> Cfg {
        if let ElementType::TypeDef(def) = def {
            self.type_cfg(def)
        } else {
            Default::default()
        }
    }

    pub(crate) fn type_cfg(&self, def: &TypeDef) -> Cfg {
        let mut features = BTreeSet::new();
        let mut keys = HashSet::new();
        self.type_requirements(def, &mut features, &mut keys);

        if def.is_deprecated() {
            features.insert("deprecated");
        }

        Cfg { arch: arch(def.attributes()), features }
    }

    pub(crate) fn field_cfg(&self, def: &Field) -> Cfg {
        let mut features = BTreeSet::new();
        let mut keys = HashSet::new();
        self.field_requirements(def, None, &mut features, &mut keys);
        Cfg { arch: Default::default(), features }
    }

    pub(crate) fn function_cfg(&self, method: &MethodDef) -> Cfg {
        let mut features = BTreeSet::new();
        let mut keys = HashSet::new();
        self.method_requirements(&method.signature(&[]), &mut features, &mut keys);
        Cfg { arch: arch(method.attributes()), features }
    }

    pub(crate) fn method_cfg(&self, def: &TypeDef, method: &MethodDef) -> Cfg {
        let mut features = BTreeSet::new();
        let mut keys = HashSet::new();
        self.add_namespace(def.namespace(), &mut features);
        self.method_requirements(&method.signature(&[]), &mut features, &mut keys);

        if method.is_deprecated() {
            features.insert("deprecated");
        }

        Cfg { arch: Default::default(), features }
    }

    fn add_namespace(&self, namespace: &'static str, namespaces: &mut BTreeSet<&'static str>) {
        if !namespace.is_empty() && namespace != self.namespace {
            //namespaces.insert(namespace);

            // TODO: use the above instead to iclude parent dependencies
            if !self.namespace.starts_with(format!("{}.", namespace).as_str()) {
                namespaces.insert(namespace);
            }
        }
    }

    fn element_requirements(&self, def: &ElementType, namespaces: &mut BTreeSet<&'static str>, keys: &mut HashSet<Row>) {
        match def {
            ElementType::TypeDef(def) => self.type_requirements(def, namespaces, keys),
            ElementType::Array((signature, _)) => self.element_requirements(&signature.kind, namespaces, keys),
            _ => {}
        }
    }

    fn type_requirements(&self, def: &TypeDef, namespaces: &mut BTreeSet<&'static str>, keys: &mut HashSet<Row>) {
        if !keys.insert(def.row.clone()) {
            return;
        }

        self.add_namespace(def.namespace(), namespaces);

        for generic in &def.generics {
            self.element_requirements(generic, namespaces, keys);
        }

        match def.kind() {
            TypeKind::Class => {
                if let Some(def) = def.default_interface() {
                    self.add_namespace(def.namespace(), namespaces);
                }
            }
            TypeKind::Struct => {
                def.fields().for_each(|field| self.field_requirements(&field, Some(def), namespaces, keys));

                // TODO: needed?
                if let Some(def) = def.is_convertible_to() {
                    self.add_namespace(def.type_name().namespace, namespaces);
                }
            }
            TypeKind::Delegate => self.method_requirements(&def.invoke_method().signature(&[]), namespaces, keys),
            _ => {}
        }

        if let Some(entry) = TypeReader::get().get_type_entry(def.type_name()) {
            for def in &entry.def {
                if let ElementType::TypeDef(def) = def {
                    self.type_requirements(def, namespaces, keys);
                }
            }
        }
    }

    fn method_requirements(&self, def: &MethodSignature, namespaces: &mut BTreeSet<&'static str>, keys: &mut HashSet<Row>) {
        def.return_sig.iter().for_each(|def| self.element_requirements(&def.kind, namespaces, keys));
        def.params.iter().for_each(|def| self.element_requirements(&def.signature.kind, namespaces, keys));
    }

    fn field_requirements(&self, def: &Field, enclosing: Option<&TypeDef>, namespaces: &mut BTreeSet<&'static str>, keys: &mut HashSet<Row>) {
        self.element_requirements(&def.signature(enclosing).kind, namespaces, keys);
    }
}

fn arch(attributes: impl Iterator<Item = Attribute>) -> BTreeSet<&'static str> {
    let mut set = BTreeSet::new();

    for attribute in attributes {
        if attribute.name() == "SupportedArchitectureAttribute" {
            if let Some((_, ConstantValue::I32(value))) = attribute.args().get(0) {
                if value & 1 == 1 {
                    set.insert("x86");
                }
                if value & 2 == 2 {
                    set.insert("x86_64");
                }
                if value & 4 == 4 {
                    set.insert("aarch64");
                }
            }
            break;
        }
    }

    set
}
