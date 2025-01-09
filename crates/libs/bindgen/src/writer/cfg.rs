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

impl Writer {
    pub fn write_cfg<R: HasAttributes>(
        &self,
        row: R,
        namespace: &str,
        dependencies: &TypeMap,
        not: bool,
    ) -> TokenStream {
        let mut features = BTreeSet::new();

        if row.has_attribute("DeprecatedAttribute") {
            features.insert("deprecated".to_string());
        }

        let mut compact: Vec<&'static str> = dependencies.keys().map(|tn| tn.namespace()).collect();
        compact.sort();
        compact.dedup();

        for pos in 0..compact.len() {
            match (compact.get(pos), compact.get(pos + 1)) {
                (Some(first), Some(second)) if namespace_starts_with(second, first) => {
                    compact.remove(pos);
                }
                (_, None) => break,
                _ => continue,
            }
        }

        for dependency in compact {
            if dependency.is_empty()
                || namespace_starts_with(namespace, dependency)
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
