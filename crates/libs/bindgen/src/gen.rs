use super::*;

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
            let is_windows_extern = namespace.starts_with("Windows.") && !self.namespace.starts_with("Windows");
            let mut relative = self.namespace.split('.').peekable();
            let mut namespace = namespace.split('.').peekable();

            while relative.peek() == namespace.peek() {
                if relative.next().is_none() {
                    break;
                }

                namespace.next();
            }

            let mut tokens = TokenStream::new();

            if is_windows_extern {
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

    pub(crate) fn doc(&self, cfg: &Cfg) -> TokenStream {
        if !self.doc {
            quote! {}
        } else {
            let mut tokens = format!("'{}'", to_feature(self.namespace));

            for features in &cfg.features(self.namespace) {
                tokens.push_str(&format!(", '{}'", to_feature(features)));
            }

            format!(r#"#[doc = "*Required features: {}*"]"#, tokens).into()
        }
    }

    pub(crate) fn cfg(&self, cfg: &Cfg) -> TokenStream {
        if !self.cfg {
            quote! {}
        } else {
            let arches = cfg.arches();
            let arch = match arches.len() {
                0 => quote! {},
                1 => {
                    quote! { #[cfg(#(target_arch = #arches),*)] }
                }
                _ => {
                    quote! { #[cfg(any(#(target_arch = #arches),*))] }
                }
            };

            let features = &cfg.features(self.namespace);
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

    pub(crate) fn not_cfg(&self, cfg: &Cfg) -> TokenStream {
        let features = &cfg.features(self.namespace);
        if !self.cfg || features.is_empty() {
            quote! {}
        } else {
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
