use super::*;

pub fn write_arches<R: HasAttributes>(row: R) -> TokenStream {
    let mut tokens = quote! {};

    if let Some(attribute) = row.find_attribute("SupportedArchitectureAttribute") {
        if let Some((_, Value::I32(value))) = attribute.args().first() {
            let mut arches = BTreeSet::new();

            if value & 1 == 1 {
                arches.insert("x86");
            }

            if value & 2 == 2 {
                arches.insert("x86_64");
                arches.insert("arm64ec");
            }

            if value & 4 == 4 {
                arches.insert("aarch64");
            }

            match arches.len() {
                0 => {}
                1 => tokens.combine(quote! { #[cfg(#(target_arch = #arches),*)] }),
                _ => tokens.combine(quote! { #[cfg(any(#(target_arch = #arches),*))] }),
            }
        }
    }

    tokens
}

#[derive(Default)]
pub struct Cfg {
    features: Vec<&'static str>,
    deprecated: bool,
}

impl Cfg {
    pub fn new<R: HasAttributes>(row: R, dependencies: &TypeMap) -> Self {
        let mut features: Vec<&'static str> =
            dependencies.keys().map(|tn| tn.namespace()).collect();
        features.sort();
        features.dedup();

        Self {
            features,
            deprecated: row.has_attribute("DeprecatedAttribute"),
        }
    }

    pub fn difference<R: HasAttributes>(&self, row: R, dependencies: &TypeMap) -> Self {
        let mut difference = Self::new(row, dependencies);

        for feature in &self.features {
            if let Ok(index) = difference.features.binary_search(feature) {
                difference.features.remove(index);
            }
        }

        difference.deprecated = !self.deprecated && difference.deprecated;
        difference
    }

    pub fn write(&self, writer: &Writer, not: bool) -> TokenStream {
        if !writer.config.package {
            return quote! {};
        }

        let mut compact = self.features.clone();

        for pos in 0..compact.len() {
            match (compact.get(pos), compact.get(pos + 1)) {
                (Some(first), Some(second)) if namespace_starts_with(second, first) => {
                    compact.remove(pos);
                }
                (_, None) => break,
                _ => continue,
            }
        }

        let mut features = BTreeSet::new();

        if self.deprecated {
            features.insert("deprecated".to_string());
        }

        for dependency in compact {
            if dependency.is_empty()
                || namespace_starts_with(writer.namespace, dependency)
                || dependency == "Windows.Foundation"
                || dependency == "Windows.Win32.Foundation"
            {
                continue;
            }

            let mut feature = String::new();

            for name in dependency.split('.').skip(1) {
                feature.push_str(name);
                feature.push('_');
            }

            feature.truncate(feature.len() - 1);
            features.insert(feature);
        }

        let mut tokens = quote! {};

        match features.len() {
            0 => {}
            1 => {
                if not {
                    tokens.combine(quote! { #[cfg(not(#(feature = #features)*))] });
                } else {
                    tokens.combine(quote! { #[cfg(#(feature = #features)*)] });
                }
            }
            _ => {
                if not {
                    tokens.combine(quote! { #[cfg(not(all( #(feature = #features),* )))] });
                } else {
                    tokens.combine(quote! { #[cfg(all( #(feature = #features),* ))] });
                }
            }
        }

        tokens
    }
}
