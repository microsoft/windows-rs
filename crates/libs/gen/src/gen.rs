use super::*;

pub struct Gen {
    pub relative: &'static str,
    pub root: &'static str,
    pub ignore_windows_features: bool,
    pub docs: bool,
    pub build: bool,
}

impl Gen {
    pub fn absolute() -> Self {
        Gen { relative: "", root: "", ignore_windows_features: false, docs: false, build: false }
    }

    pub fn relative(namespace: &'static str) -> Self {
        Gen { relative: namespace, root: "", ignore_windows_features: false, docs: false, build: false }
    }

    pub fn build(namespace: &'static str, redirect: bool) -> Self {
        Gen { relative: namespace, root: "", ignore_windows_features: false, docs: false, build: redirect }
    }

    pub fn namespace(&self, namespace: &str) -> TokenStream {
        if self.relative.is_empty() {
            let mut tokens = TokenStream::new();

            for namespace in namespace.split('.') {
                tokens.push_str(namespace);
                tokens.push_str("::");
            }

            tokens
        } else {
            if namespace == self.relative {
                return TokenStream::new();
            }

            if (self.build || (!self.root.is_empty() && self.root != "Windows")) && namespace.starts_with("Windows.") {
                let mut tokens: TokenStream = "::windows::".into();

                for namespace in namespace.split('.').skip(1) {
                    tokens.push_str(namespace);
                    tokens.push_str("::");
                }

                return tokens;
            }

            let mut relative = self.relative.split('.').peekable();
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

    pub fn class_features(&self, def: &TypeDef) -> BTreeSet<&'static str> {
        if self.root.is_empty() {
            return BTreeSet::new();
        }

        let mut features = BTreeSet::new();

        if let Some(def) = def.default_interface() {
            features.insert(def.namespace());
        }

        features
    }

    pub fn gen_function_cfg(&self, attributes: impl Iterator<Item = Attribute>, signature: &MethodSignature) -> TokenStream {
        let arch = self.gen_arch_cfg(attributes);
        let features = signature.method_features();
        let cfg = self.gen_cfg(&features);
        let doc = self.gen_cfg_doc(&features);

        quote! { #doc #arch #cfg }
    }

    pub fn gen_struct_cfg(&self, def: &TypeDef, features: &BTreeSet<&'static str>) -> TokenStream {
        let arch = self.gen_arch_cfg(def.attributes());
        let cfg = self.gen_cfg_impl(features, false);

        quote! { #arch #cfg }
    }

    pub fn gen_cfg_doc(&self, features: &BTreeSet<&'static str>) -> TokenStream {
        if !self.docs || self.root.is_empty() {
            return TokenStream::new();
        }

        let mut tokens = format!("`{}`", self.relative[self.root.len() + 1..].replace('.', "_"));

        for feature in features {
            if self.relative == *feature {
                continue;
            }

            if self.relative.len() > feature.len() && self.relative.starts_with(feature) && self.relative[feature.len()..].starts_with('.') {
                continue;
            }

            if self.ignore_windows_features && feature.starts_with("Windows.") {
                continue;
            }

            let feature = if feature.starts_with(self.root) && feature[self.root.len()..].starts_with('.') { &feature[self.root.len() + 1..] } else { feature };
            tokens.push_str(&format!(", `{}`", feature.replace('.', "_")));
        }

        format!(r#"#[doc = "*Required features: {}*"]"#, tokens).into()
    }

    fn gen_arch_cfg(&self, attributes: impl Iterator<Item = Attribute>) -> TokenStream {
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

        TokenStream::new()
    }

    pub fn gen_cfg(&self, features: &BTreeSet<&'static str>) -> TokenStream {
        self.gen_cfg_impl(features, false)
    }

    pub fn gen_cfg_not(&self, features: &BTreeSet<&'static str>) -> TokenStream {
        self.gen_cfg_impl(features, true)
    }

    fn gen_cfg_impl(&self, features: &BTreeSet<&'static str>, not: bool) -> TokenStream {
        if self.root.is_empty() {
            return TokenStream::new();
        }

        if features.is_empty() {
            return TokenStream::new();
        }

        let mut tokens = String::new();
        let mut count = 0;

        for feature in features {
            if self.relative == *feature {
                continue;
            }

            if self.relative.len() > feature.len() && self.relative.starts_with(feature) && self.relative[feature.len()..].starts_with('.') {
                continue;
            }

            if self.ignore_windows_features && feature.starts_with("Windows.") {
                continue;
            }

            let feature = if feature.starts_with(self.root) && feature[self.root.len()..].starts_with('.') { &feature[self.root.len() + 1..] } else { feature };
            tokens.push_str(&format!("feature = \"{}\", ", feature.replace('.', "_")));
            count += 1;
        }

        if count == 0 {
            return TokenStream::new();
        }

        let all = count > 1;
        let mut cfg = "#[cfg(".to_string();

        if not {
            cfg.push_str("not(")
        }

        if all {
            cfg.push_str("all(")
        }

        tokens.truncate(tokens.len() - 2);
        cfg.push_str(&tokens);

        if not {
            cfg.push(')');
        }

        if all {
            cfg.push(')');
        }

        cfg.push_str(")]");
        cfg.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace() {
        assert_eq!(Gen::absolute().namespace("Windows.Foundation").as_str(), "Windows::Foundation::");

        assert_eq!(Gen::relative("Windows").namespace("Windows.Foundation").as_str(), "Foundation::");

        assert_eq!(Gen::relative("Windows.Foundation").namespace("Windows.Foundation").as_str(), "");

        assert_eq!(Gen::relative("Windows.Foundation.Collections").namespace("Windows.Foundation").as_str(), "super::");
    }

    #[test]
    fn test_features() {
        let mut features = BTreeSet::new();
        features.insert("Windows.Foundation");
        assert_eq!(Gen { root: "Microsoft", relative: "", ignore_windows_features: false, docs: false, build: false }.gen_cfg(&features).as_str(), r#"#[cfg(feature = "Windows_Foundation")]"#);
        assert_eq!(Gen { root: "Microsoft", relative: "Microsoft.UI.Composition.Diagnostics", ignore_windows_features: false, docs: true, build: false }.gen_cfg_doc(&features).as_str(), r#"#[doc = "*Required features: `UI_Composition_Diagnostics`, `Windows_Foundation`*"]"#);

        let mut features = BTreeSet::new();
        features.insert("Microsoft.Foundation");
        assert_eq!(Gen { root: "Microsoft", relative: "", ignore_windows_features: false, docs: false, build: false }.gen_cfg(&features).as_str(), r#"#[cfg(feature = "Foundation")]"#);
        assert_eq!(Gen { root: "Microsoft", relative: "Microsoft.UI.Composition.Diagnostics", ignore_windows_features: false, docs: true, build: false }.gen_cfg_doc(&features).as_str(), r#"#[doc = "*Required features: `UI_Composition_Diagnostics`, `Foundation`*"]"#);
    }
}
