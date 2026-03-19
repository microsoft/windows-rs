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
