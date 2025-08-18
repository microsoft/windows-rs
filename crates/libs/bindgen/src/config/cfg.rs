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
    features: BTreeSet<&'static str>,
}

impl Cfg {
    pub fn new(dependencies: &TypeMap, config: &Config) -> Self {
        let features: BTreeSet<&'static str> = dependencies
            .keys()
            .filter_map(|tn| {
                if config.types.contains_key(tn) {
                    Some(tn.namespace())
                } else {
                    None
                }
            })
            .collect();

        Self { features }
    }

    pub fn difference(&self, dependencies: &TypeMap, config: &Config) -> Self {
        let mut difference = Self::new(dependencies, config);

        for feature in &self.features {
            difference.features.remove(feature);
        }

        difference
    }

    pub fn write(&self, config: &Config, not: bool) -> TokenStream {
        if !config.package {
            return quote! {};
        }

        let mut compact = BTreeSet::<&'static str>::new();

        for feature in self.features.iter().rev() {
            let mut keep = true;

            for compact in &compact {
                if namespace_starts_with(compact, feature) {
                    keep = false;
                    break;
                }
            }

            if keep {
                compact.insert(feature);
            }
        }

        let mut features = BTreeSet::new();

        for dependency in compact {
            if dependency.is_empty()
                || namespace_starts_with(config.namespace, dependency)
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
