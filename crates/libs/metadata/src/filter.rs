#[derive(Default)]
pub struct Filter(pub Vec<(String, bool)>);

impl Filter {
    pub fn new(include: &[&str], exclude: &[&str]) -> Self {
        let mut rules = vec![];

        for include in include {
            rules.push((include.to_string(), true));
        }

        for exclude in exclude {
            rules.push((exclude.to_string(), false));
        }

        rules.sort_unstable_by(|left, right| {
            let left = (left.0.len(), !left.1);
            let right = (right.0.len(), !right.1);
            left.cmp(&right).reverse()
        });

        Self(rules)
    }

    pub fn includes_namespace(&self, namespace: &str) -> bool {
        if self.0.is_empty() {
            return true;
        }

        for rule in &self.0 {
            if rule.1 {
                // include
                if rule.0.starts_with(namespace) {
                    return true;
                }
                if namespace.starts_with(&rule.0) {
                    return true;
                }
            } else {
                // exclude
                if namespace.starts_with(&rule.0) {
                    return false;
                }
            }
        }

        false
    }

    pub fn includes_type_name(&self, namespace: &str, name: &str) -> bool {
        if self.0.is_empty() {
            return true;
        }

        for rule in &self.0 {
            if match_type_name(&rule.0, namespace, name) {
                return rule.1;
            }
        }

        false
    }
}

fn match_type_name(rule: &str, namespace: &str, name: &str) -> bool {
    if rule.len() <= namespace.len() {
        return namespace.starts_with(rule);
    }

    if !rule.starts_with(namespace) {
        return false;
    }

    if rule.as_bytes()[namespace.len()] != b'.' {
        return false;
    }

    name == &rule[namespace.len() + 1..]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn includes_type_name(filter: &Filter, full_name: &'static str) -> bool {
        let type_name = crate::TypeName::parse(full_name);
        filter.includes_type_name(type_name.namespace, type_name.name)
    }

    #[test]
    fn test_namespace() {
        let include = ["N1.N2"];
        let exclude = ["N1.N2.N3"];
        let f = Filter::new(&include, &exclude);

        assert!(f.includes_namespace("N1"));
        assert!(f.includes_namespace("N1.N2"));
        assert!(f.includes_namespace("N1.N2.N4"));

        assert!(!f.includes_namespace("N1.N2.N3"));
        assert!(!f.includes_namespace("N1.N2.N3.N4"));
    }

    #[test]
    fn test_simple() {
        let include = ["N1", "N3", "N3.N4.N5"];
        let exclude = ["N2", "N3.N4"];
        let f = Filter::new(&include, &exclude);

        assert!(!includes_type_name(&f, "NN.T"));

        assert!(includes_type_name(&f, "N1.T"));
        assert!(includes_type_name(&f, "N3.T"));

        assert!(!includes_type_name(&f, "N2.T"));
        assert!(!includes_type_name(&f, "N3.N4.T"));

        assert!(includes_type_name(&f, "N3.N4.N5.T"));
    }

    #[test]
    fn filter_excludes_same_length() {
        let include = ["N.N1", "N.N2"];
        let exclude = ["N.N3", "N.N4"];
        let f = Filter::new(&include, &exclude);

        assert!(includes_type_name(&f, "N.N1.T"));
        assert!(includes_type_name(&f, "N.N2.T"));

        assert!(!includes_type_name(&f, "N.N3.T"));
        assert!(!includes_type_name(&f, "N.N4.T"));
    }

    #[test]
    fn filter_exclude_include_precedence() {
        let include = ["N.T"];
        let exclude = ["N.T"];
        let f = Filter::new(&include, &exclude);

        assert!(!includes_type_name(&f, "N.T"));
    }
}
