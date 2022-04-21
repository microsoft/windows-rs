use super::*;

pub struct Gen<'a> {
    pub reader: &'a Reader<'a>,
    pub namespace: &'a str,
    pub sys: bool,
    pub flatten: bool,
    pub cfg: bool,
    pub doc: bool,
    pub min_enum: bool,
    pub min_inherit: bool,
    pub min_xaml: bool,
    pub windows_extern: bool,
    pub component: bool,
}

impl<'a> Gen<'a> {
    pub fn new(reader: &'a Reader) -> Self {
        Self {
            reader,
            namespace:"",
            sys: false,
            flatten: false,
            cfg:false,
            doc:false,
            min_enum:false,
            min_inherit:false,
            min_xaml: false,
            windows_extern:false,
            component:false,
        }
    }
    pub fn namespace(&self, namespace: &str) -> TokenStream {
        if self.flatten || namespace == self.namespace {
            quote! {}
        } else {
            let is_external = namespace.starts_with("Windows.") && !self.namespace.starts_with("Windows");
            let mut relative = self.namespace.split('.').peekable();
            let mut namespace = namespace.split('.').peekable();

            while relative.peek() == namespace.peek() {
                if relative.next().is_none() {
                    break;
                }

                namespace.next();
            }

            let mut tokens = TokenStream::new();

            if is_external {
                tokens.push_str("::windows::");
                namespace.next();
            } else {
                for _ in 0..relative.count() {
                    tokens.push_str("super::");
                }
            }

            for namespace in namespace {
                tokens.push_str(namespace);
                tokens.push_str("::");
            }

            tokens
        }
    }

    pub fn doc(&self, cfg: &Cfg) -> TokenStream {
        if !self.doc {
            quote! {}
        } else {
            let mut tokens = format!(r#"`\"{}\"`"#, to_feature(self.namespace));

            let mut features = cfg_features(cfg, self.namespace);
            if self.windows_extern {
                features = features.into_iter().filter(|f| !f.starts_with("Windows.")).collect();
            }
            for features in features {
                tokens.push_str(&format!(r#", `\"{}\"`"#, to_feature(features)));
            }

            format!(r#"#[doc = "*Required features: {}*"]"#, tokens).into()
        }
    }

    pub fn cfg(&self, cfg: &Cfg) -> TokenStream {
        if !self.cfg {
            quote! {}
        } else {
            let arches = &cfg.arches;
            let arch = match arches.len() {
                0 => quote! {},
                1 => {
                    quote! { #[cfg(#(target_arch = #arches),*)] }
                }
                _ => {
                    quote! { #[cfg(any(#(target_arch = #arches),*))] }
                }
            };

            let mut features = cfg_features(cfg, self.namespace);
            if self.windows_extern {
                features = features.into_iter().filter(|f| !f.starts_with("Windows.")).collect();
            }

            let features = match features.len() {
                0 => quote! {},
                1 => {
                    let features = features.iter().cloned().map(to_feature);
                    quote! { #[cfg(#(feature = #features)*)] }
                }
                _ => {
                    let features = features.iter().cloned().map(to_feature);
                    quote! { #[cfg(all( #(feature = #features),* ))] }
                }
            };

            quote! { #arch #features }
        }
    }

    pub fn not_cfg(&self, cfg: &Cfg) -> TokenStream {
        let mut features = cfg_features(cfg, self.namespace);
        if !self.cfg || features.is_empty() {
            quote! {}
        } else {
            if self.windows_extern {
                features = features.into_iter().filter(|f| !f.starts_with("Windows.")).collect();
            }
            match features.len() {
                0 => quote! {},
                1 => {
                    let features = features.iter().cloned().map(to_feature);
                    quote! { #[cfg(not(#(feature = #features)*))] }
                }
                _ => {
                    let features = features.iter().cloned().map(to_feature);
                    quote! { #[cfg(not(all( #(feature = #features),* )))] }
                }
            }
        }
    }
}

fn to_feature(name: &str) -> String {
    let mut feature = String::new();

    for name in name.split('.').skip(1) {
        feature.push_str(name);
        feature.push('_');
    }

    if feature.is_empty() {
        feature = name.to_string();
    } else {
        feature.truncate(feature.len() - 1);
    }

    feature
}

pub fn cfg_features<'a>(cfg:&'a Cfg, namespace: &'a str) -> Vec<&'a str> {
    let mut compact = Vec::<&'static str>::new();
    for feature in cfg.types.keys() {
        if !feature.is_empty() && !starts_with(namespace, feature) {
            for pos in 0..compact.len() {
                if starts_with(feature, unsafe { compact.get_unchecked(pos) }) {
                    compact.remove(pos);
                    break;
                }
            }
            compact.push(feature);
        }
    }
    compact
}

fn starts_with(namespace: &str, feature: &str) -> bool {
    if namespace == feature {
        return true;
    }

    if namespace.len() > feature.len() && namespace.as_bytes().get(feature.len()) == Some(&b'.') {
        return namespace.starts_with(feature);
    }

    false
}
