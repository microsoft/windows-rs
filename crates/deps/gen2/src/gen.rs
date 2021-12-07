use super::*;
use std::collections::*;

#[derive(Default)]
pub struct Gen<'a> {
    pub namespace: &'a str,
    pub inherit: bool, // generated inherited methods
    pub sys: bool,
    pub flatten: bool,
    pub cfg: bool,
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

    pub(crate) fn arch_cfg(&self, attributes: impl Iterator<Item = Attribute>) -> TokenStream {
        for attribute in attributes {
            if attribute.name() == "SupportedArchitectureAttribute" {
                if let Some((_, ConstantValue::I32(value))) = attribute.args().get(0) {
                    let mut cfg = "#[cfg(any(".to_string();
                    if value & 1 == 1 {
                        cfg.push_str(r#"target_arch = "x86", "#);
                    }
                    if value & 2 == 2 {
                        cfg.push_str(r#"target_arch = "x86_64", "#);
                    }
                    if value & 4 == 4 {
                        cfg.push_str(r#"target_arch = "aarch64", "#);
                    }
                    cfg.push_str("))]");
                    return cfg.into();
                }
            }
        }

        quote! {}
    }

    pub(crate) fn type_cfg(&self, def: &TypeDef) -> TokenStream {
        if !self.cfg {
            quote! {}
        } else {
            let mut namespaces = BTreeSet::new();
            let mut keys = HashSet::new();
            self.type_requirements(def, &mut namespaces, &mut keys);
            cfg(&namespaces)
        }
    }

    pub(crate) fn field_cfg(&self, def: &Field) -> TokenStream {
        if !self.cfg {
            quote! {}
        } else {
            let mut namespaces = BTreeSet::new();
            let mut keys = HashSet::new();
            self.field_requirements(def, None, &mut namespaces, &mut keys);
            cfg(&namespaces)
        }
    }

    pub(crate) fn function_cfg(&self, def: &MethodDef) -> (TokenStream, TokenStream) {
        if !self.cfg {
            (quote! {}, quote! {})
        } else {
            let mut namespaces = BTreeSet::new();
            let mut keys = HashSet::new();
            self.method_requirements(&def.signature(&[]), &mut namespaces, &mut keys);
            (cfg(&namespaces), cfg_not(&namespaces))
        }
    }

    pub(crate) fn method_cfg(&self, def: &TypeDef, method: &MethodDef) -> (TokenStream, TokenStream) {
        if !self.cfg {
            (quote! {}, quote! {})
        } else {
            let mut namespaces = BTreeSet::new();
            let mut keys = HashSet::new();
            self.add_namespace(def.namespace(), &mut namespaces);
            self.method_requirements(&method.signature(&[]), &mut namespaces, &mut keys);
            (cfg(&namespaces), cfg_not(&namespaces))
        }
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

fn cfg(namespaces: &BTreeSet<&'static str>) -> TokenStream {
    if namespaces.is_empty() {
        quote! {}
    } else {
        format!("#[cfg({})]", namespaces_to_features(namespaces)).into()
    }
}

fn cfg_not(namespaces: &BTreeSet<&'static str>) -> TokenStream {
    if namespaces.is_empty() {
        quote! {}
    } else {
        format!("#[cfg(not({}))]", namespaces_to_features(namespaces)).into()
    }
}

fn namespaces_to_features(namespaces: &BTreeSet<&'static str>) -> String {
    let mut features = String::new();

    for namespace in namespaces {
        features.push_str("feature = \"");

        for namespace in namespace.split('.').skip(1) {
            features.push_str(namespace);
            features.push('_');
        }

        features.truncate(features.len() - 1);
        features.push_str("\",")
    }

    features.truncate(features.len() - 1);

    if namespaces.len() > 1 {
        format!("all({})", features)
    } else {
        features
    }
}
