use super::*;

/// Returns the overload-disambiguated Rust name produced by an
/// `[overload("…")]` attribute on a non-`SpecialName` method, if any.
///
/// This is the same value `--filter` should accept for individually
/// addressing a renamed overload (e.g. `InsertKeyFrameWithEasingFunction`
/// rather than the shared metadata name `InsertKeyFrame`). `SpecialName`
/// methods (property/event accessors) and "generated" overload suffixes
/// like `InsertKeyFrame2` are skipped — those are handled by the existing
/// raw-name + sugar paths in `filter.rs`.
pub fn method_overload_name(row: MethodDef) -> Option<String> {
    if row.flags().contains(MethodAttributes::SpecialName) {
        return None;
    }
    let attribute = row.find_attribute("OverloadAttribute")?;
    let name = row.name();
    for (_, arg) in attribute.value() {
        if let Value::Utf8(overload) = arg {
            if let Some(suffix) = overload.strip_prefix(name) {
                if suffix.parse::<u32>().is_ok() {
                    // Generated `Name2`/`Name3` overload — not a real
                    // rename, so no disambiguated handle to filter on.
                    return None;
                }
            }
            if overload == name {
                return None;
            }
            return Some(overload);
        }
    }
    None
}

#[derive(Default)]
pub struct MethodNames {
    map: HashMap<String, u32>,
    raw: bool,
}

impl MethodNames {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            raw: false,
        }
    }

    pub fn new_raw() -> Self {
        Self {
            map: HashMap::new(),
            raw: true,
        }
    }

    /// Creates a MethodNames that uses raw metadata names when `minimal` is true.
    pub(crate) fn for_style(style: &Style) -> Self {
        if style.has_com() {
            Self::new_raw()
        } else {
            Self::new()
        }
    }

    pub fn add(&mut self, method: MethodDef) -> TokenStream {
        let name = if self.raw && method.flags().contains(MethodAttributes::SpecialName) {
            // In raw/minimal mode, keep the raw metadata name for property/event accessors
            method.name().to_string()
        } else {
            method_def_special_name(method)
        };
        let overload = self.map.entry(name.clone()).or_insert(0);
        *overload += 1;
        if *overload > 1 {
            format!("{name}{overload}").parse().unwrap()
        } else {
            to_ident(&name)
        }
    }
}

fn method_def_special_name(row: MethodDef) -> String {
    let name = row.name();
    if row.flags().contains(MethodAttributes::SpecialName) {
        if name.starts_with("get") {
            name[4..].to_string()
        } else if name.starts_with("put") {
            format!("Set{}", &name[4..])
        } else if name.starts_with("add") {
            name[4..].to_string()
        } else if name.starts_with("remove") {
            format!("Remove{}", &name[7..])
        } else {
            name.to_string()
        }
    } else {
        if let Some(attribute) = row.find_attribute("OverloadAttribute") {
            for (_, arg) in attribute.value() {
                if let Value::Utf8(overload) = arg {
                    // Detect generated overload and revert back to original name.
                    if let Some(suffix) = overload.strip_prefix(name) {
                        if suffix.parse::<u32>().is_ok() {
                            return name.to_string();
                        }
                    }

                    return overload;
                }
            }
        }
        name.to_string()
    }
}
