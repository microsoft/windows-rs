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

    /// Returns true if this layout and all of its nested modules contain only winrt items.
    fn is_all_winrt(&self) -> bool {
        self.win32.is_empty() && self.modules.values().all(|m| m.is_all_winrt())
    }

    /// Returns true if this layout and all of its nested modules contain only win32 items.
    fn is_all_win32(&self) -> bool {
        self.winrt.is_empty() && self.modules.values().all(|m| m.is_all_win32())
    }

    /// Render this module as a plain `mod` block, without a kind attribute, for use inside an
    /// already-annotated parent (`#[winrt]` or `#[win32]`).
    fn to_module_plain(&self, name: &str) -> String {
        let has_content =
            !self.modules.is_empty() || !self.winrt.is_empty() || !self.win32.is_empty();

        if !has_content {
            return String::new();
        }

        let mut output = String::new();
        output.push_str("mod ");
        output.push_str(name);
        output.push('{');

        for (name, module) in &self.modules {
            output.push_str(&module.to_module_plain(name));
        }

        for items in self.winrt.values().chain(self.win32.values()) {
            let mut items = items.clone();
            items.sort();
            for tokens in &items {
                output.push_str(tokens);
            }
        }

        output.push('}');
        output
    }

    fn to_module(&self, name: &str) -> String {
        let mut output = String::new();

        let has_modules = !self.modules.is_empty();
        let has_winrt = !self.winrt.is_empty();
        let has_win32 = !self.win32.is_empty();

        // When all content in this module (direct items and all nested sub-modules) is winrt-only,
        // emit a single `#[winrt] mod` block rather than separate `mod` and `#[winrt] mod` blocks.
        // This applies whether the winrt items are direct children or live exclusively in nested mods.
        if !has_win32 && has_modules && self.modules.values().all(|m| m.is_all_winrt()) {
            output.push_str("#[winrt] mod ");
            output.push_str(name);
            output.push('{');

            for (name, module) in &self.modules {
                output.push_str(&module.to_module_plain(name));
            }

            for items in self.winrt.values() {
                let mut items = items.clone();
                items.sort();
                for tokens in &items {
                    output.push_str(tokens);
                }
            }

            output.push('}');
            return output;
        }

        // Symmetric case for win32.
        if !has_winrt && has_modules && self.modules.values().all(|m| m.is_all_win32()) {
            output.push_str("#[win32] mod ");
            output.push_str(name);
            output.push('{');

            for (name, module) in &self.modules {
                output.push_str(&module.to_module_plain(name));
            }

            for items in self.win32.values() {
                let mut items = items.clone();
                items.sort();
                for tokens in &items {
                    output.push_str(tokens);
                }
            }

            output.push('}');
            return output;
        }

        if has_modules {
            output.push_str("mod ");
            output.push_str(name);
            output.push('{');

            for (name, module) in &self.modules {
                output.push_str(&module.to_module(name));
            }

            output.push('}')
        }

        if has_winrt {
            output.push_str("#[winrt] mod ");
            output.push_str(name);
            output.push('{');

            for items in self.winrt.values() {
                let mut items = items.clone();
                items.sort();
                for tokens in &items {
                    output.push_str(tokens);
                }
            }

            output.push('}')
        }

        if has_win32 {
            output.push_str("#[win32] mod ");
            output.push_str(name);
            output.push('{');

            for items in self.win32.values() {
                let mut items = items.clone();
                items.sort();
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
