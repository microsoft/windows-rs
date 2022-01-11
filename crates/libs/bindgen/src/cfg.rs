use super::*;

#[derive(Default, Clone)]
pub struct Cfg {
    pub arch: BTreeSet<&'static str>,
    pub features: BTreeSet<&'static str>,
}

impl Cfg {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn union(&self, other: Self) -> Self {
        Self { arch: self.arch.union(&other.arch).cloned().collect(), features: self.features.union(&other.features).cloned().collect() }
    }

    pub fn and_iterator(&self) -> Self {
        let mut combo = self.clone();
        combo.features.insert("Windows.Foundation.Collections");
        combo
    }

    pub fn and_async(&self) -> Self {
        let mut combo = self.clone();
        combo.features.insert("Windows.Foundation");
        combo
    }

    pub fn gen_with_doc(&self, gen: &Gen) -> TokenStream {
        let doc = self.gen_doc(gen);
        let requires = self.gen(gen);
        quote! { #doc #requires }
    }

    pub fn gen_doc(&self, gen: &Gen) -> TokenStream {
        if !gen.doc {
            quote! {}
        } else {
            let mut tokens = format!("'{}'", to_feature(gen.namespace));

            for features in &self.features {
                tokens.push_str(&format!(", '{}'", to_feature(features)));
            }

            format!(r#"#[doc = "*Required features: {}*"]"#, tokens).into()
        }
    }

    pub fn gen(&self, gen: &Gen) -> TokenStream {
        if !gen.cfg {
            quote! {}
        } else {
            let arch = match self.arch.len() {
                0 => quote! {},
                1 => {
                    let arch = &self.arch;
                    quote! { #[cfg(#(target_arch = #arch),*)] }
                }
                _ => {
                    let arch = &self.arch;
                    quote! { #[cfg(any(#(target_arch = #arch),*))] }
                }
            };

            let features = match self.features.len() {
                0 => quote! {},
                1 => {
                    let features = self.features.iter().cloned().map(to_feature);
                    quote! { #[cfg(#(feature = #features)*)] }
                }
                _ => {
                    let features = self.features.iter().cloned().map(to_feature);
                    quote! { #[cfg(all( #(feature = #features),* ))] }
                }
            };

            quote! { #arch #features }
        }
    }

    pub fn gen_not(&self, gen: &Gen) -> TokenStream {
        if !gen.cfg || self.features.is_empty() {
            quote! {}
        } else {
            match self.features.len() {
                0 => quote! {},
                1 => {
                    let features = self.features.iter().cloned().map(to_feature);
                    quote! { #[cfg(not(#(feature = #features)*))] }
                }
                _ => {
                    let features = self.features.iter().cloned().map(to_feature);
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
