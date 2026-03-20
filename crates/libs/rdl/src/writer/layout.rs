use super::*;

#[derive(Default)]
pub struct Layout {
    modules: BTreeMap<String, Layout>,
    winrt: BTreeMap<String, Vec<String>>,
    win32: BTreeMap<String, Vec<String>>,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            modules: BTreeMap::new(),
            winrt: BTreeMap::new(),
            win32: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, namespace: &str, name: &str, winrt: bool, tokens: String) {
        if let Some((first, rest)) = namespace.split_once('.') {
            self.modules
                .entry(first.to_string())
                .or_default()
                .insert(rest, name, winrt, tokens)
        } else if winrt {
            self.modules
                .entry(namespace.to_string())
                .or_default()
                .winrt
                .entry(name.to_string())
                .or_default()
                .push(tokens);
        } else {
            self.modules
                .entry(namespace.to_string())
                .or_default()
                .win32
                .entry(name.to_string())
                .or_default()
                .push(tokens);
        }
    }

    fn to_module(&self, name: &str) -> String {
        let mut output = String::new();

        if !self.modules.is_empty() {
            output.push_str("mod ");
            output.push_str(name);
            output.push('{');

            for (name, module) in &self.modules {
                output.push_str(&module.to_module(name));
            }

            output.push('}')
        }

        if !self.winrt.is_empty() {
            output.push_str("#[winrt] mod ");
            output.push_str(name);
            output.push('{');

            for items in self.winrt.values() {
                let mut items = items.clone();
                items.sort();
                items.dedup();
                for tokens in &items {
                    output.push_str(tokens);
                }
            }

            output.push('}')
        }

        if !self.win32.is_empty() {
            output.push_str("#[win32] mod ");
            output.push_str(name);
            output.push('{');

            for items in self.win32.values() {
                let mut items = items.clone();
                items.sort();
                items.dedup();
                for tokens in &items {
                    output.push_str(tokens);
                }
            }

            output.push('}')
        }

        output
    }
}

impl std::fmt::Display for Layout {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (name, module) in &self.modules {
            write!(fmt, "{}", &module.to_module(name))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// When multiple arch variants of the same type produce identical flat nested-type
    /// definitions (e.g. `SLIST_HEADER_0` for x64 and arm64), the layout must emit
    /// only one copy rather than a separate copy per variant.
    #[test]
    fn dedup_identical_items() {
        let mut layout = Layout::new();

        // Simulate two arch variants both contributing an identical "Outer_0" flat type.
        let tokens = "struct Outer_0{value:u32,}";
        layout.insert("Test", "Outer_0", false, tokens.to_string());
        layout.insert("Test", "Outer_0", false, tokens.to_string());

        let output = layout.to_string();

        let count = output.matches("struct Outer_0{value:u32,}").count();
        assert_eq!(
            count, 1,
            "identical flat-type definitions must be deduplicated"
        );
    }

    /// When two arch variants contribute *different* definitions for the same flat
    /// name (e.g. x86 vs x64 `SLIST_HEADER_0`), both definitions are kept so that
    /// the multi-arch metadata remains intact.
    #[test]
    fn keep_distinct_items() {
        let mut layout = Layout::new();

        layout.insert(
            "Test",
            "Outer_0",
            false,
            "struct Outer_0{a:u32,}".to_string(),
        );
        layout.insert(
            "Test",
            "Outer_0",
            false,
            "struct Outer_0{b:u16,c:u16,}".to_string(),
        );

        let output = layout.to_string();

        assert!(
            output.contains("struct Outer_0{a:u32,}"),
            "first definition must be kept"
        );
        assert!(
            output.contains("struct Outer_0{b:u16,c:u16,}"),
            "second distinct definition must also be kept"
        );
    }
}
